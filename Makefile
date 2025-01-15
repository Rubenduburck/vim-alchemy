UNAME := $(shell uname)
ARCH := $(shell uname -m)
PLUGIN_DIR := $(shell pwd)
BIN_DIR := $(PLUGIN_DIR)/bin

ifeq ($(UNAME), Linux)
	OS := Linux
else ifeq ($(UNAME), Darwin)
	OS := Darwin
else
	$(error Unsupported operating system: $(UNAME))
endif

VERSION := v0.1.0
BINARY_NAME := vim-alchemy-$(OS)-$(ARCH).tar.gz
CHECKSUM_NAME := vim-alchemy-$(OS)-$(ARCH).sha256
DOWNLOAD_URL := https://github.com/Rubenduburck/vim-alchemy/releases/download/$(VERSION)/$(BINARY_NAME)
CHECKSUM_URL := https://github.com/Rubenduburck/vim-alchemy/releases/download/$(VERSION)/$(CHECKSUM_NAME)

all: install

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

install: $(BIN_DIR)
	curl -L $(DOWNLOAD_URL) | tar xz -C $(BIN_DIR)
	curl -L $(CHECKSUM_URL) -o $(BIN_DIR)/$(CHECKSUM_NAME)
	chmod +x $(BIN_DIR)/vim-alchemy

clean:
	rm -rf $(BIN_DIR)

.PHONY: all install clean
