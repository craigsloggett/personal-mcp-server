BIN           := $(PWD)/.local/bin
CACHE         := $(PWD)/.local/cache
GOPATH        := $(CACHE)/go
PATH          := $(BIN):$(PATH)
SHELL         := env PATH=$(PATH) GOPATH=$(GOPATH) /bin/sh

# Versions
go_version          := 1.25.0
staticcheck_version := 2025.1.1

# Operating System and Architecture
os ?= $(shell uname|tr A-Z a-z)

ifeq ($(shell uname -m),x86_64)
  arch   ?= amd64
endif
ifeq ($(shell uname -m),arm64)
  arch     ?= arm64
endif

.PHONY: all
all: format lint install docs test

.PHONY: tools
tools: $(BIN)/go $(BIN)/staticcheck

# Setup Go
go_package_name := go$(go_version).$(os)-$(arch)
go_package_url  := https://go.dev/dl/$(go_package_name).tar.gz
go_install_path := $(BIN)/go-$(go_version)-$(os)-$(arch)

$(BIN)/go:
	@mkdir -p $(BIN)
	@mkdir -p $(GOPATH)
	@echo "Downloading Go v$(go_version) to $(go_install_path)..."
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(go_package_url)
	@tar -C $(BIN) -xzf $(BIN)/$(go_package_name).tar.gz && rm $(BIN)/$(go_package_name).tar.gz
	@mv $(BIN)/go $(go_install_path)
	@ln -s $(go_install_path)/bin/go $(BIN)/go

# Setup Staticcheck
staticcheck_source_url   := https://github.com/dominikh/go-tools/archive/$(staticcheck_version).tar.gz
staticcheck_install_path := $(BIN)/staticcheck-$(staticcheck_version)-$(os)-$(arch)

$(BIN)/staticcheck:
	@mkdir -p $(BIN)
	@echo "Downloading Staticcheck v$(staticcheck_version) to $(staticcheck_install_path)..."
	@curl --silent --show-error --fail --create-dirs --output-dir $(BIN) -O -L $(staticcheck_source_url)
	@tar -C $(BIN) -xzf $(BIN)/$(staticcheck_version).tar.gz && rm $(BIN)/$(staticcheck_version).tar.gz
	@cd $(BIN)/go-tools-$(staticcheck_version) && go build -o $(BIN)/staticcheck ./cmd/staticcheck && cd -
	@rm -rf $(BIN)/go-tools-$(staticcheck_version) && mkdir -p $(staticcheck_install_path)
	@mv $(BIN)/staticcheck $(staticcheck_install_path)
	@ln -s $(staticcheck_install_path)/staticcheck $(BIN)/staticcheck

.PHONY: update
update: $(BIN)/go
	@echo "Updating dependencies..."
	@go get -u
	@go mod tidy

.PHONY: build
build: update
	@echo "Building..."
	@go build ./...

.PHONY: install
install: update
	@echo "Installing provider..."
	@go install ./...

.PHONY: format
format: tools
	@echo "Formatting..."
	@go fmt ./...

.PHONY: lint
lint: tools update
	@echo "Linting..."
	@staticcheck ./...

.PHONY: docs
docs: tools update install
	@echo "Generating Docs..."

.PHONY: test
test: install
	@echo "Testing..."

.PHONY: run
run: install
	@echo "Running..."
	@go run .

.PHONY: clean
clean:
	@echo "Removing the $(CACHE) directory..."
	@go clean -modcache
	@rm -rf $(CACHE)
	@echo "Removing the $(BIN) directory..."
	@rm -rf $(BIN)
	@echo "Removing the $(PWD)/.local directory..."
	@if [ -d "$(PWD)/.local" ]; then rmdir "$(PWD)/.local"; fi
