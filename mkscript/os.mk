.PHONY: os-build os-bin os-run os-difftest os-clean

OS_BIN      := $(OS_BUILD_DIR)/os.bin
OS_DIFF_BIN := $(OS_BUILD_DIR)/os-difftest.bin

os-build:
	cargo build --release -p os

os-bin: os-build
	@mkdir -p $(OS_BUILD_DIR)
	$(OBJCOPY) -O binary $(TARGET_DIR)/os $(OS_BIN)

os-run: os-bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b $(OS_BIN)

os-difftest:
	@test -f "$(REF_SO)" || { echo "missing ref: $(REF_SO)"; exit 1; }
	CARGO_TARGET_DIR=$(OS_DIFF_TARGET_DIR) cargo build --release -p os --features difftest
	@mkdir -p $(OS_BUILD_DIR)
	$(OBJCOPY) -O binary $(OS_DIFF_TARGET_DIR)/$(TARGET)/release/os $(OS_DIFF_BIN)
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b --diff="$(REF_SO)" $(OS_DIFF_BIN)

os-clean:
	cargo clean -p os
	rm -rf $(OS_BUILD_DIR) $(OS_DIFF_TARGET_DIR)
