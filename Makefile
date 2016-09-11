SHELL = /bin/sh

CARGO = cargo
CARGO_OPTS =

BOOTSTRAP = script/bootstrap
BOOTSTRAP_OPTS =

CIBUILD = script/cibuild
CIBUILD_OPTS =

AFTER_CIBUILD = script/after_cibuild
AFTER_CIBUILD_OPTS =

after_cibuild:
	${AFTER_CIBULD} ${AFTER_CIBULD_OPTS}

all:
	${MAKE} build
	${MAKE} doc
	${MAKE} doc_info

build:
	${CARGO} ${CARGO_OPTS} build

bootstrap:
	${BOOTSTRAP} ${BOOTSTRAP_OPTS}

cibuild:
	${CIBUILD} ${CIBUILD_OPTS}

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
	${CARGO}${CARGO_OPTS} doc

doc_info:
	cd docs/; \
	make info

.PHONY: all build clean check test bench doc doc_info

