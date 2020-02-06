# Programms
CARGO := cargo
CP := cp -f

# Directory of this makefile (ends without '/')
MAKEFILE_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# Project properties
REQ_DB_SCHEMA_VERSION := 1

# User setable variables
PROJECT_NAME := p0nygramm
INSTALL_DIR := /srv
CONFIG_DIR_NAME := config
STATIC_WEBCONTENT_DIR_NAME := webcontent
TEMPLATES_DIR_NAME := template
UPLOADS_DIR_NAME := uploads
BUILDMODE := debug

# Path variables
PROJECT_PATH := $(INSTALL_DIR)/$(PROJECT_NAME)
CONFIG_PATH := $(PROJECT_PATH)/$(CONFIG_DIR_NAME)
STATIC_WEBCONTENT_PATH := $(PROJECT_PATH)/$(STATIC_WEBCONTENT_DIR_NAME)
TEMPLATES_PATH := $(PROJECT_PATH)/$(TEMPLATES_DIR_NAME)
UPLOADS_PATH := $(PROJECT_PATH)/$(UPLOADS_DIR_NAME)

# Build options
CARGOFLAGS := --target-manifest $(MAKEFILE_DIR)

ifeq ($(BUILDMODE),release)
CARGOFLAGS += --release
endif

# Add user provided cargo flags
ifneq ($(EXTRA_CARGOFLAGS),)
CARGOFLAGS += $(EXTRA_CARGOFLAGS)
endif

$(PROJECT_NAME):
	$(CARGO) build $(CARGOFLAGS)
	$(CP) $(MAKEFILE_DIR)/target/$(PROJECT_NAME) $(MAKEFILE_DIR)/$(PROJECT_NAME)

.PHONY: create-dir-structure
create-dir-structure:
	mkdir -p $(PROJECT_PATH)
	mkdir -p $(CONFIG_PATH)

.PHONY: parse-templates
parse-templates:
	$(MAKE) CP='$(CP)' -C $(MAKEFILE_DIR)/ressources/config all

.PHONY: clean-templates
clean-templates:
	$(MAKE) -C $(MAKEFILE_DIR)/ressources/config clean