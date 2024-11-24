up:
	docker compose up -d
	. ./scripts/setup_keycloak.sh

clippy:
	cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

format:
	cargo +nightly fmt

watch:
	cargo watch -w backend/src -x "run --bin backend"

deps:
	brew install redis libpq curl jq openssh openssl pkg-config
	cargo install sqlx-cli cargo-watch cargo-expand tokio-console
	rustup install nightly
	chmod -R +x ./scripts
