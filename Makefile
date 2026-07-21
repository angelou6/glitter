.PHONY: build install uninstall

BIN = glitter
PREFIX ?= /usr/local
BINNAME ?= glitter

build:
    go build -ldflags="-s -w" -trimpath

install: build
	install -Dm755 $(BIN) $(DESTDIR)$(PREFIX)/bin/$(BINNAME)

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BINNAME)
