# Bare-metal C Demo

## References

* [Bare-Metal guide](https://github.com/cpq/bare-metal-programming-guide)
* [ARM GCC Options](https://gcc.gnu.org/onlinedocs/gcc/ARM-Options.html)
* [RISC-V GCC Options](https://gcc.gnu.org/onlinedocs/gcc/RISC-V-Options.html)

## Build

```bash
arm-none-eabi-gcc -nostdlib -mcpu=cortex-m7 -mfloat-abi=hard -mno-unaligned-access -std=c11 -Wall -Wextra -O3 main.c -o main
```