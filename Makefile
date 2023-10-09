.ONESHELL: # Applies to every targets in the file!

CURRENT_DIR := $(dir $(abspath $(firstword $(MAKEFILE_LIST))))

build:
	$(MAKE) -C web_server/react-app build

run-intermediary:
	$(MAKE) -C web_server run

run-gpjc-api:
	$(MAKE) -C gpjc-api run

run: run-intermediary run-gpjc-api
