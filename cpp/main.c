#include <stdint.h>

typedef float float32_t;

// https://developer.arm.com/documentation/102467/0201/Example---matrix-multiplication
void matrix_multiply_c(float32_t *A, float32_t *B, float32_t *C, uint32_t n, uint32_t m, uint32_t k) {
    for (uint32_t i_idx=0; i_idx < n; i_idx++) {
        for (uint32_t j_idx=0; j_idx < m; j_idx++) {
            C[n*j_idx + i_idx] = 0;
            for (uint32_t k_idx=0; k_idx < k; k_idx++) {
                C[n*j_idx + i_idx] += A[n*k_idx + i_idx]*B[k*j_idx + k_idx];
            }
        }
    }
}

int main(void) {
    const uint32_t n = 3;  // Rows of A and C
    const uint32_t m = 2;  // Columns of B and C
    const uint32_t k = 4;  // Columns of A and Rows of B

    // Declare and initialize matrices A and B in column-major order
    // A is 3x4
    float32_t A[12] = {
        1.0f, 2.0f, 3.0f,   // Col 0
        4.0f, 5.0f, 6.0f,   // Col 1
        7.0f, 8.0f, 9.0f,   // Col 2
        10.0f, 11.0f, 12.0f // Col 3
    };

    // B is 4x2
    float32_t B[8] = {
        1.0f, 2.0f, 3.0f, 4.0f,   // Col 0
        5.0f, 6.0f, 7.0f, 8.0f    // Col 1
    };

    // Declare the result matrix C (3x2)
    float32_t C[n * m];

    matrix_multiply_c(A, B, C, n, m, k);

    while (1);
    return 0;
}

void Reset_Handler(void) {
    main();
}

void Default_Handler(void) {
    while(1);
}