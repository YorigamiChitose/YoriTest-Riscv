/* YoriTest-Riscv 链接脚本
 * 与 YoriCore-Rev 对齐：镜像加载到 0x8000_0000，pmem 大小 16MB。
 * 入口 _start 置于 .text 最前，保证复位向量处即启动代码。
 */
OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    ram (rwx) : ORIGIN = 0x80000000, LENGTH = 0x1000000
}

SECTIONS
{
    . = ORIGIN(ram);

    .text : {
        *(.text._start)
        *(.text .text.*)
    } > ram

    . = ALIGN(4);

    .rodata : {
        *(.srodata .srodata.*)
        *(.rodata .rodata.*)
    } > ram

    . = ALIGN(4);

    .data : {
        *(.sdata .sdata.*)
        *(.data .data.*)
    } > ram

    . = ALIGN(4);

    .bss : {
        _bss_start = .;
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        . = ALIGN(4);
        _bss_end = .;
    } > ram

    /* 栈顶：内存最高处，栈向下增长 */
    _stack_top = ORIGIN(ram) + LENGTH(ram);
}
