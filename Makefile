SHELL := /bin/bash
VERSION ?= $(shell git describe --tags --dirty --always --abbrev=12)
PWD := $(shell pwd)

RUST_LOG ?= info
default: build

all:

build:
	cargo build --release
.PHONY: build

build-debug:
	cargo build
.PHONY: build

run-d1-e1:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d1_e1
.PHONY: run-d1-e1

run-d1-e2:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d1_e2
.PHONY: run-d1-e2

run-d2-e1:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d2_e1
.PHONY: run-d2-e1

run-d2-e2:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d2_e2
.PHONY: run-d2-e2

run-d3-e1:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d3_e1
.PHONY: run-d3-e1

run-d3-e2:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d3_e2
.PHONY: run-d3-e2

run-d4-e1:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d4_e1
.PHONY: run-d4-e1

run-d4-e2:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d4_e2
.PHONY: run-d4-e2

run-d5-e1:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d5_e1
.PHONY: run-d5-e1

run-d5-e2:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d5_e2
.PHONY: run-d5-e2

clean:
	cargo clean
.PHONY: clean

check:
	cargo check
.PHONY: clean
