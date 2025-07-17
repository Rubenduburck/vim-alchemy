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
else ifeq ($(UNAME), MINGW64_NT)
	RELEASE_OS := windows
	BINARY_EXT := .exe
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
	cargo build --release
	mkdir -p $(BIN_DIR)
	cp target/release/alchemy$(BINARY_EXT) $(INSTALLED_BINARY)
	@echo "✅ alchemy binary built and installed to $(INSTALLED_BINARY)"

# Check if binary is installed and working
check:
	@if [ -f "$(INSTALLED_BINARY)" ]; then \
		echo "✅ alchemy binary found at $(INSTALLED_BINARY)"; \
		$(INSTALLED_BINARY) --help > /dev/null 2>&1 && echo "✅ alchemy binary is working" || echo "❌ alchemy binary is not working"; \
	else \
		echo "❌ alchemy binary not found. Run 'make install' first."; \
	fi

.PHONY: all install clean build check