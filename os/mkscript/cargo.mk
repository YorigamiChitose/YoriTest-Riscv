.PHONY: build bin

build:
	@printf '$(C_BLU)--- cargo build --release ---$(C_OFF)\n'
	cargo build --release

bin: build
	@mkdir -p $(BUILD_DIR)
	$(OBJCOPY) -O binary $(TARGET_DIR)/os $(BUILD_DIR)/os.bin
	@printf '$(C_GRN)--- $(BUILD_DIR)/os.bin ---$(C_OFF)\n'
