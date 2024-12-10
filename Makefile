SHELL := /bin/bash
PWD := $(shell pwd)

RUST_LOG ?= info
default: build
DAY ?= 7
EJ ?= 2

all:

build:
	cargo build --release
.PHONY: build

build-debug:
	cargo build
.PHONY: build

run:
	RUST_BACKTRACE=full RUST_LOG=${RUST_LOG} $(PWD)/target/release/d$(DAY)_e$(EJ)
.PHONY: run

clean:
	cargo clean
.PHONY: clean

check:
	cargo check
.PHONY: clean
