include $(CURDIR)/mkscript/env.mk
include $(CURDIR)/mkscript/config.mk
include $(CURDIR)/mkscript/cargo.mk
include $(CURDIR)/mkscript/run.mk
include $(CURDIR)/mkscript/clean.mk

.PHONY: help

help:
	@printf '$(C_BLU)可用目标:$(C_OFF)\n'
	@printf '  $(C_YEL)build$(C_OFF)     cargo build --release\n'
	@printf '  $(C_YEL)bin$(C_OFF)       objcopy 转 .bin 到 build/\n'
	@printf '  $(C_YEL)run$(C_OFF)       运行 OS 自检\n'
	@printf '  $(C_YEL)difftest$(C_OFF)  OS 确定性 difftest\n'
	@printf '  $(C_YEL)clean$(C_OFF)     清理构建产物\n'

.DEFAULT_GOAL := help
