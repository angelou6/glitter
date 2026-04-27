build:
	cargo build --release

install: build
	cp target/release/glitter /usr/local/bin

local_install: build
	cp target/release/glitter ~/.local/bin
