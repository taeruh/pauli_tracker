MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
SHELL := /usr/bin/dash

all: target/.stamp

target/.stamp: target/.image.stamp meta.yaml build.sh | target/ dist/
	cp ../dist/pauli_tracker-0.1.0-cp38-abi3-manylinux_2_28_x86_64.whl target
	sudo chown -R "0:0" target dist
	sudo docker run --network=host \
		--mount=type=bind,source="$$(pwd)",target=/app conda_pauli_tracker_build:latest
	sudo chown -R "$$(id -u):$$(id -g)" target dist
	touch $@

target/.image.stamp: Dockerfile | target/
	sudo docker build --network=host -t conda_pauli_tracker_build .
	touch $@

clean:
	rm target dist -rf

%/:
	@mkdir -p $@
