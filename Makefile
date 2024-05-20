CC=cargo
FMT=fmt

OPTIONS=

default: fmt
	$(CC) build
	@make clippy

fmt:
	$(CC) fmt --all

check:
	$(CC) test --all

clean:
	$(CC) clean

clippy:
	$(CC) clippy --all --tests

release:
	$(CC) build --release