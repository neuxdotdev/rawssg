# Makefile for RawSSG project

CARGO := cargo
BIN := target/release/rawssg
INSTALL_DIR := /usr/local/bin

.PHONY: help build release install uninstall git

help:
	@echo "RawSSG Makefile commands:"
	@echo "  make build         Build debug version"
	@echo "  make release       Build release version"
	@echo "  make install       Install binary globally"
	@echo "  make uninstall     Remove installed binary"
	@echo "  make git status    Show git status"
	@echo "  make git pull      Pull latest changes"
	@echo "  make git push      Push changes"
	@echo "  make help          Show this help"

build:
	$(CARGO) build
	@echo "✅ Debug build complete!"

release:
	$(CARGO) build --release
	@echo "✅ Release build complete!"

install: release
	@echo "Installing RawSSG..."
	@cp $(BIN) $(INSTALL_DIR)
	@chmod +x $(INSTALL_DIR)/rawssg
	@echo "✅ Installed to $(INSTALL_DIR)/rawssg"

uninstall:
	@echo "Uninstalling RawSSG..."
	@rm -f $(INSTALL_DIR)/rawssg
	@echo "✅ Uninstalled"

verbose:
	$(CARGO) build --verbose
