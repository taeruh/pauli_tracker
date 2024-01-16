from quay.io/pypa/manylinux_2_28_x86_64

run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
run source $HOME/.cargo/env
env PATH="/root/.cargo/bin:${PATH}"
run rustup default stable
run /opt/python/cp311-cp311/bin/pip install maturin

workdir /app

cmd [ \
  "bash", "-c", \
  "/opt/python/cp311-cp311/bin/maturin build \
    # I don't get it: --features abi3 does not work (really strange error about a missing
    # static library; changing the feature name or not enabling the pyo3 feature with it
    # changes nothing), however, --all-features works fine -> okay for now, since there's
    # only one feature, but maybe open an issue about this
    # --features abi3 \
    --all-features \
    --interpreter /opt/python/cp311-cp311/bin/python \
    --compatibility manylinux_2_28_x86_64 \
    --release \
    --out dist \
    --sdist \
  " \
]

# run for py in /opt/python/cp3{8,9,10,11}*/bin; do "${py}/pip" install maturin; done
# cmd [ \
#   "bash", "-c", \
#   "for py in /opt/python/cp3{8,9,10,11}*/bin; do ${py}/maturin build \
#     --interpreter ${py}/python \
#     --compatibility manylinux_2_28_x86_64 \
#     --release \
#     --sdist \
#   ; done" \
# ]
