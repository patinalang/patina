ifneq ($(filter install,$(MAKECMDGOALS)),)
    ifndef INSTALL_PREFIX
        $(error INSTALL_PREFIX is required)
    endif
    PROFILE ?= release
else
    PROFILE ?= debug
endif

CARGO_FLAGS := $(if $(filter release,$(PROFILE)),--release)

PATINA := target/$(PROFILE)/patina
ROOT := target/$(PROFILE)/root

APPS := patina_core
CORE := $(ROOT)/patina_core
CORE_APP := $(CORE)/ebin/patina_core.app

CORE_ERL := $(wildcard lib/core/erl_src/*.erl)
PATINA_VSN = $(shell cargo pkgid | cut -d'\#' -f2 | cut -d@ -f2)

.DELETE_ON_ERROR:

.PHONY: help
## Print this help message.
help:
	@echo 'Usage: make [TARGET] [PROFILE=debug|release]'
	@echo
	@echo 'PROFILE defaults to `release` if install is a target; `debug` otherwise.'
	@echo
	@echo 'targets:'
	@echo
	@awk -F: '/^## / { doc = substr($$0, 4); next } \
		doc { printf "  %-20s%s\n", $$1, doc; doc = "" }' $(firstword $(MAKEFILE_LIST))

.PHONY: build
## Build the compiler and its libraries.
build: $(CORE_APP)

.PHONY: install
## Build and install into INSTALL_PREFIX.
install: build
	mkdir -p "$(INSTALL_PREFIX)"
	cd "$(INSTALL_PREFIX)" && rm -rf $(APPS)
	cp -R $(addprefix $(ROOT)/,$(APPS)) "$(INSTALL_PREFIX)"
	mkdir -p "$(INSTALL_PREFIX)/patina/bin"
	cp $(PATINA) "$(INSTALL_PREFIX)/patina/bin/patina"

.PHONY: test
## Build and run all tests. Pass `cargo` arguments in ARGS.
test: build
	cargo test $(CARGO_FLAGS) $(ARGS)

.PHONY: compiletest
## Build and run the UI tests.
compiletest: build
	cargo test $(CARGO_FLAGS) --test compiletest $(ARGS)

.PHONY: compiletest-bless
## Build and bless UI test snapshots.
compiletest-bless: build
	cargo test $(CARGO_FLAGS) --test compiletest -- --bless

.PHONY: clean
## Clean up everything.
clean:
	cargo clean

.PHONY: clean-install
## Clean up the install tree.
clean-install:
	rm -rf $(ROOT)

$(PATINA): FORCE
	cargo build $(CARGO_FLAGS)

.PHONY: FORCE
FORCE:

$(CORE_APP): $(PATINA) lib/core/Patina.toml $(wildcard lib/core/src/*) $(wildcard lib/core/erl_src/*)
	rm -rf $(CORE)
	mkdir -p $(CORE)/ebin
	cp -R lib/core/Patina.toml lib/core/src lib/core/erl_src $(CORE)
	sed -e 's/{vsn, "[^"]*"}/{vsn, "$(PATINA_VSN)"}/' lib/core/erl_src/patina_core.app.src \
		> $(CORE)/erl_src/patina_core.app.src
	erlc -o $(CORE)/ebin $(CORE_ERL)
	cd lib/core && PATINA_BOOTSTRAP=1  $(abspath $(PATINA)) --root=$(abspath $(ROOT)) \
		build --out-dir $(abspath $(CORE)/ebin)
	rm -rf $(CORE)/ebin/.patina_incremental
