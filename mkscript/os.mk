.PHONY: os-build os-bin os-run os-clean

OS_BIN      := $(OS_BUILD_DIR)/os.bin

os-build:
	cargo build --release -p os

os-bin: os-build
	@mkdir -p $(OS_BUILD_DIR)
	$(OBJCOPY) -O binary $(TARGET_DIR)/os $(OS_BIN)

os-run: os-bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b $(OS_BIN)

os-clean:
	cargo clean -p os
	rm -rf $(OS_BUILD_DIR)
