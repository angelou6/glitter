build:
	go build

install: build
	mv gitfuckyou /usr/local/bin
