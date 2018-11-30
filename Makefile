PROJECT_NAME=$(notdir $(CURDIR))
BUILDNO=$(shell date "+%Y%m%d.%H%M%S")

.PHONY:help
help:
	@echo "help              disply this help"
	@echo "test              run cargo test"
	@echo "build             build production release"

.PHONY: build
build: test
	@mkdir -p build
	@cargo build --release
	@cp $(CARGO_TARGET_DIR)/release/$(PROJECT_NAME) build/$(PROJECT_NAME)
	@cp $(CARGO_TARGET_DIR)/release/$(PROJECT_NAME) build/$(PROJECT_NAME)_v$(BUILDNO)
	strip build/$(PROJECT_NAME)
	@echo -e "\nremember copy deploy/.env to build/ and config\n"

.PHONY: test
test:
	cargo test
