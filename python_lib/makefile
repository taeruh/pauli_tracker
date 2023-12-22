MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
SHELL := /usr/bin/dash

rust_source := $(shell find rust/src -type f -name "*") \
							 rust/Cargo.toml rust/Cargo.lock
python_source := $(shell find python/src -type f -name "*.py" -o -name "*.pyi")
source := $(rust_source) $(python_source)

all: dist/.stamp

dist/.stamp: target/.image.stamp pyproject.toml $(source) | target/
	sudo chown -R "0:0" target
	sudo docker run --network=host \
		--mount=type=bind,source="$$(pwd)",target=/app manylinux:latest
	sudo chown -R "$$(id -u):$$(id -g)" target dist
	touch $@

target/.image.stamp: Dockerfile | target/
	sudo docker build --network=host -t manylinux .
	touch $@

clean:
	rm target/.image.stamp dist -rf

%/:
	@mkdir -p $@
