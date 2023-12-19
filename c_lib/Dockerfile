from archlinux:latest

run pacman -Syu --noconfirm rust make

run mkdir -p /home/docker
workdir /home/docker

copy builder/ builder/
copy makefile makefile
copy pauli_tracker_clib/ pauli_tracker_clib/

run make library
