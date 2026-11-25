.PHONY: run check-tests

check-tests:
	@mkdir -p $(BUILD_DIR)
	@printf '%s\n' $(EXPECTED_BINS) | sort > $(BUILD_DIR)/.expected-bins
	@printf '%s\n' $(TEST_BINS) | sort > $(BUILD_DIR)/.test-bins
	@diff -u $(BUILD_DIR)/.expected-bins $(BUILD_DIR)/.test-bins || { \
		echo "expected.list 与测试源不一致"; exit 1; \
	}
	@echo "test manifest OK"

run: bin
	@bins="$(ALL)"; \
	if [ -z "$$bins" ]; then \
		bins="$(EXPECTED_BINS)"; \
	fi; \
	pass=0; fail=0; rows=""; \
	for b in $$bins; do \
		printf '$(C_YEL)--- run %s ---$(C_OFF)\n' "$$b"; \
		if [ ! -f "$(BUILD_DIR)/$$b.bin" ]; then \
			fail=$$((fail+1)); rows="$${rows}  $(C_RED)[MISSING]$(C_OFF) $$b\n"; \
			continue; \
		fi; \
		log=$$(timeout $(TEST_TIMEOUT)s env -u LD_LIBRARY_PATH $(SIM_BIN) -b "$(BUILD_DIR)/$$b.bin" 2>&1); rc=$$?; \
		if [ $$rc -eq 124 ] || [ $$rc -eq 137 ]; then actual=TIMEOUT; \
		elif [ $$rc -eq 0 ]; then actual=0; \
		else actual=$$(printf '%s\n' "$$log" | grep -oE 'code=[0-9]+' | head -1 | cut -d= -f2); fi; \
		expected=$$(grep -E "^$$b[[:space:]]" $(EXPECT_FILE) | head -1 | awk '{print $$2}'); \
		[ -z "$$expected" ] && expected=0; \
		if [ "$$actual" = "$$expected" ]; then \
			pass=$$((pass+1)); rows="$${rows}  $(C_GRN)[PASS][$$actual==$$expected]$(C_OFF) $$b\n"; \
		else \
			fail=$$((fail+1)); rows="$${rows}  $(C_RED)[FAIL][$$actual!=$$expected]$(C_OFF) $$b\n"; \
			printf '%s\n' "$$log"; \
		fi; \
	done; \
	printf '\n$(C_BLU)=== 汇总 ===$(C_OFF)\n'; \
	printf '%b' "$${rows}"; \
	printf '$(C_BLU)  总计 %d，通过 %d，失败 %d$(C_OFF)\n' "$$((pass+fail))" "$$pass" "$$fail"; \
	[ $$fail -eq 0 ]
