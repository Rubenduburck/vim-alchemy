# Detect OS and architecture
UNAME := $(shell uname)
ARCH := $(shell uname -m)

# Map architecture names to release naming convention
ifeq ($(ARCH), x86_64)
	RELEASE_ARCH := x86_64
else ifeq ($(ARCH), aarch64)
	RELEASE_ARCH := aarch64
else ifeq ($(ARCH), arm64)
	RELEASE_ARCH := aarch64
else
	$(error Unsupported architecture: $(ARCH))
endif

# Map OS names to release naming convention
ifeq ($(UNAME), Linux)
	RELEASE_OS := linux
	BINARY_EXT := 
else ifeq ($(UNAME), Darwin)
	RELEASE_OS := macos
	BINARY_EXT := 
# Windows not supported due to rug dependency
else
	$(error Unsupported operating system: $(UNAME))
endif

# GitHub repository and release settings
REPO := rubenduburck/alchemy
BINARY_NAME := alchemy-$(RELEASE_ARCH)-$(RELEASE_OS)$(BINARY_EXT)
BIN_DIR := $(shell pwd)/bin
INSTALLED_BINARY := $(BIN_DIR)/alchemy$(BINARY_EXT)

# Read the required alchemy version from .alchemy-version file
REQUIRED_VERSION := $(shell cat .alchemy-version 2>/dev/null || echo "latest")

# Determine download URL
ifeq ($(REQUIRED_VERSION), latest)
	VERSION_TAG := $(shell curl -s https://api.github.com/repos/$(REPO)/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
else
	VERSION_TAG := $(REQUIRED_VERSION)
endif

DOWNLOAD_URL := https://github.com/$(REPO)/releases/download/$(VERSION_TAG)/$(BINARY_NAME)

all: install

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

install: $(BIN_DIR)
	@echo "Installing alchemy for $(RELEASE_OS)-$(RELEASE_ARCH)..."
	@curl -L -f $(DOWNLOAD_URL) -o $(INSTALLED_BINARY) 2>/dev/null || \
		(echo "Failed to download alchemy. Please check your internet connection." && exit 1)
	@chmod +x $(INSTALLED_BINARY)
	@echo "✅ Installation complete!"

clean:
	rm -rf $(BIN_DIR)

# Check if binary is installed and working (silent unless there's an issue)
check:
	@if [ -f "$(INSTALLED_BINARY)" ]; then \
		if $(INSTALLED_BINARY) --help > /dev/null 2>&1; then \
			echo "✅ Plugin-local alchemy binary found at: $(INSTALLED_BINARY)"; \
			echo "   Version: $$($(INSTALLED_BINARY) --version)"; \
		else \
			echo "❌ alchemy binary is not working properly"; \
		fi \
	else \
		echo "❌ alchemy not found. Plugin will download it automatically on next use."; \
	fi

.PHONY: all install clean check