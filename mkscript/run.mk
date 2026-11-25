.PHONY: run

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

