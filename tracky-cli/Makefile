# ---------------------------------------
# Tracky CLI — Makefile
# ---------------------------------------

# Name of the binary crate
BIN = tracky-cli

# Cargo bin directory
CARGO_BIN = $(HOME)/.cargo/bin

# Default target
.PHONY: help
help:
	@echo ""
	@echo "Tracky CLI Makefile"
	@echo "--------------------"
	@echo "make build         - Build debug version"
	@echo "make run ARGS='...' - Run the CLI (example: make run ARGS='scan /music')"
	@echo "make release       - Build release version"
	@echo "make install       - Install binary globally"
	@echo "make reinstall     - Build & install"
	@echo "make uninstall     - Remove from ~/.cargo/bin"
	@echo "make clean         - Clean target directory"
	@echo ""

# Build debug
.PHONY: build
build:
	cargo build

# Run with arguments: make run ARGS="scan /music"
.PHONY: run
run:
	cargo run -- $(ARGS)

# Build optimized release
.PHONY: release
release:
	cargo build --release

# Install system-wide
.PHONY: install
install:
	cargo install --path . --force
	@echo "Installed at: $(CARGO_BIN)/$(BIN)"

# One-shot: build + install
.PHONY: reinstall
reinstall: build install

# Remove installed binary
.PHONY: uninstall
uninstall:
	rm -f $(CARGO_BIN)/$(BIN)
	@echo "Removed $(BIN) from $(CARGO_BIN)"

# Clean entire build
.PHONY: clean
clean:
	cargo clean
