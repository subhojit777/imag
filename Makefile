toml = $@/Cargo.toml
bin = $@/target/debug/$@

doc-crate-toml=./.imag-documentation/Cargo.toml

ECHO=$(shell which echo) -e
CARGO=$(shell which cargo)

BINS=$(shell find -maxdepth 1 -name "imag-*" -type d)
LIBS=$(shell find -maxdepth 1 -name "libimag*" -type d)

BIN_TARGETS=$(patsubst imag-%,,$(BINS))
LIB_TARGETS=$(LIBS)
LIB_TARGETS_TEST=$(foreach x,$(subst ./,,$(LIBS)),test-$(x))
TARGETS=$(BIN_TARGETS) $(LIB_TARGETS)

all: $(TARGETS)
	@$(ECHO) "\t[ALL   ]"

bin: $(BIN_TARGETS)
	@$(ECHO) "\t[ALLBIN]"

lib: $(LIB_TARGETS)
	@$(ECHO) "\t[ALLLIB]"

lib-test: $(LIB_TARGETS_TEST)

$(TARGETS): %: .FORCE
	@$(ECHO) "\t[CARGO ]:\t$@"
	@$(CARGO) build --manifest-path ./$@/Cargo.toml

$(LIB_TARGETS_TEST): %: .FORCE
	@$(ECHO) "\t[TEST  ]:\t$@"
	@$(CARGO) test --manifest-path ./$(subst test-,,$@)/Cargo.toml

.FORCE:

