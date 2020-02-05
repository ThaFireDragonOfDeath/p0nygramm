# Directory of this makefile (ends without '/')
MAKEFILE_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# Project properties
REQ_DB_SCHEMA_VERSION := 1

# User setable variables
PROJECT_NAME := p0nygramm
INSTALL_DIR := /srv
CONFIG_DIR_NAME := config
STATIC_WEBCONTENT_DIR_NAME := webcontent
UPLOADS_DIR_NAME := uploads
BUILDMODE := debug

PROJECT_PATH := $(INSTALL_DIR)/$(PROJECT_NAME)
CONFIG_PATH := $(PROJECT_PATH)/$(CONFIG_DIR_NAME)
STATIC_WEBCONTENT_PATH := $(PROJECT_PATH)/$(STATIC_WEBCONTENT_DIR_NAME)
UPLOADS_PATH := $(PROJECT_PATH)/$(UPLOADS_DIR_NAME)

.PHONY: create-dir-structure
create-dir-structure:
	mkdir -p $(PROJECT_PATH)
	mkdir -p $(CONFIG_PATH)

.PHONY: parse-template
parse-template:
	$(MAKE) -C $(MAKEFILE_DIR)/ressources/config
