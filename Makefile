build:
	go build

install: build
	mv glitter /usr/local/bin
