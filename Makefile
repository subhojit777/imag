toml = $@/Cargo.toml
bin = $@/target/debug/$@

default: all

.PHONY: clean

all: imag-counter imag-link imag-notes imag-store imag-tag imag-view

imag-%: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

prep:
	mkdir -p out/

clean:
	rm -rf out/
