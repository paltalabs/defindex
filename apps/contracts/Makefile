SUBDIRS = $(shell find . -maxdepth 1 -mindepth 1 -type d | grep -v node_modules | grep -v target | grep -v dist | grep -v .turbo | grep -v .soroban | grep -v src | xargs echo)
default: build

all: adapters defindex factory

build:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir build || break; \
	done

test: build
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir test || break; \
	done

fmt:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir fmt || break; \
	done

clean:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir clean || break; \
	done