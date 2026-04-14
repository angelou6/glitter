build:
	cargo build --release

install: build
	cp target/release/glitter /usr/local/bin
