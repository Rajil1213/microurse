.PHONY: run
run:
	cargo run

.PHONY: watch
watch:
	cargo watch -q -c -w src/ -x "run"

.PHONY: lint
lint:
	cargo clippy

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: deploy
deploy:
	docker compose up -d posts
