FROM ubuntu:bionic as builder

RUN apt-get update && apt-get install -yq software-properties-common && \
	add-apt-repository ppa:ocl-dev/intel-opencl && \
	apt-get update

RUN apt-get install -yq intel-opencl \
	build-essential \
	gfortran \
	cmake \
	clang \
	llvm \
	git \
	libblas-dev \
	ocl-icd-opencl-dev

RUN git clone https://github.com/clMathLibraries/clBLAS && cd clBLAS && mkdir build && cd build && \
	cmake ../src/

RUN cd clBLAS/build && make -j10 && make install

RUN apt-get install -yq cargo \
		 && \
	rm -rf /var/lib/apt/lists/*
