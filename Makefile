SHELL = /bin/sh

CARGO = cargo
CARGO_OPTS =

.SILENT:

all: build doc

build:
	${CARGO} ${CARGO_OPTS} build

clean:
	${CARGO} ${CARGO_OPTS} clean

check:
	${MAKE} build
	${MAKE} test

test:
	${CARGO} ${CARGO_OPTS} test

bench:
	${CARGO} ${CARGO_OPTS} bench

doc:
	${CARGO} ${CARGO_OPTS} doc
#	${MAKE} doc_gen_html
#	${MAKE} doc_gen_info

doc_gen_html:
	cd doc/sphinx && make html

doc_gen_info:
	cd doc/sphinx && make info

.PHONY: all build clean check test bench doc doc_info

