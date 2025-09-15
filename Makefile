BIN         := $(PWD)/.local/bin
CACHE       := $(PWD)/.local/cache
RUSTUP_HOME := $(CACHE)/rustup
CARGO_HOME  := $(CACHE)/cargo
PATH        := $(BIN):$(CARGO_HOME)/bin:/usr/bin:/bin
SHELL       := env PATH=$(PATH) RUSTUP_HOME=$(RUSTUP_HOME) CARGO_HOME=$(CARGO_HOME) /bin/sh

# Project Name
project_name := personal-mcp-server

# Versions
rustup_version          := 1.28.2
cargo_audit_version     := 0.21.2
cargo_auditable_version := 0.7.0

# Architecture, Vendor, and Operating System
arch   := $(shell uname -m)
vendor := $(shell cc -dumpmachine | awk -F- '{print $$2}')
os     := $(shell uname|tr A-Z a-z)

ifeq ($(arch),arm64)
	arch = aarch64
endif

# TODO: Handle musl libc distributions.
ifeq ($(vendor),linux)
	vendor = unknown
	os     = linux-gnu
endif

# Rust Tooling
target_triple      := $(arch)-$(vendor)-$(os)
rustup_package_url := https://static.rust-lang.org/rustup/archive/$(rustup_version)/$(target_triple)/rustup-init

.PHONY: all
all: format lint release test audit

.PHONY: tools
tools: $(CARGO_HOME)/bin/cargo $(CARGO_HOME)/bin/cargo-audit $(CARGO_HOME)/bin/cargo-auditable

.PHONY: build
build: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Building using the dev profile..."
	@cargo build

.PHONY: release
release: $(CARGO_HOME)/bin/cargo-auditable
	@printf '%s\n' "Building using the release profile with dependency lists embedded in the binaries..."
	@cargo auditable build --release

$(CARGO_HOME)/bin/cargo-auditable: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Installing cargo-auditable to produce auditable binaries..."
	@cargo install cargo-auditable --version $(cargo_auditable_version) --locked >/dev/null 2>&1

.PHONY: run
run: $(CARGO_HOME)/bin/cargo
	@printf '%s\n'   "Building using the dev profile and running..."
	@printf '%s\n'   "--> This Make target passes any arguments listed in the ARGS"
	@printf '%s\n'   "--> variable to the program being run by Cargo like:"
	@printf '%s\n\n' '--> make run ARGS="arg1 arg2 etc"'
	@set -- $(ARGS) && cargo run -- "$$@"

.PHONY: test
test: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Building using the test profile and testing..."
	@cargo test

.PHONY: lint
lint: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Linting code style with rustfmt..."
	@cargo fmt --check
	@printf '%s\n' "Linting code with Clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: check
check: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Checking code for errors..."
	@cargo check

.PHONY: format
format: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Formatting all files..."
	@cargo fmt

.PHONY: docs
docs: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Generating and opening docs..."
	@cargo doc --open

.PHONY: audit
audit: release $(CARGO_HOME)/bin/cargo-audit
	@printf '%s\n' "Scanning the release binary for vulnerabilities..."
	@cargo audit bin target/release/$(project_name)

$(CARGO_HOME)/bin/cargo-audit: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Installing cargo-audit to scan source and binaries for security vulnerabilities..."
	@cargo install cargo-audit --version $(cargo_audit_version) --locked --features=fix >/dev/null 2>&1

.PHONY: update
update: $(CARGO_HOME)/bin/cargo
	@printf '%s\n' "Updating dependencies as recorded in the local lock file..."
	@cargo update

$(CARGO_HOME)/bin/cargo: $(BIN)/rustup-init rust-toolchain.toml
	@printf '%s\n' "Initializing the Rust toolchain..."
	@rustup-init --default-toolchain none --no-modify-path -y >/dev/null 2>&1
	@printf '%s\n' "Setting up Cargo..."
	@rustup toolchain install >/dev/null 2>&1
	@printf '%s\n' "Finished setting up Cargo:"
	@cargo --version
	@# Update the target timestamp so its newer than rust-toolchain.toml.
	@touch $@

$(BIN)/rustup-init:
	@printf '%s\n' "Fetching rustup-init from $(rustup_package_url)..."
	@mkdir -p $(BIN)
	@mkdir -p $(RUSTUP_HOME)
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(rustup_package_url)
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(rustup_package_url).sha256
	@cd $(BIN) && shasum -a 256 -c $(BIN)/rustup-init.sha256 >/dev/null 
	@chmod +x $(BIN)/rustup-init

.PHONY: clean
clean:
	@printf '%s\n' "Removing generated artifacts..."
	@[ "$$(command -v cargo)" ] && cargo clean || true
	@printf '%s\n' "Removing the $(CACHE) directory..."
	@rm -rf $(CACHE)
	@printf '%s\n' "Removing the $(BIN) directory..."
	@rm -rf $(BIN)
	@printf '%s\n' "Removing the $(PWD)/.local directory..."
	@if [ -d "$(PWD)/.local" ]; then rmdir "$(PWD)/.local"; fi

.PHONY: debug
debug:
	@printf '%s\n' "$(ARGS)"
	@set -- "$(ARGS)" && printf '%s\n' "$$1"
