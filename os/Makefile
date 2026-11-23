TARGET     := riscv32imac-unknown-none-elf
TARGET_DIR := target/$(TARGET)/release
BUILD_DIR  := build
OBJCOPY    := riscv64-unknown-elf-objcopy
SIM_BIN    ?= ../../YoriCore-Rev/build/verilator/VSimTop

.PHONY: all build bin run clean

all: run

build:
	cargo build --release

bin: build
	@mkdir -p $(BUILD_DIR)
	$(OBJCOPY) -O binary $(TARGET_DIR)/os $(BUILD_DIR)/os.bin

run: bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b $(BUILD_DIR)/os.bin

clean:
	cargo clean
	rm -rf $(BUILD_DIR)
