FROM ubuntu:24.04

# Metadata
LABEL maintainer="scott.mitchell913@gmail.com"
LABEL description="Docker image for cross-building C code for ARM Cortex-M targets"

# Avoid interactive apt prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install base build tools, QEMU, and dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    gdb-multiarch \
    ca-certificates \
    qemu-system-arm \
    qemu-system-misc \
    qemu-system-riscv64 \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable

# Make cargo/rustup available in PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rust targets for bare-metal/QEMU
RUN rustup target add \
    riscv64gc-unknown-none-elf \
    thumbv7em-none-eabihf

RUN rustup component add llvm-tools

# Install useful Cargo tools
RUN cargo install \
    cargo-binutils

# Install GNU Arm Embedded Toolchain (latest official release)
RUN apt update && apt install -y gcc-arm-none-eabi gdb-arm-none-eabi binutils-arm-none-eabi

# Create a working directory
WORKDIR /home/dev

# Set entrypoint (optional)
CMD ["/bin/bash"]
