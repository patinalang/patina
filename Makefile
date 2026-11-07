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

APPS := patina patina_core
PATINA_APP := $(ROOT)/patina/ebin/patina.app
CORE_APP := $(ROOT)/patina_core/ebin/patina_core.app

PATINA_ERL := $(wildcard lib/patina/src/*.erl)
PATINA_VSN = $(shell cargo pkgid | cut -d'\#' -f2 | cut -d@ -f2)
PATINA_MODULES = $(shell echo $(basename $(notdir $(PATINA_ERL))) | tr ' ' ,)

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
build: $(PATINA_APP) $(CORE_APP)

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

$(PATINA_APP): $(PATINA_ERL) lib/patina/src/patina.app.src
	rm -rf $(dir $@) $(ROOT)/patina/src
	mkdir -p $(dir $@) $(ROOT)/patina/src
	cp $(PATINA_ERL) lib/patina/src/patina.app.src $(ROOT)/patina/src
	erlc -o $(dir $@) $(PATINA_ERL)
	sed -e 's/vsn, "/vsn, "$(PATINA_VSN)/' -e 's/modules, \[/modules, [$(PATINA_MODULES)/' \
		lib/patina/src/patina.app.src > $@

$(CORE_APP): $(PATINA) $(PATINA_APP) lib/core/Patina.toml $(wildcard lib/core/src/*.pt)
	rm -rf $(ROOT)/patina_core
	mkdir -p $(ROOT)/patina_core
	cp -R lib/core/Patina.toml lib/core/src $(ROOT)/patina_core
	cd lib/core && PATINA_BOOTSTRAP=1  $(abspath $(PATINA)) --root=$(abspath $(ROOT)) \
		build --out-dir $(abspath $(dir $@))
