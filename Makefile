.DEFAULT_GOAL := help
.PHONY: help

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| sed -n 's/^\(.*\): \(.*\)##\(.*\)/\1\3/p' \
	| column -t  -s ' '

lint: ## Run linting tools
	cargo clippy

fmt: ## Format the code
	cargo fmt

clean: ## Clean the project
	cargo clean

run: ## Build and install to the device
	cargo run --release
