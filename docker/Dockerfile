FROM ubuntu:24.04

# Metadata
LABEL maintainer="scott.mitchell913@gmail.com"
LABEL description="Docker image for cross-building C code for ARM Cortex-M targets"

# Update base packages and install dependencies
RUN apt update && apt install -y \
    build-essential \
    gcc \
    make \
    cmake \
    ninja-build \
    git \
    curl \
    wget \
    python3 \
    python3-pip \
    libusb-1.0-0-dev \
    libtool \
    pkg-config \
    gdb-multiarch \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install GNU Arm Embedded Toolchain (latest official release)
RUN apt update && apt install -y gcc-arm-none-eabi gdb-arm-none-eabi binutils-arm-none-eabi

# Optional: Install OpenOCD for flashing/debugging
RUN apt update && apt install -y openocd

# Create a working directory
WORKDIR /home/dev

# Set environment variables for convenience
ENV CC=arm-none-eabi-gcc
ENV OBJDUMP=arm-none-eabi-objdump
ENV CFLAGS="-mcpu=cortex-m4 -mthumb -O3"

# Set entrypoint (optional)
CMD ["/bin/bash"]
