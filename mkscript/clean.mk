.PHONY: clean

clean:
	@printf '$(C_RED)--- 清理构建产物 ---$(C_OFF)\n'
	cargo clean
	rm -rf $(BUILD_DIR) $(OS_DIFF_TARGET_DIR)
