BIN         := $(PWD)/.local/bin
CACHE       := $(PWD)/.local/cache
RUSTUP_HOME := $(CACHE)/rustup
CARGO_HOME  := $(CACHE)/cargo
PATH        := $(BIN):$(CARGO_HOME)/bin:/usr/bin:/bin
SHELL       := env PATH=$(PATH) RUSTUP_HOME=$(RUSTUP_HOME) CARGO_HOME=$(CARGO_HOME) /bin/sh

# Versions
rustup_version := 1.28.2

# Architecture, Vendor, and Operating System
arch   := $(shell uname -m)
vendor := $(shell cc -dumpmachine | awk -F- '{print $$2}')
os     := $(shell uname|tr A-Z a-z)

ifeq ($(arch),arm64)
	arch = aarch64
endif

# Rust Tooling
target_triple      := $(arch)-$(vendor)-$(os)
rustup_package_url := https://static.rust-lang.org/rustup/archive/$(rustup_version)/$(target_triple)/rustup-init

.PHONY: all
all: tools

.PHONY: tools
tools: $(CARGO_HOME)/bin/cargo

$(BIN)/rustup-init:
	@echo "Fetching rustup-init from $(rustup_package_url)..."
	@mkdir -p $(BIN)
	@mkdir -p $(RUSTUP_HOME)
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(rustup_package_url)
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(rustup_package_url).sha256
	@cd $(BIN) && shasum -a 256 -c $(BIN)/rustup-init.sha256 >/dev/null 
	@chmod +x $(BIN)/rustup-init

$(CARGO_HOME)/bin/cargo: rust-toolchain.toml $(BIN)/rustup-init
	@echo "Initializing the Rust toolchain..."
	@rustup-init --default-toolchain none --no-modify-path -y >/dev/null 2>&1
	@echo "Setting up Cargo..."
	@rustup toolchain install >/dev/null 2>&1
	@echo "Finished setting up Cargo:"
	@rustc --version
	@cargo --version
	@# Update the target timestamp so its newer than rust-toolchain.toml.
	@touch $@

.PHONY: update
update: tools
	@echo "Updating dependencies..."

.PHONY: build
build: tools
	@echo "Building..."

.PHONY: format
format: tools
	@echo "Formatting..."

.PHONY: lint
lint: build
	@echo "Linting..."

.PHONY: test
test: build
	@echo "Testing..."

.PHONY: clean
clean:
	@echo "Removing the $(CACHE) directory..."
	@rm -rf $(CACHE)
	@echo "Removing the $(BIN) directory..."
	@rm -rf $(BIN)
	@echo "Removing the $(PWD)/.local directory..."
	@if [ -d "$(PWD)/.local" ]; then rmdir "$(PWD)/.local"; fi
