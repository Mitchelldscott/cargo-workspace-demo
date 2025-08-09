#include <stdint.h>

extern uint32_t _estack;  // symbol from linker script (end of RAM)

void Reset_Handler(void);
void Default_Handler(void);

__attribute__((section(".isr_vector")))
uint32_t vector_table[] = {
    (uint32_t) &_estack,   // Initial stack pointer value (address)
    (uint32_t) Reset_Handler,
    (uint32_t) Default_Handler,
    (uint32_t) Default_Handler,
    // ... add other handlers as needed
};

