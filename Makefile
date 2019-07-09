.PHONY: all
all:
	$(MAKE) run

.PHONY: run
run:
	cargo run

.PHONY:  test
test:
	cargo test --verbose

.PHONY: setup
setup:
	curl https://sh.rustup.rs -sSf | sh
	rustup default nightly
	rustup update && cargo update

.PHONY: fmt
fmt:
	rustfmt --verbose --edition 2018 src/main.rs
