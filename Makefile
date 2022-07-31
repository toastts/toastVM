all:
	test prod

test:
	cargo test

prod:
	cargo build --release

dev:
	cargo build

