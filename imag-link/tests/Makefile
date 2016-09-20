ECHO=$(shell which echo) -e
TARGETS=$(shell find -name "*test.sh" -type f)
BASH=$(shell which bash)

all: $(TARGETS)
	@$(ECHO) $(TARGETS)

$(TARGETS): %: .FORCE
	@$(ECHO) "\t[BASH   ]:\t$@"
	@$(BASH) $@

.FORCE:

