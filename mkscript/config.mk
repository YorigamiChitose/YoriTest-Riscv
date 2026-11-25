TOP_DIR    := $(CURDIR)
BUILD_DIR  := $(TOP_DIR)/build

TARGET     := riscv32imac-unknown-none-elf
TARGET_DIR := $(TOP_DIR)/target/$(TARGET)/release
OBJCOPY    := riscv64-unknown-elf-objcopy

YCORE_DIR ?= $(TOP_DIR)/../YoriCore-Rev

SIM_BIN ?= $(YCORE_DIR)/build/verilator/VSimTop

EXPECT_FILE := $(TOP_DIR)/expected.list

TEST_PKGS := -p am -p cpu-test -p am-test
TEST_BINS := $(notdir $(basename $(wildcard $(TOP_DIR)/cpu-test/src/bin/*.rs) $(wildcard $(TOP_DIR)/am-test/src/bin/*.rs)))

OS_BUILD_DIR       := $(BUILD_DIR)/os

ALL ?=

TEST_TIMEOUT ?= 60
OS_TIMEOUT   ?= 60
EXPECTED_BINS := $(shell awk 'NF {print $$1}' $(EXPECT_FILE))
