RUST_BACKTRACE=1

.PHONY: all
all: build

.PHONY: build
build:
	wasm-pack build

.PHONY: build-prod
build-prod:
	wasm-pack build --release

.PHONY: lint
lint:
	cargo check --target wasm32-unknown-unknown --features "console_error_panic_hook wee_alloc"
	cargo fmt
	cargo clippy

.PHONY: test
test:
	cargo test
	# wasm-pack test --chrome --firefox --headless
