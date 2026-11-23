TARGET     := riscv32imac-unknown-none-elf
TARGET_DIR := ../target/$(TARGET)/release
BUILD_DIR  := build
DIFF_TARGET_DIR := ../target/os-difftest
OBJCOPY    := riscv64-unknown-elf-objcopy
SIM_BIN    ?= ../../YoriCore-Rev/build/verilator/VSimTop
REF_SO     ?= ../../YoriCore-Rev/build/ref/riscv32-spike-so

.PHONY: all build bin run difftest clean

all: run

build:
	cargo build --release

bin: build
	@mkdir -p $(BUILD_DIR)
	$(OBJCOPY) -O binary $(TARGET_DIR)/os $(BUILD_DIR)/os.bin

run: bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b $(BUILD_DIR)/os.bin

difftest:
	@test -f $(REF_SO) || { echo "missing ref: $(REF_SO)"; exit 1; }
	CARGO_TARGET_DIR=$(DIFF_TARGET_DIR) cargo build --release --features difftest
	@mkdir -p $(BUILD_DIR)
	$(OBJCOPY) -O binary $(DIFF_TARGET_DIR)/$(TARGET)/release/os $(BUILD_DIR)/os-difftest.bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b --diff=$(REF_SO) $(BUILD_DIR)/os-difftest.bin

clean:
	cargo clean -p os
	rm -rf $(BUILD_DIR) $(DIFF_TARGET_DIR)
