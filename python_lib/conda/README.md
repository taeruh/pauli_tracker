# Building a Conda package from the wheel

I'm not going to install Conda, so we do it in a docker:

Run the `makefile`.

The package is `dist/linux-64/pauli_tracker-0.1.0-py311_0.tar.bz2`.

Test it by doing something like
```bash
sudo docker run --network=host \
  --mount=type=bind,source=$(pwd),target=/app conda_pauli_tracker_build:latest
conda install --use-local dist/linux-64/pauli_tracker-0.1.0-py311_0.tar.bz2
python
>>> from pauli_tracker.live.vec import Live
>>> tracker = Live(2)
>>> tracker.track_y(0)
>>> tracker.cx(0, 1)
>>> print(tracker.into_py_array_recursive())
```
