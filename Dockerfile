# Build Stage
FROM --platform=linux/amd64 ubuntu:20.04 as builder

## Install build dependencies.
# Update default packages
RUN apt-get update -y

# Get Ubuntu packages
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y build-essential curl cmake clang git python

ENV CC=clang 
ENV CXX=clang++

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

## Add source code to the build stage.
ADD . /parity-wasm
WORKDIR /parity-wasm

RUN git submodule update --init --recursive

# Configure Rust and build fuzz file
RUN cargo install cargo-fuzz
RUN rustup override set nightly
RUN cargo fuzz build deserialize

# Package Stage
FROM --platform=linux/amd64 ubuntu:20.04
COPY --from=builder /parity-wasm/ /