C_RED  := \e[31m
C_GRN  := \e[32m
C_YEL  := \e[33m
C_BLU  := \e[34m
C_OFF  := \e[0m

define log_start
	@printf '$(C_RED)--- %s ---$(C_OFF)\n' '$(1)'
endef

define log_done
	@printf '$(C_GRN)--- %s ---$(C_OFF)\n' '$(1)'
endef
