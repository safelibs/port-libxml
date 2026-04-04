PYTHON ?= python3
ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../..)
STAGE ?= $(ROOT)/safe/target/stage
TRIPLET ?= $(shell gcc -print-multiarch)
BUILD_ROOT := $(ROOT)/safe/target/python-build
SITE_PACKAGES := $(STAGE)/usr/lib/python3/dist-packages

.PHONY: build install clean

build:
	rm -rf "$(BUILD_ROOT)"
	mkdir -p "$(BUILD_ROOT)/lib" "$(BUILD_ROOT)/temp"
	cd "$(ROOT)/safe/python" && \
		LIBXML2_STAGE="$(STAGE)" LIBXML2_TRIPLET="$(TRIPLET)" \
		"$(PYTHON)" setup.py build_ext \
			--build-temp "$(BUILD_ROOT)/temp" \
			--build-lib "$(BUILD_ROOT)/lib"

install: build
	mkdir -p "$(SITE_PACKAGES)"
	install -m 0644 "$(ROOT)/safe/python/libxml2.py" "$(SITE_PACKAGES)/libxml2.py"
	install -m 0644 "$(ROOT)/safe/python/drv_libxml2.py" "$(SITE_PACKAGES)/drv_libxml2.py"
	install -m 0755 "$(BUILD_ROOT)"/lib/libxml2mod*.so "$(SITE_PACKAGES)/"

clean:
	rm -rf "$(BUILD_ROOT)"
