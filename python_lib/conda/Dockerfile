from continuumio/miniconda3

run \
  conda install conda-build -y; \
  conda-build --version; \
  conda update conda -y; \
  conda update conda-build -y;

workdir /app

cmd [ \
  "bash", "-c", "\
  conda-build . --output-folder dist; \
  "\
]
