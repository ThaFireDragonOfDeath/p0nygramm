# Export all variables by default
export

# Programms
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
TEMPLATES_DIR_NAME := template
UPLOADS_DIR_NAME := uploads
UPLOADS_PRV_DIR_NAME := uploads-prv

# Path variables (source)
SRC_CONFIG_PATH := $(MAKEFILE_DIR)/ressources/config
SRC_STATIC_WEBCONTENT_PATH := $(MAKEFILE_DIR)/ressources/static-webcontent
SRC_TEMPLATES_PATH := $(MAKEFILE_DIR)/ressources/templates

# Path variables (target)
PROJECT_PATH := $(INSTALL_DIR)/$(PROJECT_NAME)
CONFIG_PATH := $(PROJECT_PATH)/$(CONFIG_DIR_NAME)
STATIC_WEBCONTENT_PATH := $(PROJECT_PATH)/$(STATIC_WEBCONTENT_DIR_NAME)
TEMPLATES_PATH := $(PROJECT_PATH)/$(TEMPLATES_DIR_NAME)
UPLOADS_PATH := $(PROJECT_PATH)/$(UPLOADS_DIR_NAME)
UPLOADS_PRV_PATH := $(PROJECT_PATH)/$(UPLOADS_PRV_DIR_NAME)

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

# Add user provided cargo flags
ifneq ($(EXTRA_CARGOFLAGS),)
CARGOFLAGS += $(EXTRA_CARGOFLAGS)
endif

# Main build targets
all: $(PROJECT_NAME)

$(PROJECT_NAME):
	cd $(MAKEFILE_DIR)
	$(CARGO) build $(CARGOFLAGS)
	$(CP) $(MAKEFILE_DIR)/target/$(BUILDMODE)/$(PROJECT_NAME) $(MAKEFILE_DIR)/$(PROJECT_NAME)

install: $(PROJECT_NAME) install-resources

install-resources: create-dir-structure install-config-files install-static-webcontent install-template-files

uninstall:
	$(RM_RECURSIVE) $(PROJECT_PATH)

# Helper build targets
.PHONY: create-dir-structure
create-dir-structure:
	$(MKPATH) $(PROJECT_PATH)
	$(MKPATH) $(CONFIG_PATH)
	$(MKPATH) $(STATIC_WEBCONTENT_PATH)
	$(MKPATH) $(TEMPLATES_PATH)
	$(MKPATH) $(UPLOADS_DIR_NAME)
	$(MKPATH) $(UPLOADS_PRV_DIR_NAME)

.PHONY: install-config-files
install-config-files:
	$(MAKE) -C $(SRC_CONFIG_PATH) install

.PHONY: install-static-webcontent
install-static-webcontent:
	$(MAKE) -C $(SRC_STATIC_WEBCONTENT_PATH) install

.PHONY: install-template-files
install-template-files:
	$(MAKE) -C $(SRC_TEMPLATES_PATH) install
