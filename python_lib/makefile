MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
SHELL := /usr/bin/dash

rust_source := $(shell find rust/src -type f -name "*") \
							 rust/Cargo.toml rust/Cargo.lock
python_source := $(shell find python/pauli_tracker -type f -name "*.py" -o -name "*.pyi")
source := $(rust_source) $(python_source)

package: dist/.stamp

dist/.stamp: target/.image.stamp pyproject.toml $(source) | dist/ target/
	sudo chown -R "0:0" dist
	sudo chown -R "0:0" target
	sudo docker run --network=host \
		--mount=type=bind,source="$$(pwd)",target=/app manylinux:latest
	sudo chown -R "$$(id -u):$$(id -g)" dist
	sudo chown -R "$$(id -u):$$(id -g)" target
	touch $@

target/.image.stamp: Dockerfile | target/
	sudo docker build --network=host -t manylinux .
	touch $@

update_docs: dist/
	maturin develop
	(cd docs; rm -rf _build _autosummary; make html)
	cp -R docs/_build/html/. dist/docs
	rm -r dist/docs/_sources
	# touch dist/docs/.nojekyll # needed when using default pages deployment

clean:
	rm target/.image.stamp dist -rf

%/:
	@mkdir -p $@
