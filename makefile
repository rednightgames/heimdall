gateway:
	cargo watch -x "run --bin gateway" -w ./services/gateway/

config:
	cargo watch -x "run --bin config" -w ./services/config/