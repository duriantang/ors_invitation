PROJECT_NAME=$(notdir $(CURDIR))
BUILDNO=$(shell date "+%Y%m%d.%H%M%S")

.PHONY:help
help:
	@echo "help              disply this help"
	@echo "build             build production release"

.PHONY: build
build:
	@mkdir -p build
	@cargo build --release
	@cp target/release/$(PROJECT_NAME) build/$(PROJECT_NAME)
	@cp target/release/$(PROJECT_NAME) build/$(PROJECT_NAME)_v$(BUILDNO)
	strip build/$(PROJECT_NAME)
	@echo -e "\nremember copy deploy/.env to build/ and config\n"
