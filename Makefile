SHELL := /bin/bash
PWD := $(shell pwd)

default: build
LOG_LEVEL ?= info
DAY ?= 9
EJ ?= 2
INPUT ?= "./input/d$(DAY).txt"
INPUT_MINIMAL ?= "./input/d$(DAY)-minimal.txt"

all:

build:
	cargo build --release
.PHONY: build

build-debug:
	cargo build
.PHONY: build

run:
	RUST_BACKTRACE=full LOG_LEVEL=$(LOG_LEVEL) INPUT_PATH=$(INPUT) $(PWD)/target/release/d$(DAY)_e$(EJ)
.PHONY: run

run-minimal:
	RUST_BACKTRACE=full LOG_LEVEL=$(LOG_LEVEL) INPUT_PATH=$(INPUT_MINIMAL) $(PWD)/target/release/d$(DAY)_e$(EJ)
.PHONY: run-minimal

clean:
	cargo clean
.PHONY: clean

check:
	cargo check
.PHONY: clean
