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
REPO := Rubenduburck/vim-alchemy
BINARY_NAME := alchemy-$(RELEASE_ARCH)-$(RELEASE_OS)$(BINARY_EXT)
BIN_DIR := $(shell pwd)/bin
INSTALLED_BINARY := $(BIN_DIR)/alchemy$(BINARY_EXT)

# Get latest release tag from GitHub API
LATEST_TAG := $(shell curl -s https://api.github.com/repos/$(REPO)/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
DOWNLOAD_URL := https://github.com/$(REPO)/releases/download/$(LATEST_TAG)/$(BINARY_NAME)

all: install

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

install: $(BIN_DIR)
	@echo "Detected platform: $(RELEASE_OS)-$(RELEASE_ARCH)"
	@echo "Downloading latest release: $(LATEST_TAG)"
	@echo "From: $(DOWNLOAD_URL)"
	curl -L -f $(DOWNLOAD_URL) -o $(INSTALLED_BINARY)
	chmod +x $(INSTALLED_BINARY)
	@echo "✅ alchemy binary installed to $(INSTALLED_BINARY)"

clean:
	rm -rf $(BIN_DIR)

# Build from source (development)
build:
	CFLAGS="-std=gnu17" CXXFLAGS="-std=gnu++17" cargo build --release
	mkdir -p $(BIN_DIR)
	cp target/release/alchemy$(BINARY_EXT) $(INSTALLED_BINARY)
	@echo "✅ alchemy binary built and installed to $(INSTALLED_BINARY)"

# Build for a specific target architecture
# Usage: make build-target TARGET=aarch64-unknown-linux-gnu
build-target:
	@if [ -z "$(TARGET)" ]; then \
		echo "❌ TARGET not specified. Usage: make build-target TARGET=<target-triple>"; \
		echo "Available targets:"; \
		echo "  - x86_64-unknown-linux-gnu"; \
		echo "  - aarch64-unknown-linux-gnu"; \
		echo "  - x86_64-apple-darwin"; \
		echo "  - aarch64-apple-darwin"; \
		exit 1; \
	fi
	CFLAGS="-std=gnu17" CXXFLAGS="-std=gnu++17" cargo build --release --target $(TARGET)
	@echo "✅ Built for target: $(TARGET)"
	@echo "Binary location: target/$(TARGET)/release/alchemy$(BINARY_EXT)"

# Check if binary is installed and working
check:
	@if [ -f "$(INSTALLED_BINARY)" ]; then \
		echo "✅ alchemy binary found at $(INSTALLED_BINARY)"; \
		$(INSTALLED_BINARY) --help > /dev/null 2>&1 && echo "✅ alchemy binary is working" || echo "❌ alchemy binary is not working"; \
	else \
		echo "❌ alchemy binary not found. Run 'make install' first."; \
	fi

.PHONY: all install clean build check build-target