TOP_DIR    := $(CURDIR)
BUILD_DIR  := $(TOP_DIR)/build

TARGET     := riscv32imac-unknown-none-elf
TARGET_DIR := $(TOP_DIR)/target/$(TARGET)/release
OBJCOPY    := riscv64-unknown-elf-objcopy

YCORE_DIR ?= $(TOP_DIR)/../YoriCore-Rev

SIM_BIN ?= $(YCORE_DIR)/build/verilator/VSimTop

EXPECT_FILE := $(TOP_DIR)/expected.list

ALL ?=
