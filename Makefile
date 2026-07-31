.PHONY: build install update uninstall

BIN = glitter
PREFIX ?= /usr/local
BINNAME ?= glitter

build:
	go build -ldflags="-s -w" -trimpath

install: build
	install -Dm755 $(BIN) $(DESTDIR)$(PREFIX)/bin/$(BINNAME)

update: build
	install -Dm755 $(BIN) $(shell which $(BINNAME))

uninstall:
	rm -f $(shell which $(BINNAME))
