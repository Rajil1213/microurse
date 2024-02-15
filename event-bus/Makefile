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

.PHONY: image
image:
	docker build -t whitenoise1213/blog-event-bus:0.0.1 .

.PHONY: deploy
deploy:
	docker compose up -d event-bus
