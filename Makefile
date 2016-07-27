toml = $@/Cargo.toml
bin = $@/target/debug/$@

doc-crate-toml=./.imag-documentation/Cargo.toml

default: all

.PHONY: clean

all: imag-counter imag-link imag-notes imag-store imag-tag imag-view

imag-%: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

lib-doc:
	cargo build --manifest-path $(doc-crate-toml)

prep:
	mkdir -p out/

clean:
	rm -rf out/
