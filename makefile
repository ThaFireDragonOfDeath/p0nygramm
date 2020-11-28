# Export all variables by default
export

# Programs
CARGO := cargo
CP := cp -f
INSTALL := install
MKPATH := mkdir -p
RM := rm -f
RM_RECURSIVE := rm -rf

# Directory of this makefile (ends without '/')
MAKEFILE_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# Project name
PROJECT_NAME := p0nygramm

# Dir names (target)
INSTALL_DIR := /srv
CONFIG_DIR_NAME := config
STATIC_WEBCONTENT_DIR_NAME := webcontent
TEMPLATES_DIR_NAME := templates
UPLOADS_DIR_NAME := uploads
UPLOADS_PRV_DIR_NAME := uploads-prv

# Path variables (source)
SRC_CONFIG_PATH := $(MAKEFILE_DIR)/resources/config
SRC_STATIC_WEBCONTENT_PATH := $(MAKEFILE_DIR)/resources/static-webcontent
SRC_TEMPLATES_PATH := $(MAKEFILE_DIR)/resources/templates

# Path variables (install target)
PROJECT_PATH := $(INSTALL_DIR)/$(PROJECT_NAME)
CONFIG_PATH := $(PROJECT_PATH)/$(CONFIG_DIR_NAME)
STATIC_WEBCONTENT_PATH := $(PROJECT_PATH)/static/$(STATIC_WEBCONTENT_DIR_NAME)
TEMPLATES_PATH := $(PROJECT_PATH)/static/$(TEMPLATES_DIR_NAME)
UPLOADS_PATH := $(PROJECT_PATH)/$(UPLOADS_DIR_NAME)
UPLOADS_PRV_PATH := $(PROJECT_PATH)/$(UPLOADS_PRV_DIR_NAME)
CARGO_TARGET_OUT_DIR := $(MAKEFILE_DIR)/target

# File names (source)
CONFIG_FILES := system-config.toml
STATIC_WEBCONTENT_FILES := p0nygramm.css p0nygramm.js p0nygramm_api.js p0nygramm_ui.js
TEMPLATE_FILES := index.html

# Build options
BUILDMODE := debug
CARGOFLAGS :=

ifeq ($(BUILDMODE),release)
CARGOFLAGS := --release
endif

# Path variables for the resulting output executable
BIN_OUTPUT_DIR := $(CARGO_TARGET_OUT_DIR)/$(BUILDMODE)
BIN_OUTPUT_FILE := $(BIN_OUTPUT_DIR)/$(PROJECT_NAME)

# Add user provided cargo flags
ifneq ($(EXTRA_CARGOFLAGS),)
CARGOFLAGS += $(EXTRA_CARGOFLAGS)
endif

# Main build targets
.PHONY: all
all: light-clean $(PROJECT_NAME)

$(PROJECT_NAME):
	cd $(MAKEFILE_DIR)
	$(CARGO) build $(CARGOFLAGS)
	$(CP) $(BIN_OUTPUT_FILE) $(MAKEFILE_DIR)/$(PROJECT_NAME)

# Main install targets
.PHONY: install
install: $(PROJECT_NAME) install-resources
	$(INSTALL) $(PROJECT_NAME) $(PROJECT_PATH)

.PHONY: install-resources
install-resources: create-dir-structure install-config-files install-static-webcontent install-template-files

# Main uninstall targets
# Uninstall server binary but keep the project data
.PHONY: uninstall
uninstall:
	$(RM) $(PROJECT_PATH)/$(PROJECT_NAME)

# Uninstall project including the config and upload files
.PHONY: full-uninstall
full-uninstall:
	$(RM_RECURSIVE) $(PROJECT_PATH)

# Main clean targets
.PHONY: clean
clean: light-clean
	$(RM_RECURSIVE) CARGO_TARGET_OUT_DIR

# Only remove the exec binary from the makefile directory (don't touch the target directory)
.PHONY: light-clean
light-clean:
	$(RM) $(PROJECT_NAME)

# Helper targets
.PHONY: create-dir-structure
create-dir-structure:
	$(MKPATH) $(PROJECT_PATH)
	$(MKPATH) $(CONFIG_PATH)
	$(MKPATH) $(STATIC_WEBCONTENT_PATH)
	$(MKPATH) $(TEMPLATES_PATH)
	$(MKPATH) $(UPLOADS_PATH)
	$(MKPATH) $(UPLOADS_PRV_PATH)

.PHONY: install-config-files
install-config-files:
	$(MAKE) -C $(SRC_CONFIG_PATH) install

.PHONY: install-static-webcontent
install-static-webcontent:
	$(MAKE) -C $(SRC_STATIC_WEBCONTENT_PATH) install

.PHONY: install-template-files
install-template-files:
	$(MAKE) -C $(SRC_TEMPLATES_PATH) install