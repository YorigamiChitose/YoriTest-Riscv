.PHONY: clean

clean:
	@printf '$(C_RED)--- 清理构建产物 ---$(C_OFF)\n'
	cargo clean -p os
	rm -rf $(BUILD_DIR) $(DIFF_TARGET_DIR)
