UNAME := $(shell uname)
ARCH := $(shell uname -m)
PLUGIN_DIR := $(shell pwd)
BIN_DIR := $(PLUGIN_DIR)/bin

ifeq ($(UNAME), Linux)
	OS := linux
else ifeq ($(UNAME), Darwin)
	OS := darwin
else
	$(error Unsupported operating system: $(UNAME))
endif

VERSION := v0.1.0
BINARY_NAME := alchemy-$(OS)-$(ARCH)
DOWNLOAD_URL := https://github.com/Rubenduburck/vim-alchemy/releases/download/$(VERSION)/$(BINARY_NAME)

all: install

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

install: $(BIN_DIR)
	curl -L $(DOWNLOAD_URL) -o $(BIN_DIR)/alchemy
	chmod +x $(BIN_DIR)/alchemy

clean:
	rm -rf $(BIN_DIR)

.PHONY: all install clean
