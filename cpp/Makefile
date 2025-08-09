CC = arm-none-eabi-gcc
OBJDUMP = arm-none-eabi-objdump
CFLAGS = -nostdlib -mcpu=cortex-m7 -mfloat-abi=hard -mno-unaligned-access -std=c11 -Wall -Wextra -O3

SRC = main.c
OUT = main
DISASM = main.s

all: $(OUT)

$(OUT): $(SRC)
	$(CC) $(CFLAGS) $^ -o $@

disasm: $(OUT)
	$(OBJDUMP) -d ./$(OUT) > $(DISASM)

clean:
	rm -f $(OUT)
	rm -f $(OUT).s
	rm -f $(OUT).txt