.PHONY: build bin

build:
	@if [ -z "$(ALL)" ]; then \
		echo "--- cargo build --release $(TEST_PKGS) ---"; \
		cargo build --release $(TEST_PKGS) || exit 1; \
	else \
		for b in $(ALL); do \
			echo "--- cargo build --release --bin $$b ---"; \
			cargo build --release --bin $$b || exit 1; \
		done; \
	fi
	@echo "--- 编译完成 ---"

bin: build
	@mkdir -p $(BUILD_DIR)
	@if [ -z "$(ALL)" ]; then \
		names="$(TEST_BINS)"; \
	else \
		names="$(ALL)"; \
	fi; \
	for name in $$names; do \
		$(OBJCOPY) -O binary "$(TARGET_DIR)/$$name" "$(BUILD_DIR)/$$name.bin" && \
			echo "  [bin] $$name.bin"; \
	done
