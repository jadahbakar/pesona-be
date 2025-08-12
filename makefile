# ---------------------------------------------------------------------------------------------------
# Project Variables & Configuration
# ---------------------------------------------------------------------------------------------------
# Define tools to ensure consistency
CARGO := cargo
GOOSE := goose
DOCKER := docker

# ---------------------------------------------------------------------------------------------------
# Phony Targets (Commands that are not files)
# ---------------------------------------------------------------------------------------------------
.PHONY: install run build test test-cov format lint clean migrate-up migrate-down migrate-fix docker-build help git

# ---------------------------------------------------------------------------------------------------
# Development Workflow
# ---------------------------------------------------------------------------------------------------

install: ## Install all required development tools.
	@echo ">> Installing development tools (cargo-watch, cargo-tarpaulin, goose)..."
	@$(CARGO) install cargo-watch
	@$(CARGO) install cargo-tarpaulin
	@go install github.com/pressly/goose/v3/cmd/goose@latest

run: ## Run the application in watch mode for live reloading.
	@echo ">> Starting application in watch mode..."
	@$(CARGO) watch -q -c -w src -x run

build: ## Build the application for release with optimizations.
	@echo ">> Building release binary..."
	@$(CARGO) build --release

clean: ## Remove build artifacts.
	@echo ">> Cleaning up build artifacts..."
	@$(CARGO) clean

# ---------------------------------------------------------------------------------------------------
# Help
# ---------------------------------------------------------------------------------------------------

help: ## Show this help message.
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST) | sort

# ---------------------------------------------------------------------------------------------------
# Git Repository
# ---------------------------------------------------------------------------------------------------
# Usage: 
#   make git m="your commit message"
#   or
#   make git "your commit message" 
git:
	@git add . || (echo "Error in git add"; exit 1)
	@git commit -m "$(if $(m),$(m),$(filter-out $@,$(MAKECMDGOALS)))" || (echo "Error in git commit"; exit 1)
	@git push || (echo "Error in git push"; exit 1)