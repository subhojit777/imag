ECHO=$(shell which echo) -e
RUBY=$(shell which ruby)
RUBY_TESTS=$(shell find ./test -maxdepth 1 -name "*.rb" -type f)
RUBY_TEST_TARGETS=$(foreach x,$(subst ,,$(RUBY_TESTS)),$(x))

all: lib

lib:
	$(MAKE) -C .. imag-ruby

lib-release:
	$(MAKE) -C .. imag-ruby-release

test: lib $(RUBY_TEST_TARGETS)

$(RUBY_TEST_TARGETS): %: lib .FORCE
	@$(ECHO) "\t[TEST   ]:\t$@"
	@$(RUBY) $(subst -test,,$@)

.FORCE:
