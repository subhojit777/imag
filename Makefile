toml = imag-$@/Cargo.toml
bin = imag-$@/target/debug/imag-$@

default: all

.PHONY: clean

all: counter link notes store tag view

counter: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

link: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

notes: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

store: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

tag: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

view: prep
	cargo build --manifest-path $(toml)
	cp $(bin) out/

prep:
	mkdir -p out/

clean:
	rm -rf out/
