toml = $@/Cargo.toml
bin = $@/target/debug/$@

doc-crate-toml=./.imag-documentation/Cargo.toml

ECHO=$(shell which echo) -e
MAKE=$(shell which make)
BASH=$(shell which bash)
CARGO=$(shell which cargo)

BINS=$(shell find -maxdepth 1 -name "imag-*" -type d | sort)
LIBS=$(shell find -maxdepth 1 -name "libimag*" -type d | sort)

BIN_TARGETS=$(patsubst imag-%,,$(BINS))
BIN_TARGET_TESTS=$(foreach x,$(BIN_TARGETS),$(x)-test)
LIB_TARGETS=$(LIBS)
LIB_TARGETS_TEST=$(foreach x,$(subst ./,,$(LIBS)),$(x)-test)
TARGETS=$(BIN_TARGETS) $(LIB_TARGETS)
RELEASE_TARGETS=$(foreach x,$(TARGETS),$(x)-release)
INSTALL_TARGETS=$(foreach x,$(BIN_TARGETS),$(x)-install)
CHECK_TARGETS=$(foreach x,$(TARGETS),$(x)-check)
CHECK_OUTDATED_TARGETS=$(foreach x,$(TARGETS),$(x)-check-outdated)

all: $(TARGETS) imag-bin
	@$(ECHO) "\t[ALL    ]"

imag-bin:
	@$(ECHO) "\t[IMAG   ][BUILD  ]"
	@$(CARGO) build --manifest-path ./bin/Cargo.toml

imag-bin-release:
	@$(ECHO) "\t[IMAG   ][RELEASE]"
	@$(CARGO) build --release --manifest-path ./bin/Cargo.toml

imag-bin-install:
	@$(ECHO) "\t[IMAG   ][INSTALL]"
	@$(CARGO) install --force --path ./bin

imag-bin-check:
	@$(ECHO) "\t[IMAG   ][CHECK  ]"
	@$(CARGO) check --manifest-path ./bin/Cargo.toml

imag-bin-check-outdated:
	@$(ECHO) "\t[IMAG   ][CHECK  ]"
	@$(CARGO) outdated --manifest-path ./bin/Cargo.toml --lockfile-path ./bin/Cargo.lock

release: $(RELEASE_TARGETS) imag-bin-release
	@$(ECHO) "\t[RELEASE]"

bin: $(BIN_TARGETS) imag-bin
	@$(ECHO) "\t[ALLBIN ]"

bin-test: $(BIN_TARGET_TESTS) imag-bin

lib: $(LIB_TARGETS)
	@$(ECHO) "\t[ALLLIB ]"

lib-test: $(LIB_TARGETS_TEST)

lib-imag-ruby-test:
	@$(MAKE) -C libimagruby

test: bin-test lib-test

install: $(INSTALL_TARGETS) imag-bin-install
	@$(ECHO) "\t[INSTALL]"

update:
	@$(ECHO) "\t[UPDATE ]"
	@$(CARGO) update

clean:
	@$(ECHO) "\t[CLEAN  ]"
	@$(CARGO) clean

check: $(CHECK_TARGETS) imag-bin-check

check-outdated: $(CHECK_OUTDATED_TARGETS) imag-bin-check-outdated

$(TARGETS): %: .FORCE
	@$(ECHO) "\t[CARGO  ]:\t$@"
	@$(CARGO) build --manifest-path ./$@/Cargo.toml

$(BIN_TARGET_TESTS): %-test: % .FORCE
	@$(ECHO) "\t[BINTEST]:\t$@"
	if [ -f $(subst -test,,$@)/tests/Makefile ]; then \
		$(MAKE) -C $(subst -test,,$@)/tests || exit 1;\
	fi;

$(RELEASE_TARGETS): %: .FORCE
	@$(ECHO) "\t[RELEASE]:\t$(subst -release,,$@)"
	@$(CARGO) build --release --manifest-path ./$(subst -release,,$@)/Cargo.toml

$(LIB_TARGETS_TEST): %: .FORCE
	@$(ECHO) "\t[TEST   ]:\t$@"
	@$(CARGO) test --manifest-path ./$(subst -test,,$@)/Cargo.toml

$(INSTALL_TARGETS): %: .FORCE imag-bin-install
	@$(ECHO) "\t[INSTALL]:\t$(subst -install,,$@)"
	@$(CARGO) install --force --path ./$(subst -install,,$@)

$(CHECK_TARGETS): %: .FORCE
	@$(ECHO) "\t[CHECK  ]:\t$(subst -check,,$@)"
	@$(CARGO) check --manifest-path ./$(subst -check,,$@)/Cargo.toml

$(CHECK_OUTDATED_TARGETS): %: .FORCE $(subst -check-outdated,-update,$@)
	@$(ECHO) "\t[OUTDATE]:\t$(subst -check-outdated,,$@)"
	@$(CARGO) outdated -R										  \
		--manifest-path $(PWD)/$(subst -check-outdated,,$@)/Cargo.toml \
		--lockfile-path $(PWD)/$(subst -check-outdated,,$@)/Cargo.lock

.FORCE:

