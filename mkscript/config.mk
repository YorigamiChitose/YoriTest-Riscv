TOP_DIR    := $(CURDIR)
BUILD_DIR  := $(TOP_DIR)/build

TARGET     := riscv32imac-unknown-none-elf
TARGET_DIR := $(TOP_DIR)/target/$(TARGET)/release
OBJCOPY    := riscv64-unknown-elf-objcopy

YCORE_DIR ?= $(TOP_DIR)/../YoriCore-Rev

SIM_BIN ?= $(YCORE_DIR)/build/verilator/VSimTop
REF_SO  ?= $(YCORE_DIR)/build/ref/riscv32-spike-so

EXPECT_FILE := $(TOP_DIR)/expected.list

DIFFTEST_SKIP_BINS ?= inst_lr_sc_store

ALL ?=
