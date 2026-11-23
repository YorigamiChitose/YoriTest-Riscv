# ===============================================================
#  YoriTest-Riscv 顶层批处理入口
#  编译交 cargo，本文件只编排流程（build → bin → run/difftest）。
# ===============================================================

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
	@printf '  $(C_YEL)run$(C_OFF)       运行镜像（默认全部，ALL=<bin 名> 指定单个）\n'
	@printf '  $(C_YEL)difftest$(C_OFF)  Spike 逐指令对比（ALL=<bin 名>）\n'
	@printf '  $(C_YEL)clean$(C_OFF)     清理构建产物\n'

.DEFAULT_GOAL := help
