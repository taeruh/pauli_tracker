MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
SHELL := /usr/bin/dash

all: target/wheels/stamp

target/wheels/stamp: target/image.stamp Cargo.toml pyproject.toml src | target/wheels/
	sudo chown -R "0:0" target
	sudo docker run --network=host \
		--mount=type=bind,source="$$(pwd)",target=/app manylinux:latest
	sudo chown -R "$$(id -u):$$(id -g)" target
	touch $@

target/image.stamp: Dockerfile | target/
	sudo docker build --network=host -t manylinux .
	touch $@

clean:
	rm target/image.stamp target/wheels -rf

%/:
	@mkdir -p $@
