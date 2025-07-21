use core::array;
use core::mem::transmute;

use p3_air::utils::{u64_to_16_bit_limbs, u64_to_bits_le};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;
use p3_maybe_rayon::iter::repeat_n;
use p3_maybe_rayon::prelude::*;
use tracing::instrument;

use crate::columns::{Sha256CompressCols, NUM_SHA256_COLS};

// TODO: Take generic iterable
#[instrument(name = "generate SHA256 trace", skip_all)]
pub fn generate_trace_rows<F: PrimeField64>(
    input: [u32; 4],
) -> Option<RowMajorMatrix<F>> {
    None
}