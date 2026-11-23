# YoriTest-Riscv 测试框架方案

为 RISC-V 处理器 **YoriCore-Rev**（RV32IMA + Zicsr + Zifencei + RV32C）提供的一整套测试程序框架。
采用 Rust（`no_std` 裸机）+ Makefile 构建，仓库间按「独立 + 产物拷贝」协作。

---

## 1. 定位与目标

YoriCore-Rev 当前是 NEMU 式 C++ monitor + Spike 逐指令 difftest，仓库内不保存测试程序。
本框架作为**外部测试程序来源**，产出可加载的裸机镜像（`.bin`），用于：

- 验证核的指令实现正确性（cpu-test）
- 验证抽象机（AM）库的正确性与高层功能（am-test）
- 远期支撑操作系统运行（os，待核演进）

---

## 2. 项目结构（cargo workspace）

整个项目是一个 cargo workspace，`am`、`cpu-test`、`am-test`、`os` 四个 crate 在文件层级**平级**（`os` 远期），
由工作区根 `Cargo.toml` 统一管理。`am` 虽为基座，但只是普通成员 crate，并非其他 crate 的上级目录。

| crate | 角色 | 状态 |
|-------|------|------|
| `am/` | 通用库（抽象机 + 构建基座），唯一对接硬件的层（lib crate） | 当前 |
| `cpu-test/` | 无系统下的纯基础指令测试，含 RV32C 专项，依赖 `am`（bin crate，多 `[[bin]]`） | 当前 |
| `am-test/` | 无系统下的 AM 高层测试，依赖 `am`（bin crate） | 当前 |
| `os/` | 操作系统（OS 本体 + OS 上的应用） | 远期 |

> `os/` 依赖核先支持 S/U 特权级、MMU、中断。当前核只有 M 态，故暂缓立项。

---

## 3. 分层架构

```
硬件（YoriCore-Rev：halt@0xF0000000、AXI、串口）
   └─ am（唯一对接硬件的层：halt/串口/内存 + 启动 + 链接脚本 + rlib）
        ├─ cpu-test（通过 am 的 halt 退出，测指令语义）
        ├─ am-test（测 am 正确性 + 高层功能）
        └─ os（远期）
```

**核心原则**：所有硬件交互（halt、串口、内存映射、设备地址）只收敛在 `am`，
上层仓库不直接写硬件地址，只调用 `am` 的 API。

---

## 4. 硬件契约（与 YoriCore-Rev 对接）

这是四个仓库共同遵守的单一事实来源：

- **镜像格式**：裸二进制 `.bin`（由 `.elf` 经 `objcopy` 转换），加载到 `0x8000_0000`。
- **halt 退出**：向 `0xF0000000` 写 1 字节，`tohost` 编码：
  - `1` → PASS
  - `(code << 1) | 1` → FAIL（`code` 为失败码，`0` 表示超时等约定）
- **串口（ns16550 子集）**：基址 `0x10000000`。写 `THR`（`+0`）发送一个字符，
  读 `LSR`（`+5`）的 `0x20`（THR 空）位轮询等待。`am::putchar` 内部已封装该轮询。
- **内存区间（可缓存 DRAM）**：`0x8000_0000` 起 16MB，RTL `Config.Mem`（Base+Size）与仿真
  `CONFIG_MBASE`/`CONFIG_MSIZE` 对齐；落在该区间才进 L1 缓存。
- **MMIO 区间标准化**：RTL（`Config.Mmio.Devices`）与仿真（`device.cpp` 设备表）
  统一以「起始地址 + 大小」描述不可缓存设备区间，与普通访存区分。
  - `0x0200b000`（4KB，RTC，只读 64 位微秒）
  - `0x10000000`（4KB，串口）
  - `0xF0000000`（256MB，halt 及其它 MMIO，覆盖到 `0xFFFFFFFF`）
- **difftest**：所有测试镜像应可被 Spike 逐指令对比（与 YoriCore-Rev 的 difftest 对齐）。
- **复位向量**：`0x8000_0000`（`Config.Addr.Base`）。

---

## 5. Rust 裸机技术底座（收敛在 am）

Rust 写裸机最大的坑集中在启动与运行环境，全部收敛到 `am`，避免各仓库重复实现：

1. `#![no_std]` + `#![no_main]`
2. `#[panic_handler]`：panic 直接写 halt 设备以 FAIL 退出（不依赖默认 `unreachable`）
3. 链接脚本 `.lds`：定义入口 `_start`、`.text/.rodata/.data/.bss` 布局、栈顶、bss 清零
4. 启动 `_start`：清 bss → 设置栈指针 → 调用 `main`
5. 全局 allocator（`#[global_allocator]`，供需要堆的 am-test 使用）
6. 目标 triple：默认 `riscv32imac-unknown-none-elf`（默认启用 C）；所有 crate、所有指令测试统一使用该目标，不再区分 C / 非 C

> `printf!`/`println!` 基于 `core::fmt`，需由 `am` 提供底层 `putchar` 写串口实现。

---

## 6. AM 通用库职责与 API

参照 ysyx 的 Abstract Machine（AM），Rust 版抽象如下：

**构建基座角色**：链接脚本、启动代码、`#[panic_handler]`、allocator、目标 triple 配置、
通用 `mkscript/` 片段、riscv 寄存器/CSR 访问封装。

**TRM API（最小运行时）**：

| API | 语义 |
|-----|------|
| `am::halt(code: u32) -> !` | 写 halt 设备并停机 |
| `am::putchar(c: u8)` | 写串口单字符 |
| `am::print!` / `am::println!` | 格式化输出（基于 `core::fmt`） |
| 堆分配（可选） | 全局 allocator |

