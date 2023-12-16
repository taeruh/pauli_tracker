from quay.io/pypa/manylinux_2_28_x86_64

run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
run source $HOME/.cargo/env
env PATH="/root/.cargo/bin:${PATH}"
run rustup default stable
run for py in /opt/python/cp3{10,11}*/bin; do "${py}/pip" install maturin; done

workdir /app
cmd [ \
  "bash", "-c", \
  "for py in /opt/python/cp3{10,11}*/bin; do ${py}/maturin build -i ${py}/python; done" \
]
