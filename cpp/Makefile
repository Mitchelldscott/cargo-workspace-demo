CC = arm-none-eabi-gcc
OBJDUMP = arm-none-eabi-objdump
CFLAGS = -nostdlib -mcpu=cortex-m4 -mfloat-abi=hard -mthumb -mfpu=fpv4-sp-d16 -mno-unaligned-access -std=c11 -Wall -Wextra -O3 -T linker.ld

SRC = startup.c main.c
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