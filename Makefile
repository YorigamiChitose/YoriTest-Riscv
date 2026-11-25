include $(CURDIR)/mkscript/env.mk
include $(CURDIR)/mkscript/config.mk
include $(CURDIR)/mkscript/cargo.mk
include $(CURDIR)/mkscript/run.mk
include $(CURDIR)/mkscript/os.mk
include $(CURDIR)/mkscript/clean.mk

.PHONY: help

help:
	@printf '$(C_BLU)可用目标:$(C_OFF)\n'
	@printf '  $(C_YEL)build$(C_OFF)      编译测试框架（am/cpu-test/am-test）\n'
	@printf '  $(C_YEL)bin$(C_OFF)        生成测试镜像到 build/\n'
	@printf '  $(C_YEL)run$(C_OFF)        运行测试镜像（ALL=<bin 名>）\n'
	@printf '  $(C_YEL)os-build$(C_OFF)   编译 OS\n'
	@printf '  $(C_YEL)os-bin$(C_OFF)     生成 OS 镜像到 build/os/\n'
	@printf '  $(C_YEL)os-run$(C_OFF)     运行 OS 自检\n'
	@printf '  $(C_YEL)test$(C_OFF)      运行全部测试与 OS 自检\n'
	@printf '  $(C_YEL)os-clean$(C_OFF)   清理 OS 产物\n'
	@printf '  $(C_YEL)clean$(C_OFF)      清理整个项目\n'

.PHONY: test

test: check-tests run os-run

.DEFAULT_GOAL := help
