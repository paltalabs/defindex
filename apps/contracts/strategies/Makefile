SUBDIRS = $(shell find . -maxdepth 1 -mindepth 1 -type d | grep -v ./core | grep -v ./external_wasms | xargs echo)
default: build

all: soroswap

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