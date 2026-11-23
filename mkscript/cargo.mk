# ===============================================================
#  cargo 编译与镜像转换（编译交 cargo，本文件只编排）
# ===============================================================

.PHONY: build bin

# 编译：ALL 为空编译全部 bin，否则只编译指定 bin（可空格分隔多个）。
# 用 --bin 只编译目标 bin，避免跑单测试时仍全量编译。
build:
	@if [ -z "$(ALL)" ]; then \
		echo "--- cargo build --release ---"; \
		cargo build --release || exit 1; \
	else \
		for b in $(ALL); do \
			echo "--- cargo build --release --bin $$b ---"; \
			cargo build --release --bin $$b || exit 1; \
		done; \
	fi
	@echo "--- 编译完成 ---"

# 将 ELF 转成裸二进制镜像到 build/*.bin（ALL 为空转全部，否则只转指定 bin）
bin: build
	@mkdir -p $(BUILD_DIR)
	@if [ -z "$(ALL)" ]; then \
		names=$$(find $(TARGET_DIR) -maxdepth 1 -type f -executable -printf '%f\n'); \
	else \
		names="$(ALL)"; \
	fi; \
	for name in $$names; do \
		$(OBJCOPY) -O binary "$(TARGET_DIR)/$$name" "$(BUILD_DIR)/$$name.bin" && \
			echo "  [bin] $$name.bin"; \
	done
