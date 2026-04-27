.PHONY: build install local_install

BIN = target/release/glitter

build:
	cargo build --release

install: build
	install -Dm755 $(BIN) /usr/local/bin/glitter

local_install: build
	install -Dm755 $(BIN) ~/.local/bin/glitter
