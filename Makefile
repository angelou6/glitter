.PHONY: build install uninstall

BIN = target/release/glitter
PREFIX ?= /usr/local
BINNAME ?= glitter

build:
	cargo build --release

install: build
	install -Dm755 $(BIN) $(DESTDIR)$(PREFIX)/bin/$(BINNAME)

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BINNAME)