**最小可信指令子集（硬约束）**：

`am` 的启动路径（`_start`、清 bss）与 halt 路径**只能使用最基础的指令**——
`lui` / `addi` / `lw` / `sw` / `jal` / `jalr`（必要时 `beq`/`bne`）。
不得用到乘除、CSR 操作、复杂分支等 cpu-test 正在验证的指令，避免「测指令却依赖该指令」的循环依赖。
这与 riscv-tests 的 tohost 最小启动约定一致。

---

## 7. cpu-test 与 am-test 分工

- **cpu-test**：逐条验证 RV32IMA + Zicsr + Zifencei + RV32C 各指令的语义与边界
  （算术/逻辑/移位/分支/跳转/load-store/乘除/CSR/系统指令，含符号扩展、字节使能、除零、溢出等边界）。
  每条测试独立成一个 `.bin`，通过 `am::halt` 退出。
- **cpu-test 中的 RV32C 专项**：用 `.option rvc` 在汇编里显式嵌入 16 位指令，
  覆盖 C.ADDI/ADDI4SPN/ADDI16SP/LI/LUI/ALU/移位/访存/跳转/分支/变长取指；
  每个 bin 的 halt code 是 6 位失败位图。
- **am-test**：验证 `am` 库自身正确性（halt、串口、堆、格式化输出），
  以及用 `am` 写的高层功能（字符串、内存分配等），作为「AM 正确性」的回归。

---

## 8. OS（远期）

- 依赖核先支持：S/U 特权级、页表/MMU、中断与异常（当前核仅有 M 态、4 个 CSR、无 MMU、无中断）。
- 内容：OS 本体（trap、调度、内存管理、驱动）+ OS 上运行的应用。
- 与 `am` 的关系：OS 可能复用 `am` 的设备抽象，或提供更完整的运行时。

---

## 9. 依赖与产物流转

- 方式：**cargo workspace**。工作区根 `Cargo.toml` 声明 `[workspace] members`，
  `cpu-test`/`am-test`/`os` 通过 workspace 内路径依赖 `am`（`am = { path = "../am" }`）。
- 编译由 cargo 负责（默认 `cargo build --release` 使用 `riscv32imac-unknown-none-elf`，所有 crate/测试统一带 C），
  `am` 产出 `libam.rlib`，其余 crate 自动链接，无需手动拷贝产物。
- 最终各 crate 产出 `.bin` 镜像，加载到 YoriCore-Rev（`make run ALL=...` / `make difftest ALL=...`）。

---

## 10. Makefile 约定（批处理脚本角色）

Makefile **不负责编译**（编译交给 cargo），而是作为批处理脚本驱动整个项目的自动化：

- 顶层 `Makefile` 聚合常用目标：`make build`（调 `cargo build`）、
  `make run`（编译 + 逐镜像加载运行 + 汇总 PASS/FAIL）、`make clean` 等。
- 各子项目内部由 cargo 负责编译（`Cargo.toml` 定义依赖与 target），Makefile 只调用 cargo 命令。
- 镜像转换（`.elf` → `.bin`，`objcopy`）与运行/对比（difftest）由 Makefile 编排。
- 约定沿用 YoriCore-Rev 风格：`mkscript/*.mk` 分片段 + `include` 聚合 + `build/` 产物目录。

---

## 11. 目录结构

```
YoriTest-Riscv/
├── Cargo.toml           # workspace 根（members 含 am/cpu-test/am-test）
├── Makefile             # 批处理脚本（驱动 cargo + 运行测试）
├── mkscript/            # 顶层 make 片段（env.mk 等）
├── README.md            # 本方案文档
├── am/                  # 通用库（lib crate）
│   ├── Cargo.toml
│   ├── src/             # lib.rs / halt.rs / uart.rs / ...
│   └── link/            # 链接脚本 .lds
├── cpu-test/            # 指令测试（bin crate，多 [[bin]]）
│   ├── Cargo.toml
│   └── src/             # 每条指令/每类指令一个 bin 源文件（含 c_* 专项）
├── am-test/             # AM 高层测试（bin crate）
│   ├── Cargo.toml
│   └── src/
└── os/                  # 操作系统（远期）
    ├── Cargo.toml
    └── src/
```

---

## 12. 关键设计决策

| 决策 | 结论 | 理由 |
|------|------|------|
| cpu-test 是否依赖 am | 依赖 | 硬件交互收敛到 am，避免重复；am 提供最小可信启动/halt 规避循环依赖 |
| OS 建立时机 | 远期 | 当前核无 S/U + MMU + 中断，OS 无法运行 |
| 项目管理方式 | cargo workspace（4 crate 平级） | 依赖与编译统一由 cargo 管理，Makefile 仅作批处理脚本 |
| 目标 triple | `riscv32imac-unknown-none-elf` | 所有 crate、所有指令测试统一默认带 C，不做 C / 非 C 区分 |
| 构建基座位置 | 收敛进 `am` | 避免仓库碎片化，am 兼任通用库 + 构建基座 |

---

## 13. 实施顺序（建议）

1. **am 仓库骨架**：链接脚本、启动、`#[panic_handler]`、halt/串口 API、Makefile，产出一个可 halt 的最小镜像打通链路。
2. **cpu-test**：先落 2~3 条指令测试跑通「编译 → 链接 am → 加载 → difftest」闭环，再铺开全部指令；RV32C 专项也放在 cpu-test 内。
3. **am-test**：验证 am 的 halt/串口/堆/格式化输出。
4. **os**：待核演进后再立项。
