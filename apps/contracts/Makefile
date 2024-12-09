SUBDIRS = strategies vault factory integration-test

default: build

all: build

build:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir build || break; \
	done

test: build
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir test || break; \
	done

fmt:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir fmt || break; \
	done

clean:
	@for dir in $(SUBDIRS); do \
		$(MAKE) -C $$dir clean || break; \
	done