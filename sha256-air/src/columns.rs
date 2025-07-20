const WORD_SIZE: usize = 4;

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Word<T>(pub [T; WORD_SIZE]);

/// A set of columns needed to compute `rotateright` of a word with a fixed offset R.
///
/// Note that we decompose shifts into a byte shift and a bit shift.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct FixedRotateRightOperation<T> {
    /// The output value.
    pub value: Word<T>,

    /// The shift output of `shrcarry` on each byte of a word.
    pub shift: Word<T>,

    /// The carry ouytput of `shrcarry` on each byte of a word.
    pub carry: Word<T>,
}

/// A set of columns needed to compute the xor of two words.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct XorOperation<T> {
    /// The result of `x ^ y`.
    pub value: Word<T>,
}

/// A set of columns needed to compute the and of two words.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct AndOperation<T> {
    /// The result of `x & y`.
    pub value: Word<T>,
}

/// A set of columns needed to compute the not of a word.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct NotOperation<T> {
    /// The result of `!x`.
    pub value: Word<T>,
}

/// A set of columns needed to compute the sum of five words.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Add5Operation<T> {
    /// The result of `a + b + c + d + e`.
    pub value: Word<T>,

    /// Indicates if the carry for the `i`th limb is 0.
    pub is_carry_0: Word<T>,

    /// Indicates if the carry for the `i`th limb is 1.
    pub is_carry_1: Word<T>,

    /// Indicates if the carry for the `i`th limb is 2.
    pub is_carry_2: Word<T>,

    /// Indicates if the carry for the `i`th limb is 3.
    pub is_carry_3: Word<T>,

    /// Indicates if the carry for the `i`th limb is 4. The carry when adding 5 words is at most 4.
    pub is_carry_4: Word<T>,

    /// The carry for the `i`th limb.
    pub carry: Word<T>,
}

/// A set of columns needed to compute the add of two words.
#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct AddOperation<T> {
    /// The result of `a + b`.
    pub value: Word<T>,

    /// Trace.
    pub carry: [T; 3],
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct Sha256CompressCols<T> {
    /// Which cycle within the octet we are currently processing.
    pub octet: [T; 8],

    /// This will specify which octet we are currently processing.
    ///  - The first octet is for initialize.
    ///  - The next 8 octets are for compress.
    ///  - The last octet is for finalize. 
    pub octet_num: [T; 10], 

    pub a: Word<T>,
    pub b: Word<T>,
    pub c: Word<T>,
    pub d: Word<T>,
    pub e: Word<T>,
    pub f: Word<T>,
    pub g: Word<T>,
    pub h: Word<T>,

    /// Current value of K[i]. This is a constant array that loops around every 64 iterations.
    pub k: Word<T>,

    pub e_rr_6: FixedRotateRightOperation<T>,
    pub e_rr_11: FixedRotateRightOperation<T>,
    pub e_rr_25: FixedRotateRightOperation<T>,

    /// `S1 := (e rightrotate 6) xor (e rightrotate 11) xor (e rightrotate 25)`.
    pub s1: XorOperation<T>,

    pub e_and_f: AndOperation<T>,
    pub e_not: NotOperation<T>,
    pub e_not_and_g: AndOperation<T>,
    /// `ch := (e and f) xor ((not e) and g)`.
    pub ch: XorOperation<T>,

    /// `temp1 := h + S1 + ch + k[i] + w[i]`.
    pub temp1: Add5Operation<T>,

    pub a_rr_2: FixedRotateRightOperation<T>,
    pub a_rr_13: FixedRotateRightOperation<T>,
    pub a_rr_22: FixedRotateRightOperation<T>,
    pub s0_intermediate: XorOperation<T>,
    /// `S0 := (a rightrotate 2) xor (a rightrotate 13) xor (a rightrotate 22)`.
    pub s0: XorOperation<T>,

    pub a_and_b: AndOperation<T>,
    pub a_and_c: AndOperation<T>,
    pub b_and_c: AndOperation<T>,
    pub maj_intermediate: XorOperation<T>,
    /// `maj := (a and b) xor (a and c) xor (b and c)`.
    pub maj: XorOperation<T>,

    /// `temp2 := S0 + maj`.
    pub temp2: AddOperation<T>,

    /// The next value of `e` is `d + temp1`.
    pub d_add_temp1: AddOperation<T>,
    /// The next value of `a` is `temp1 + temp2`.
    pub temp1_add_temp2: AddOperation<T>,

    pub is_initialize: T,
    pub is_compression: T,
    pub is_finalize: T,
    pub is_last_row: T,

}