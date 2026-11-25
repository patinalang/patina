ifneq ($(filter install,$(MAKECMDGOALS)),)
    ifndef INSTALL_PREFIX
        $(error INSTALL_PREFIX is required)
    endif
    PROFILE ?= release
else
    PROFILE ?= debug
endif

ifneq ($(wildcard .git),)
    GIT_COMMIT = $(eval GIT_COMMIT := $(shell git --git-dir=.git rev-parse --verify HEAD))$(GIT_COMMIT)
    export CFG_VERSION ?= nightly ($(or $(GIT_COMMIT),$(error Could not resolve Git commit SHA, set CFG_VERSION manually)))
endif

CARGO_FLAGS := $(if $(filter release,$(PROFILE)),--release)

PATINA := target/$(PROFILE)/patina
ROOT := target/$(PROFILE)/root

APPS := patina_core
CORE_SRC := lib/core
CORE := $(ROOT)/patina_core
CORE_EBIN := $(CORE)/ebin
CORE_APP := $(CORE_EBIN)/patina_core.app

CORE_ERL := $(wildcard $(CORE_SRC)/erl_src/*.erl)
CORE_APP_SRC := $(CORE_SRC)/erl_src/patina_core.app.src
PATINA_VSN = $(shell cargo pkgid | cut -d'\#' -f2 | cut -d@ -f2)

TOOLS_TARGET := target/tools
VENDOR := target/vendor
COPYRIGHT_HTML := target/COPYRIGHT.html
LICENSE_METADATA := license-metadata.json
RUST_SNAPSHOT_COMMIT := 5f0f034d2db380f5bb135759a75170009591e71f
GENERATE_COPYRIGHT := $(TOOLS_TARGET)/debug/generate-copyright

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
		doc { printf "  %-24s%s\n", $$1, doc; doc = "" }' $(firstword $(MAKEFILE_LIST))

.PHONY: build
## Build the compiler and its libraries.
build: $(CORE_APP)

.PHONY: install
## Build and install into INSTALL_PREFIX.
install: build $(COPYRIGHT_HTML)
	mkdir -p "$(INSTALL_PREFIX)"
	cd "$(INSTALL_PREFIX)" && rm -rf $(APPS)
	cp -R $(addprefix $(ROOT)/,$(APPS)) "$(INSTALL_PREFIX)"
	mkdir -p "$(INSTALL_PREFIX)/patina/bin"
	cp $(PATINA) "$(INSTALL_PREFIX)/patina/bin/patina"
	cp $(COPYRIGHT_HTML) LICENSE-APACHE LICENSE-MIT "$(INSTALL_PREFIX)/patina"
	rm -rf "$(INSTALL_PREFIX)/patina/LICENSES"
	cp -R LICENSES "$(INSTALL_PREFIX)/patina"

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

.PHONY: license-metadata
## Regenerate license-metadata.json from REUSE.toml.
license-metadata: $(LICENSE_METADATA)

.PHONY: check-license-metadata
## Check that license-metadata.json matches REUSE.toml.
check-license-metadata: export ONLY_CHECK := 1
check-license-metadata: $(LICENSE_METADATA)

.PHONY: copyright
## Generate target/COPYRIGHT.html.
copyright: $(COPYRIGHT_HTML)

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

$(LICENSE_METADATA): $(if $(filter %license-metadata,$(MAKECMDGOALS)),FORCE)
	RUST_SNAPSHOT_COMMIT=$(RUST_SNAPSHOT_COMMIT) DEST="$(abspath $@)" \
		cargo run --manifest-path rust/src/tools/collect-license-metadata/Cargo.toml \
		--target-dir $(TOOLS_TARGET)

$(GENERATE_COPYRIGHT): FORCE
	cargo build --manifest-path rust/src/tools/generate-copyright/Cargo.toml \
		--target-dir $(TOOLS_TARGET)

$(COPYRIGHT_HTML): $(GENERATE_COPYRIGHT) Cargo.lock $(LICENSE_METADATA)
	cargo vendor --locked --versioned-dirs $(VENDOR) > /dev/null
	CARGO="$(shell which cargo)" CARGO_HOME="$${CARGO_HOME:-$$HOME/.cargo}" \
		SRC_DIR="$(abspath .)" VENDOR_DIR="$(abspath $(VENDOR))" \
		CARGO_MANIFESTS="$(abspath Cargo.toml)" \
		LICENSE_METADATA="$(abspath $(LICENSE_METADATA))" \
		DEST="$(abspath $@)" \
		DEST_LIBSTD="$(abspath $(TOOLS_TARGET)/COPYRIGHT-library.html)" \
		$(GENERATE_COPYRIGHT)

$(CORE_APP): $(PATINA) $(CORE_SRC)/Patina.toml $(wildcard $(CORE_SRC)/src/*) $(CORE_ERL) $(CORE_APP_SRC)
	rm -rf $(CORE)
	mkdir -p $(CORE_EBIN)
	cp -R $(CORE_SRC)/Patina.toml $(CORE_SRC)/src $(CORE_SRC)/erl_src $(CORE)
	sed -e 's/{vsn, "[^"]*"}/{vsn, "$(PATINA_VSN)"}/' $(CORE_APP_SRC) \
		> $(CORE)/erl_src/patina_core.app.src
	erlc -o $(CORE_EBIN) $(CORE_ERL)
	cd $(CORE_SRC) && PATINA_BOOTSTRAP=1  $(abspath $(PATINA)) --root=$(abspath $(ROOT)) \
		build --out-dir $(abspath $(CORE_EBIN))
	rm -rf $(CORE_EBIN)/.patina_incremental
