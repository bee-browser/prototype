export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)
export BEE_CARGO_CODEGEN_DIR := $(PROJDIR)/target/codegen

# These are used in the "lldb.launch.sourceMap" property in //.vscode/settings.json.
export BEE_DEV_RUSTC_COMMIT_HASH := $(shell rustc -vV | grep 'commit-hash' | cut -d ' ' -f 2)
export BEE_DEV_RUST_TOOLCHAIN_PATH := $(shell rustup toolchain list -v | grep '(default)' | cut -f 2)

# NOTES
# -----
# In this project, commands in //tools/bin are used in build scripts.  For example, bee-codegen is
# used for generating source files.  So, we need to add the path to the PATH enviroment before
# building.
#
# The `env` property in //.vscode/launch.json doesn't work for this purpose as you expected.  See:
# https://github.com/vadimcn/vscode-lldb/blob/v1.6.0/extension/cargo.ts#L204
#
# The `lldb.adapterEnv` property works, but it doesn't support the variable substitusion like
# below:
#
#   "lldb.adapterEnv": {
#     "PATH": "${workspaceFolder}/tools/bin:${env:PATH}"
#   }
#
# We've solved this issue by exporting enviroments before launching VSCode as you can see in the
# `launch-vscode` task below.
#BEE_DEV_CONTAINER_RUSTC_COMMIT_HASH=$(docker run --rm mcr.microsoft.com/vscode/devcontainers/rust rustc -vV | grep 'commit-hash' | cut -d ' ' -f 2)
#BEE_DEV_CONTAINER_RUST_TOOLCHAIN_PATH=$(docker run --rm mcr.microsoft.com/vscode/devcontainers/rust rustup toolchain list -v | grep '(default)' | cut -f 2)

BUILD_TARGETS = $(addprefix build-,\
  webui \
)

CLEAN_TARGETS = $(addprefix clean-,\
  webui \
)

TESTGEN_TARGETS = $(addprefix testgen-,\
  libs/layout \
)

COVERAGE_TEST_ENV_VARS = \
  CARGO_INCREMENTAL='0' \
  RUSTFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests' \
  RUSTDOCFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests'

GRCOV_COMMON_ARGS = \
  --branch --llvm --ignore-not-existing -s $(PROJDIR) \
  --ignore '*/src/main.rs' \
  --excl-line '<coverage:exclude/>|unimplemented!|unreachable!' \
  --excl-start '<coverage:exclude>' \
  --excl-stop '</coverage:exclude>' \
  $(PROJDIR)/target/debug

.PHONY: all
all: build

.PHONY: build
build: $(BUILD_TARGETS)
	cargo build --release

.PHONY: test
test: testgen
	cargo test --release --all-features

.PHONY: clean
clean: $(CLEAN_TARGETS)
	cargo clean

.PHONY: debug-build
debug-build: $(BUILD_TARGETS)
	cargo build

.PHONY: debug-test
debug-test: testgen
	cargo test --all-features

.PHONY: coverage-test
coverage: testgen
	env $(COVERAGE_TEST_ENV_VARS) cargo +nightly test --all-features --no-fail-fast

.PHONY: coverage-lcov
coverage-lcov: coverage-test install-grcov
	grcov -t lcov -o $(PROJDIR)/target/coverage/lcov.info $(GRCOV_COMMON_ARGS)

.PHONY: coverage-html
coverage-html: coverage-test install-grcov
	grcov -t html -o $(PROJDIR)/target/coverage $(GRCOV_COMMON_ARGS)

.PHONE: testgen
testgen: $(TESTGEN_TARGETS)

.PHONY: devenv
devenv:
	@make -C tools prepare
	@make -C webui prepare

.PHONY: install-grcov
install-grcov:
	cargo install grcov

.PHONY: github-ci
github-ci: devenv
	@echo "$GITHUB_WORKSPACE/tools/bin" >>$GITHUB_PATH
	@echo "$GITHUB_WORKSPACE/tools/node_modules/.bin" >>$GITHUB_PATH

.PHONY: github-workflows
github-workflows:
	@sh .github/workflows/update.sh

.PHONY: $(BUILD_TARGETS)
$(BUILD_TARGETS):
	@make -C $(subst build-,,$@) build

.PHONY: $(TESTGEN_TARGETS)
$(TESTGEN_TARGETS):
	@make -C $(subst testgen-,,$@) testgen

.PHONY: $(CLEAN_TARGETS)
$(CLEAN_TARGETS):
	@make -C $(subst clean-,,$@) clean
