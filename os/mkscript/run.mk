.PHONY: run difftest

run: bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b $(BUILD_DIR)/os.bin

difftest:
	@test -f "$(REF_SO)" || { echo "missing ref: $(REF_SO)"; exit 1; }
	@printf '$(C_BLU)--- cargo build --release --features difftest ---$(C_OFF)\n'
	CARGO_TARGET_DIR=$(DIFF_TARGET_DIR) cargo build --release --features difftest
	@mkdir -p $(BUILD_DIR)
	$(OBJCOPY) -O binary $(DIFF_TARGET_DIR)/$(TARGET)/release/os $(BUILD_DIR)/os-difftest.bin
	env -u LD_LIBRARY_PATH $(SIM_BIN) -b --diff="$(REF_SO)" $(BUILD_DIR)/os-difftest.bin
