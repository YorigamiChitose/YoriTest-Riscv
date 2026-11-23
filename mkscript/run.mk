# ===============================================================
#  在仿真器上运行镜像 / difftest
#  run 直接调用 SIM_BIN（环境变量可覆盖），默认全部，ALL=<bin 名> 只跑指定。
#  每个 bin 输出原样透传，末尾按「实际 vs 预期」对比汇总。
#  预期读取 EXPECT_FILE（未列出的 bin 默认 PASS）；实际 FAIL 时提取 halt code。
# ===============================================================

.PHONY: run difftest

run: bin
	@bins="$(ALL)"; \
	if [ -z "$$bins" ]; then \
		bins=$$(find $(BUILD_DIR) -maxdepth 1 -name '*.bin' -printf '%f\n' | sed 's/\.bin$$//' | sort); \
	fi; \
	pass=0; fail=0; rows=""; \
	for b in $$bins; do \
		printf '$(C_YEL)--- run %s ---$(C_OFF)\n' "$$b"; \
		log=$$(env -u LD_LIBRARY_PATH $(SIM_BIN) -b "$(BUILD_DIR)/$$b.bin" 2>&1); rc=$$?; \
		printf '%s\n' "$$log"; \
		if [ $$rc -eq 0 ]; then actual=0; \
		else actual=$$(printf '%s\n' "$$log" | grep -oE 'code=[0-9]+' | head -1 | cut -d= -f2); fi; \
		expected=$$(grep -E "^$$b[[:space:]]" $(EXPECT_FILE) | head -1 | awk '{print $$2}'); \
		[ -z "$$expected" ] && expected=0; \
		if [ "$$actual" = "$$expected" ]; then \
			pass=$$((pass+1)); rows="$${rows}  $(C_GRN)[PASS][$$actual==$$expected]$(C_OFF) $$b\n"; \
		else \
			fail=$$((fail+1)); rows="$${rows}  $(C_RED)[FAIL][$$actual!=$$expected]$(C_OFF) $$b\n"; \
		fi; \
	done; \
	printf '\n$(C_BLU)=== 汇总 ===$(C_OFF)\n'; \
	printf '%b' "$${rows}"; \
	printf '$(C_BLU)  总计 %d，通过 %d，失败 %d$(C_OFF)\n' "$$((pass+fail))" "$$pass" "$$fail"

difftest: bin
	@test -n "$(ALL)" || { echo "usage: make difftest ALL=<bin 名，不含 .bin>"; exit 1; }
	$(MAKE) -C $(YCORE_DIR) difftest IMG="$(BUILD_DIR)/$(ALL).bin"
