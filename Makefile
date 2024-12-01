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
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} CONFIG_PATH=$(PWD)/configs/d1_e1.yaml $(PWD)/target/release/d1_e1
.PHONY: run-d1-e1

run-d1-e2:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} CONFIG_PATH=$(PWD)/configs/d1_e2.yaml $(PWD)/target/release/d1_e2
.PHONY: run-d1-e2

clean:
	cargo clean
.PHONY: clean

check:
	cargo check
.PHONY: clean
