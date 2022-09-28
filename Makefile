prog :=http_server
debug ?=

ifdef debug
  release :=
  target :=debug
else
  release :=--release
  target :=release
endif

all: build install

build:
	cargo build $(release)

install:
	mkdir -p out/extensions
	cp target/$(target)/$(prog) out/$(prog)-$(target)
	cp target/$(target)/*.dylib out/extensions/ || true
	cp target/$(target)/*.so out/extensions/ || true
	cp target/$(target)/*.dll out/extensions/ || true

clean:
	cargo clean
	rm -rf out/

help:
	@echo "usage: make $(prog) [debug=1]"