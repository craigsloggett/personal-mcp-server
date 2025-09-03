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

.PHONY: all
all: #format lint test
	@printf '%s\n' "$(arch)-$(vendor)-$(os)"

.PHONY: tools
tools: $(CARGO_HOME)/bin/cargo rust

# Setup rustup
target_triple      := $(arch)-$(vendor)-$(os)
rustup_package_url := https://static.rust-lang.org/rustup/archive/$(rustup_version)/$(target_triple)/rustup-init
toolchain_sha256   := $(shell shasum -a 256 $(PWD)/rust-toolchain.toml | awk '{ print $$1 }')

$(BIN)/rustup-init:
	@mkdir -p $(BIN)
	@mkdir -p $(RUSTUP_HOME)
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(rustup_package_url)
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(rustup_package_url).sha256
	@cd $(BIN) && shasum -a 256 -c $(BIN)/rustup-init.sha256 >/dev/null 
	@chmod +x $(BIN)/rustup-init

$(CACHE)/rust-toolchain.toml.sha256: rust-toolchain.toml
	@echo "Capturing rust-toolchain.toml hash..."
	@printf '%s' "$(toolchain_sha256)" > $(CACHE)/rust-toolchain.toml.sha256

$(CARGO_HOME)/bin/cargo: $(BIN)/rustup-init $(CACHE)/rust-toolchain.toml.sha256
	@echo "Initializing the rust toolchain..."
	@rustup-init --default-toolchain none --no-modify-path -y >/dev/null 2>&1
	@echo "Setting up Cargo..."
	@rustup toolchain install >/dev/null 2>&1
	@rustc --version
	@cargo --version

.PHONY: rust
rust: $(CACHE)/rust-toolchain.toml.sha256
	@echo $(toolchain_sha256)

.PHONY: update
update: $(BIN)/go
	@echo "Updating dependencies..."

.PHONY: build
build: update
	@echo "Building..."

.PHONY: format
format: tools
	@echo "Formatting..."

.PHONY: lint
lint: tools build
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
