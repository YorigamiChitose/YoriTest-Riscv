# ===============================================================
#  路径与工具变量
# ===============================================================

TOP_DIR    := $(CURDIR)
BUILD_DIR  := $(TOP_DIR)/build

# Rust 目标三元组（不含 C 扩展）与镜像工具链
TARGET     := riscv32im-unknown-none-elf
TARGET_DIR := $(TOP_DIR)/target/$(TARGET)/release
OBJCOPY    := riscv64-unknown-elf-objcopy

# YoriCore-Rev 仿真目录（difftest 时委派过去）
YCORE_DIR ?= $(TOP_DIR)/../YoriCore-Rev

# 仿真器二进制（可用环境变量 SIM_BIN 覆盖）
SIM_BIN ?= $(YCORE_DIR)/build/verilator/VSimTop

# 测试预期结果清单（实际 vs 预期对比判定 PASS）
EXPECT_FILE := $(TOP_DIR)/expected.list

# run 目标：默认运行全部 bin，ALL=<bin 名> 时只运行指定的（可空格分隔多个）
ALL ?=
