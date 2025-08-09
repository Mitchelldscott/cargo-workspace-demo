#![no_std]
#![no_main]

// use panic_halt as _;
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
fn matrix_multiply(a: &[f32], b: &[f32], c: &mut [f32], n: usize, m: usize, k: usize) {
    // The dimensions of the matrices are passed as `usize`, the standard type for sizes and indices.
    for i_idx in 0..n {
        for j_idx in 0..m {
            // Initialize the element of C to zero before summation
            c[n * j_idx + i_idx] = 0.0;
            for k_idx in 0..k {
                // Perform the dot product for the C[i][j] element
                c[n * j_idx + i_idx] += a[n * k_idx + i_idx] * b[k * j_idx + k_idx];
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Define the dimensions of the matrices
    const N: usize = 3;  // Rows of A and C
    const M: usize = 2;  // Columns of B and C
    const K: usize = 4;  // Columns of A and Rows of B

    // Declare and initialize matrices A and B in column-major order using vectors.
    // A is 3x4
    let a: [f32; 12] = [
        1.0, 2.0, 3.0,   // Col 0
        4.0, 5.0, 6.0,   // Col 1
        7.0, 8.0, 9.0,   // Col 2
        10.0, 11.0, 12.0 // Col 3
    ];

    // B is 4x2
    let b: [f32; 8] = [
        1.0, 2.0, 3.0, 4.0,   // Col 0
        5.0, 6.0, 7.0, 8.0    // Col 1
    ];

    // Declare the result matrix C (3x2) as a mutable vector, initialized with zeros.
    let mut c: [f32; 6] = [0.0; N * M];
    matrix_multiply(&a, &b, &mut c, N, M, K);
    loop {}
}
