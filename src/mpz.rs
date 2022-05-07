//! Unbounded Integer
//!
//! [MPIR 3.0.0 - C documentation](https://mpir.org/mpir-3.0.0.pdf)

use core::ffi::c_size_t;
use std::mem::{size_of, uninitialized};

use crate::ctype::{
    c_char, c_double, c_int, c_long, c_ulong, c_void, mp_bitcnt_t, mpz_ptr, mpz_srcptr, mpz_struct,
    size_t, CString, mp_limb_t
};

use crate::Sign;

#[link(name = "mpir", kind = "static")]
extern "C" {

    // ---------------------------------------------------------------------------------------------
    // Initialisation Functions

    /* The functions for integer arithmetic assume that all integer objects are initialized.
    You do that by calling the function mpz_init. For example,
    {
        mpz_t integ;
        mpz_init (integ);
        ...
        mpz_add (integ, ...);
        ...
        mpz_sub (integ, ...);
        /* Unless the program is about to exit, do ... */
        mpz_clear (integ);
    }
    As you can see, you can store new values any number of times, once an object is initialized */

    /// Initialize integer, and set its value to 0
    pub fn mpz_init(x: mpz_ptr);

    /// Initialize integer, with space for n bits, and set its value to 0.
    ///
    /// n is only the initial space, integer will grow automatically in the normal way, if necessary,
    /// for subsequent values stored. mpz_init2 makes it possible to avoid such reallocations if a
    /// maximum size is known in advance
    pub fn mpz_init2(x: mpz_ptr, n: mp_bitcnt_t);

    /// Free the space occupied by integer. Call this function for all mpz_t variables when you are
    /// done with them.
    pub fn mpz_clear(x: mpz_ptr);

    /// Change the space allocated for integer to n bits. The value in integer is preserved if it fits,
    /// or is set to 0 if not.
    ///
    /// This function can be used to increase the space for a variable in order to avoid repeated
    /// automatic reallocations, or to decrease it to give memory back to the heap
    pub fn mpz_realloc2(x: mpz_ptr, n: mp_bitcnt_t);

    // ---------------------------------------------------------------------------------------------
    // Assignment Functions

    /* These functions assign new values to already initialized integers */

    /// Set the value of rop from another mpz.
    pub fn mpz_set(rop: mpz_ptr, op: mpz_ptr);

    /// Set the value of rop from op.
    pub fn mpz_set_ui(rop: mpz_ptr, op: c_ulong);

    /// Set the value of rop from op.
    pub fn mpz_set_si(rop: mpz_ptr, op: c_ulong);

    /// Set the value of rop from a C double.
    pub fn mpz_set_d(rop: mpz_ptr, op: c_double);

    // TODO : when rational is added
    // /// Set the value of rop from op.
    // pub fn mpz_set_q (rop: mpz_ptr, op: );

    // TODO : when float is added
    // /// Set the value of rop from op.
    // pub fn mpz_set_f (rop: mpz_ptr, op: mpf_ptr);

    /// Set the value of rop from str, a null-terminated C string in base base. White space is allowed
    /// in the string, and is simply ignored.
    ///
    /// The base may vary from 2 to 62, or if base is 0, then the leading characters are used: 0x and
    /// 0X for hexadecimal, 0b and 0B for binary, 0 for octal, or decimal otherwise.
    /// For bases up to 36, case is ignored; upper-case and lower-case letters have the same value. For
    /// bases 37 to 62, upper-case letter represent the usual 10..35 while lower-case letter represent
    /// 36..61.
    ///
    /// This function returns 0 if the entire string is a valid number in base base. Otherwise it returns
    /// −1
    pub fn mpz_set_str(rop: mpz_ptr, s: *const c_char, base: c_int) -> c_int;

    /// Swap the values rop1 and rop2 efficiently
    pub fn mpz_swap(rop1: mpz_ptr, rop2: mpz_ptr);

    // ---------------------------------------------------------------------------------------------
    // Combined Initialization and Assignment Functions

    /* for convenience, MPIR provides a parallel series of initialize-and-set functions which initialize
    the output and then store the value there. These functions’ names have the form mpz_init_
    set...

    Here is an example of using one:
    {
        mpz_t pie;
        mpz_init_set_str (pie, "3141592653589793238462643383279502884", 10);
        ...
        mpz_sub (pie, ...);
        ...
        mpz_clear (pie);
    }

    Once the integer has been initialized by any of the mpz_init_set... functions, it can be used
    as the source or destination operand for the ordinary integer functions. Don’t use an initialize-
    and-set function on a variable already initialized! */

    /// Initialize rop with limb space and set the initial numeric value from another mpz.
    pub fn mpz_init_set(rop: mpz_ptr, op: mpz_ptr);

    /// Set the value of rop from op.
    pub fn mpz_init_set_ui(rop: mpz_ptr, op: c_ulong);

    /// Set the value of rop from op.
    pub fn mpz_init_set_si(rop: mpz_ptr, op: c_ulong);

    /// Initialize rop with limb space and set the initial numeric value from a C double.
    pub fn mpz_init_set_d(rop: mpz_ptr, op: c_double);

    /// Initialize rop and set its value like mpz_set_str (see its documentation above for details).
    ///
    /// If the string is a correct base base number, the function returns 0; if an error occurs it returns
    /// −1. rop is initialized even if an error occurs. (I.e., you have to call mpz_clear for it.)
    pub fn mpz_init_set_str(rop: mpz_ptr, s: *const c_char, base: c_int) -> c_int;

    // Conversion Functions

    /* This section describes functions for converting MPIR integers to standard C types. */

    /// Return the value of op as an mpir_ui.
    ///
    /// If op is too big to fit an mpir_ui then just the least significant bits that do fit are returned.
    /// The sign of op is ignored, only the absolute value is used.
    pub fn mpz_get_ui(op: mpz_srcptr) -> c_ulong;

    /// If op fits into a mpir_si return the value of op. Otherwise return the least significant part
    /// of op, with the same sign as op.
    ///
    /// If op is too big to fit in a mpir_si, the returned result is probably not very useful. To find
    /// out if the value will fit, use the function mpz_fits_slong_p.
    pub fn mpz_get_si(op: mpz_srcptr) -> c_ulong;

    /// Convert op to a double, truncating if necessary (ie. rounding towards zero).
    ///
    /// If the exponent from the conversion is too big, the result is system dependent. An infinity is
    /// returned where available. A hardware overflow trap may or may not occur.
    pub fn mpz_get_d(op: mpz_srcptr) -> c_double;

    /// Convert op to a double, truncating if necessary (ie. rounding towards zero), and returning
    /// the exponent separately.
    ///
    /// The return value is in the range 0.5 ≤ |d| < 1 and the exponent is stored to *exp. d ∗ 2exp is
    /// the (truncated) op value. If op is zero, the return is 0.0 and 0 is stored to *exp.
    ///
    /// This is similar to the standard C frexp function (see Section “Normalization Functions” in
    /// The GNU C Library Reference Manual).
    pub fn mpz_get_d_2exp(exp: mpz_ptr, op: mpz_srcptr);

    /// Convert op to a string of digits in base base. The base may vary from 2 to 36 or from −2 to
    /// −36.
    ///
    /// For base in the range 2..36, digits and lower-case letters are used; for −2..−36, digits and
    /// upper-case letters are used; for 37..62, digits, upper-case letters, and lower-case letters (in
    /// that significance order) are used.
    ///
    /// If str is NULL, the result string is allocated using the current allocation function (see
    /// Chapter 14 [Custom Allocation], page 106). The block will be strlen(str)+1 bytes, that
    /// being exactly enough for the string and null-terminator.
    ///
    /// If str is not NULL, it should point to a block of storage large enough for the result, that being
    /// mpz_sizeinbase (op, base) + 2. The two extra bytes are for a possible minus sign, and the
    /// null-terminator.
    ///
    /// A pointer to the result string is returned, being either the allocated block, or the given str.
    pub fn mpz_get_str(s: *mut c_char, base: c_int, op: mpz_srcptr) -> *mut c_char;

    // ---------------------------------------------------------------------------------------------
    // Arithmetic Functions

    /// Set rop to op1 + op2.
    pub fn mpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to op1 + op2.
    pub fn mpz_add_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong) -> c_int;

    /// Set rop to op1 − op2.
    pub fn mpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to op1 − op2.
    pub fn mpz_sub_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);

    /// Set rop to op1 − op2.
    pub fn mpz_ui_sub(rop: mpz_ptr, op1: c_ulong, op2: mpz_srcptr);

    /// Set rop to op1 × op2.
    pub fn mpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to op1 × op2.
    pub fn mpz_mul_si(rop: mpz_ptr, op1: mpz_srcptr, op2: c_long);

    /// Set rop to op1 × op2.
    pub fn mpz_mul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);

    /// Set rop to rop + op1 × op2.
    pub fn mpz_addmul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to rop + op1 × op2.
    pub fn mpz_addmul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);

    /// Set rop to rop − op1 × op2.
    pub fn mpz_submul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to rop − op1 × op2.
    pub fn mpz_submul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);

    /// Set rop to op1 × 2op2. This operation can also be defined as a left shift by op2 bits.
    pub fn mpz_mul_2exp(rop: mpz_ptr, op1: mpz_srcptr, op2: mp_bitcnt_t);

    /// Set rop to −op.
    pub fn mpz_neg(rop: mpz_ptr, op: mpz_srcptr);

    /// Set rop to the absolute value of op.
    pub fn mpz_abs(rop: mpz_ptr, op: mpz_srcptr);

    // ---------------------------------------------------------------------------------------------
    // Division Functions

    /* Division is undefined if the divisor is zero. Passing a zero divisor to the division or modulo
    functions (including the modular powering functions mpz_powm and mpz_powm_ui), will cause an
    intentional division by zero. This lets a program handle arithmetic exceptions in these functions
    the same way as for normal C int arithmetic.

    Divide n by d, forming a quotient q and/or remainder r. For the 2exp functions, d = 2b. The
    rounding is in three styles, each suiting different applications.

    • cdiv rounds q up towards +∞, and r will have the opposite sign to d. The c stands for
    “ceil”.

    • fdiv rounds q down towards −∞, and r will have the same sign as d. The f stands for
    “floor”.

    • tdiv rounds q towards zero, and r will have the same sign as n. The t stands for
    “truncate”.

    In all cases q and r will satisfy n = qd + r, and r will satisfy 0 ≤ |r| < |d|.
    34 MPIR 3.0.0

    The q functions calculate only the quotient, the r functions only the remainder, and the qr
    functions calculate both. Note that for qr the same variable cannot be passed for both q and
    r, or results will be unpredictable.

    For the ui variants the return value is the remainder, and in fact returning the remainder is
    all the div_ui functions do. For tdiv and cdiv the remainder can be negative, so for those
    the return value is the absolute value of the remainder.

    For the 2exp variants the divisor is 2b. These functions are implemented as right shifts and
    bit masks, but of course they round the same as the other functions.

    For positive n both mpz_fdiv_q_2exp and mpz_tdiv_q_2exp are simple bitwise right shifts.
    For negative n, mpz_fdiv_q_2exp is effectively an arithmetic right shift treating n as twos
    complement the same as the bitwise logical functions do, whereas mpz_tdiv_q_2exp effec-
    tively treats n as sign and magnitude. */

    pub fn mpz_cdiv_q(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong);

    pub fn mpz_cdiv_r(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong);

    pub fn mpz_cdiv_qr(rop: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: c_ulong);

    pub fn mpz_cdiv_q_ui(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_cdiv_r_ui(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_cdiv_qr_ui(rop: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_cdiv_ui(rop: mpz_ptr, d: c_ulong) -> c_ulong;

    pub fn mpz_cdiv_q_2exp(rop: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);

    pub fn mpz_cdiv_r_2exp(rop: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);

    pub fn mpz_fdiv_q(rop: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    pub fn mpz_fdiv_r(rop: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    pub fn mpz_fdiv_qr(rop: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    pub fn mpz_fdiv_q_ui(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_fdiv_r_ui(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_fdiv_qr_ui(rop: mpz_ptr, r: mpz_srcptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_fdiv_ui(rop: mpz_ptr, d: c_ulong) -> c_ulong;

    pub fn mpz_fdiv_q_2exp(rop: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);

    pub fn mpz_fdiv_r_2exp(rop: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);

    pub fn mpz_tdiv_q(rop: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    pub fn mpz_tdiv_r(rop: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    pub fn mpz_tdiv_qr(rop: mpz_ptr, r: mpz_srcptr, n: mpz_srcptr, d: mpz_srcptr);

    pub fn mpz_tdiv_q_ui(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_tdiv_r_ui(rop: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_tdiv_qr_ui(rop: mpz_ptr, r: mpz_srcptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;

    pub fn mpz_tdiv_ui(rop: mpz_ptr, d: c_ulong) -> c_ulong;

    pub fn mpz_tdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);

    pub fn mpz_tdiv_r_2exp(r: mpz_ptr, n: mpz_srcptr, b: mp_bitcnt_t);

    /// Set r to n mod d. The sign of the divisor is ignored; the result is always non-negative.
    ///
    /// mpz_mod_ui is identical to mpz_fdiv_r_ui above, returning the remainder as well as setting
    /// r. See mpz_fdiv_ui above if only the return value is wanted.
    pub fn mpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    /// Set r to n mod d. The sign of the divisor is ignored; the result is always non-negative.
    ///
    /// mpz_mod_ui is identical to mpz_fdiv_r_ui above, returning the remainder as well as setting
    /// r. See mpz_fdiv_ui above if only the return value is wanted.
    pub fn mpz_mod_ui(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr) -> c_ulong;

    /// Set q to n/d. These functions produce correct results only when it is known in advance that
    /// d divides n.
    ///
    /// These routines are much faster than the other division functions, and are the best choice
    /// when exact division is known to occur, for example reducing a rational to lowest terms.
    pub fn mpz_divexact(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);

    /// Set q to n/d. These functions produce correct results only when it is known in advance that
    /// d divides n.
    ///
    /// These routines are much faster than the other division functions, and are the best choice
    /// when exact division is known to occur, for example reducing a rational to lowest terms.
    pub fn mpz_divexact_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong);

    /// Return non-zero if n is exactly divisible by d, or in the case of mpz_divisible_2exp_p by 2b.
    ///
    /// n is divisible by d if there exists an integer q satisfying n = qd. Unlike the other division
    /// functions, d = 0 is accepted and following the rule it can be seen that only 0 is considered
    /// divisible by 0.
    pub fn mpz_divisible_p(n: mpz_ptr, d: mpz_srcptr) -> c_int;

    /// Return non-zero if n is exactly divisible by d, or in the case of mpz_divisible_2exp_p by 2b.
    ///
    /// n is divisible by d if there exists an integer q satisfying n = qd. Unlike the other division
    /// functions, d = 0 is accepted and following the rule it can be seen that only 0 is considered
    /// divisible by 0.
    pub fn mpz_divisible_ui_p(n: mpz_ptr, d: c_ulong) -> c_int;

    /// Return non-zero if n is exactly divisible by d, or in the case of mpz_divisible_2exp_p by 2b.
    ///
    /// n is divisible by d if there exists an integer q satisfying n = qd. Unlike the other division
    /// functions, d = 0 is accepted and following the rule it can be seen that only 0 is considered
    /// divisible by 0.
    pub fn mpz_divisible_2exp_p(n: mpz_ptr, b: mp_bitcnt_t) -> c_int;

    /// Return non-zero if n is congruent to c modulo d, or in the case of mpz_congruent_2exp_p
    /// modulo 2b.
    ///
    /// n is congruent to c mod d if there exists an integer q satisfying n = c + qd. Unlike the other
    /// division functions, d = 0 is accepted and following the rule it can be seen that n and c are
    /// considered congruent mod 0 only when exactly equal.
    pub fn mpz_congruent_p(n: mpz_ptr, c: mpz_srcptr, d: mpz_srcptr) -> c_int;

    /// Return non-zero if n is congruent to c modulo d, or in the case of mpz_congruent_2exp_p
    /// modulo 2b.
    ///
    /// n is congruent to c mod d if there exists an integer q satisfying n = c + qd. Unlike the other
    /// division functions, d = 0 is accepted and following the rule it can be seen that n and c are
    /// considered congruent mod 0 only when exactly equal.
    pub fn mpz_congruent_ui_p(n: mpz_ptr, c: c_ulong, d: c_ulong) -> c_int;

    /// Return non-zero if n is congruent to c modulo d, or in the case of mpz_congruent_2exp_p
    /// modulo 2b.
    ///
    /// n is congruent to c mod d if there exists an integer q satisfying n = c + qd. Unlike the other
    /// division functions, d = 0 is accepted and following the rule it can be seen that n and c are
    /// considered congruent mod 0 only when exactly equal.
    pub fn mpz_congruent_2exp_p(n: mpz_ptr, c: mpz_srcptr, b: mp_bitcnt_t) -> c_int;

    // ---------------------------------------------------------------------------------------------
    // Exponentiation Functions

    /// Set rop to baseexp mod mod.
    ///
    /// A negative exp is supported in mpz_powm if an inverse base−1 mod mod exists (see mpz_
    /// invert in Section 5.9 [Number Theoretic Functions], page 36). If an inverse doesn’t exist
    /// then a divide by zero is raised.
    pub fn mpz_powm(rop: mpz_ptr, base: mpz_srcptr, exp: mpz_srcptr, m: mpz_srcptr);

    /// Set rop to base^exp mod mod.
    ///
    /// A negative exp is supported in mpz_powm if an inverse base−1 mod mod exists (see mpz_
    /// invert in Section 5.9 [Number Theoretic Functions], page 36). If an inverse doesn’t exist
    /// then a divide by zero is raised.
    pub fn mpz_powm_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong, m: mpz_srcptr);

    /// Set rop to base^exp. The case 00 yields 1.
    pub fn mpz_pow_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong);

    /// Set rop to base^exp. The case 00 yields 1.
    pub fn mpz_ui_pow_ui(rop: mpz_ptr, base: c_ulong, exp: c_ulong);

    // ---------------------------------------------------------------------------------------------
    // Root Extraction Functions

    /// Set rop to b n
    ///
    /// √opc, the truncated integer part of the nth root of op. Return non-zero if the
    /// computation was exact, i.e., if op is rop to the nth power.
    pub fn mpz_nthroot(rop: mpz_ptr, op: mpz_srcptr, n: c_ulong);

    /// Set rop to b n
    ///
    /// √opc, the truncated integer part of the nth root of op.
    pub fn mpz_root(rop: mpz_ptr, op: mpz_srcptr, n: c_ulong) -> c_int;

    /// Set root to b n
    ///
    /// √uc, the truncated integer part of the nth root of u. Set rem to the remainder,
    /// (u − rootn).
    pub fn mpz_rootrem(rop: mpz_ptr, rem: mpz_srcptr, u: mpz_srcptr, n: c_ulong);

    /// Set rop to b√opc, the truncated integer part of the square root of op.
    pub fn mpz_sqrt(rop: mpz_ptr, op: mpz_srcptr);

    /// Set rop1 to b√opc, like mpz_sqrt. Set rop2 to the remainder (op − rop12), which will be
    /// zero if op is a perfect square.
    ///
    /// If rop1 and rop2 are the same variable, the results are undefined.
    pub fn mpz_sqrtrem(rop1: mpz_ptr, rop2: mpz_ptr, op: mpz_srcptr);

    /// Return non-zero if op is a perfect power, i.e., if there exist integers a and b, with b > 1, such
    /// that op = ab.
    ///
    /// Under this definition both 0 and 1 are considered to be perfect powers. Negative values of
    /// op are accepted, but of course can only be odd perfect powers.
    pub fn mpz_perfect_power_p(op: mpz_srcptr) -> c_int;

    /// Return non-zero if op is a perfect square, i.e., if the square root of op is an integer. Under
    /// this definition both 0 and 1 are considered to be perfect squares.
    pub fn mpz_perfect_square_p(op: mpz_srcptr) -> c_int;

    // ---------------------------------------------------------------------------------------------
    // Number Theoretic Functions

    // TODO mpz_probable_prime_p
    // /// Determine whether n is a probable prime with the chance of error being at most 1 in 2^prob.
    // /// return value is 1 if n is probably prime, or 0 if n is definitely composite.
    // ///
    // /// This function does some trial divisions to speed up the average case, then some probabilistic
    // /// primality tests to achieve the desired level of error.
    // /// div can be used to inform the function that trial division up to div has already been performed
    // /// on n and so n has NO divisors <= div.Use 0 to inform the function that no trial division has
    // /// been done.
    // /// This function interface is preliminary and may change in the future.
    // pub fn mpz_probable_prime_p (mpz t n, gmp randstate t state, int prob, mpir ui div) -> c_int;

    // TODO mpz_likely_prime_p
    // /// Determine whether n is likely a prime, i.e. you can consider it a prime for practical purposes.
    // /// return value is 1 if n can be considered prime, or 0 if n is definitely composite.
    // ///
    // /// This function does some trial divisions to speed up the average case, then some probabilistic
    // /// primality tests. The term “likely” refers to the fact that the number will not have small
    // /// factors.
    // /// div can be used to inform the function that trial division up to div has already been performed
    // /// on n and so n has NO divisors <= div
    // /// This function interface is preliminary and may change in the future.
    // pub fn mpz_likely_prime_p(mpz t n, gmp randstate t state, mpir ui div) -> c_int;

    // TODO mpz_next_prime_candidate
    // /// Set rop to the next candidate prime greater than op. Note that this function will occasionally
    // /// return composites. It is designed to give a quick method for generating numbers which do
    // /// not have small prime factors (less than 1000) and which pass a small number of rounds of
    // /// Miller-Rabin (just two rounds).The test is designed for speed, assuming that a high quality
    // /// followup test can then be run to ensure primality.
    // ///
    // /// The variable state must be initialized by calling one of the gmp_randinit functions
    // /// (Section 9.1 [Random State Initialization], page 67) before invoking this function.
    // pub fn mpz_next_prime_candidate(mpz t rop, mpz t op, gmp randstate t state);

    // TODO mpz_gcd
    // /// Set rop to the greatest common divisor of op1 and op2. The result is always positive even if
    // /// one or both input operands are negative.
    // pub fn mpz_gcd(mpz t rop, mpz t op1, mpz t op2);

    // TODO mpz_gcd_ui
    // /// Compute the greatest common divisor of op1 and op2. If rop is not NULL, store the result
    // /// there.
    // /// If the result is small enough to fit in an mpir_ui, it is returned. If the result does not fit, 0
    // /// is returned, and the result is equal to the argument op1. Note that the result will always fit
    // /// if op2 is non-zero.
    // pub fn mpz_gcd_ui (mpz t rop, mpz t op1, mpir ui op2) -> c_ulong;

    // TODO mpz_gcdext
    // /// Set g to the greatest common divisor of a and b, and in addition set s and t to coefficients
    // /// satisfying as + bt = g. The value in g is always positive, even if one or both of a and b
    // /// are negative (or zero if both inputs are zero). The values in s and t are chosen such that
    // /// normally, |s| < |b|/(2g) and |t| < |a|/(2g), and these relations define s and t uniquely. There
    // /// are a few exceptional cases:
    // /// If |a| = |b|, then s = 0, t = sgn(b).
    // /// Otherwise, s = sgn(a) if b = 0 or |b| = 2g, and t = sgn(b) if a = 0 or |a| = 2g.
    // /// In all cases, s = 0 if and only if g = |b|, i.e., if b divides a or a = b = 0.
    // /// If t is NULL then that value is not computed.
    // pub fn mpz_gcdext(mpz t g, mpz t s, mpz t t, const mpz t a, const mpz t b);

    // TODO mpz_lcm
    // /// Set rop to the least common multiple of op1 and op2. rop is always positive, irrespective of
    // /// the signs of op1 and op2. rop will be zero if either op1 or op2 is zero.
    // pub fn mpz_lcm(mpz t rop, mpz t op1, mpz t op2);

    // TODO mpz_lcm_ui
    // /// Set rop to the least common multiple of op1 and op2. rop is always positive, irrespective of
    // /// the signs of op1 and op2. rop will be zero if either op1 or op2 is zero.
    // pub fn mpz_lcm_ui (mpz t rop, mpz t op1, mpir ui op2);

    // TODO mpz_invert
    // /// Compute the inverse of op1 modulo op2 and put the result in rop. If the inverse exists, the
    // /// return value is non-zero and rop will satisfy 0 ≤ rop < op2. If an inverse doesn’t exist the
    // /// return value is zero and rop is undefined.
    // pub fn mpz_invert (mpz t rop, mpz t op1, mpz t op2) -> c_int;

    // TODO mpz_jacobi
    // /// Calculate the Jacobi symbol ( a b ).
    // /// This is defined only for b odd.
    // pub fn mpz_jacobi (mpz t a, mpz t b) -> c_int;

    // TODO mpz_legendre
    // /// Calculate the Legendre symbol ( a p ).
    // /// This is defined only for p an odd positive prime, and
    // /// for such p it’s identical to the Jacobi symbol.
    // pub fn mpz_legendre (mpz t a, mpz t p) -> c_int;

    // TODO mpz_kronecker
    // /// Calculate the Jacobi symbol ( a b ) with the Kronecker extension ( a 2 ) = ( 2 a ) when a odd, or( a 2 ) = 0 when a even.
    // ///
    // /// When b is odd the Jacobi symbol and Kronecker symbol are identical, so mpz_kronecker_ui
    // /// etc can be used for mixed precision Jacobi symbols too.
    // ///
    // /// For more information see Henri Cohen section 1.4.2 (see Appendix B [References], page 145),
    // /// or any number theory textbook. See also the example program demos/qcn.c which uses
    // /// mpz_kronecker_ui on the MPIR website.
    // pub fn mpz_kronecker (mpz t a, mpz t b) -> c_int;

    // TODO mpz_kronecker_si
    // /// Calculate the Jacobi symbol ( a b ) with the Kronecker extension ( a 2 ) = ( 2 a ) when a odd, or( a 2 ) = 0 when a even.
    // ///
    // /// When b is odd the Jacobi symbol and Kronecker symbol are identical, so mpz_kronecker_ui
    // /// etc can be used for mixed precision Jacobi symbols too.
    // ///
    // /// For more information see Henri Cohen section 1.4.2 (see Appendix B [References], page 145),
    // /// or any number theory textbook. See also the example program demos/qcn.c which uses
    // /// mpz_kronecker_ui on the MPIR website.
    // pub fn mpz_kronecker_si (mpz t a, mpir si b) -> c_int;

    // TODO mpz_kronecker_ui
    // /// Calculate the Jacobi symbol ( a b ) with the Kronecker extension ( a 2 ) = ( 2 a ) when a odd, or( a 2 ) = 0 when a even.
    // ///
    // /// When b is odd the Jacobi symbol and Kronecker symbol are identical, so mpz_kronecker_ui
    // /// etc can be used for mixed precision Jacobi symbols too.
    // ///
    // /// For more information see Henri Cohen section 1.4.2 (see Appendix B [References], page 145),
    // /// or any number theory textbook. See also the example program demos/qcn.c which uses
    // /// mpz_kronecker_ui on the MPIR website.
    // pub fn mpz_kronecker_ui (mpz t a, mpir ui b) -> c_int;

    // TODO mpz_si_kronecker
    // /// Calculate the Jacobi symbol ( a b ) with the Kronecker extension ( a 2 ) = ( 2 a ) when a odd, or( a 2 ) = 0 when a even.
    // ///
    // /// When b is odd the Jacobi symbol and Kronecker symbol are identical, so mpz_kronecker_ui
    // /// etc can be used for mixed precision Jacobi symbols too.
    // ///
    // /// For more information see Henri Cohen section 1.4.2 (see Appendix B [References], page 145),
    // /// or any number theory textbook. See also the example program demos/qcn.c which uses
    // /// mpz_kronecker_ui on the MPIR website.
    // pub fn mpz_si_kronecker (mpir si a, mpz t b) -> c_int;

    // TODO mpz_ui_kronecker
    // /// Calculate the Jacobi symbol ( a b ) with the Kronecker extension ( a 2 ) = ( 2 a ) when a odd, or( a 2 ) = 0 when a even.
    // ///
    // /// When b is odd the Jacobi symbol and Kronecker symbol are identical, so mpz_kronecker_ui
    // /// etc can be used for mixed precision Jacobi symbols too.
    // ///
    // /// For more information see Henri Cohen section 1.4.2 (see Appendix B [References], page 145),
    // /// or any number theory textbook. See also the example program demos/qcn.c which uses
    // /// mpz_kronecker_ui on the MPIR website.
    // pub fn mpz_ui_kronecker (mpir ui a, mpz t b) -> c_int;

    // TODO mpz_remove
    // /// Remove all occurrences of the factor f from op and store the result in rop. The return value
    // /// is how many such occurrences were removed.
    // pub fn mpz_remove (mpz t rop, mpz t op, mpz t f) -> mp_bitcnt_t;

    // TODO mpz_fac_ui
    // /// Set rop to the factorial of n: mpz_fac_ui computes the plain factorial n!, mpz_2fac_ui
    // /// computes the double-factorial n!!, and mpz_mfac_uiui the m-multi-factorial n!(m).
    // pub fn mpz_fac_ui (mpz t rop, unsigned long int n);

    // TODO mpz_2fac_ui
    // /// Set rop to the factorial of n: mpz_fac_ui computes the plain factorial n!, mpz_2fac_ui
    // /// computes the double-factorial n!!, and mpz_mfac_uiui the m-multi-factorial n!(m).
    // pub fn mpz_2fac_ui (mpz t rop, unsigned long int n);

    // TODO mpz_mfac_uiui
    // /// Set rop to the factorial of n: mpz_fac_ui computes the plain factorial n!, mpz_2fac_ui
    // /// computes the double-factorial n!!, and mpz_mfac_uiui the m-multi-factorial n!(m).
    // pub fn mpz_mfac_uiui (mpz t rop, unsigned long int n, unsigned long int m);

    // TODO mpz_primorial_ui
    // /// Set rop to the primorial of n, i.e. the product of all positive prime numbers ≤ n.
    // pub fn mpz_primorial_ui (mpz t rop, unsigned long int n);

    // TODO mpz_bin_ui
    // /// Compute the binomial coefficient ( n k ) and store the result in rop.
    // /// Negative values of n are supported by mpz_bin_ui, using the identity ( −n k ) = (−1)k ( n+k−1 k )
    // pub fn mpz_bin_ui (mpz t rop, mpz t n, mpir ui k);

    // TODO mpz_bin_uiui
    // /// Compute the binomial coefficient ( n k ) and store the result in rop.
    // /// Negative values of n are supported by mpz_bin_ui, using the identity ( −n k ) = (−1)k ( n+k−1 k )
    // pub fn mpz_bin_uiui (mpz t rop, mpir ui n, mpir ui k);

    // TODO mpz_fib_ui
    // /// mpz_fib_ui sets fn to to Fn, the n’th Fibonacci number. mpz_fib2_ui sets fn to Fn, and fnsub1 to Fn−1.
    // ///
    // /// These functions are designed for calculating isolated Fibonacci numbers. When a sequence of
    // /// values is wanted it’s best to start with mpz_fib2_ui and iterate the defining Fn+1 = Fn +Fn−1
    // /// or similar.
    // fn mpz_fib_ui (mpz t fn, mpir ui n);

    // TODO mpz_fib2_ui
    // /// mpz_fib_ui sets pub fn to to Fn, the n’th Fibonacci number. mpz_fib2_ui sets fn to Fn, and fnsub1 to Fn−1.
    // ///
    // /// These functions are designed for calculating isolated Fibonacci numbers. When a sequence of
    // /// values is wanted it’s best to start with mpz_fib2_ui and iterate the defining Fn+1 = Fn +Fn−1
    // /// or similar.
    // pub fn mpz_fib2_ui (mpz t fn, mpz t fnsub1, mpir ui n);

    // TODO mpz_lucnum_ui
    // /// mpz_lucnum_ui sets ln to to Ln, the n’th Lucas number. mpz_lucnum2_ui sets ln to Ln, and
    // /// lnsub1 to Ln−1.
    // ///
    // /// These functions are designed for calculating isolated Lucas numbers. When a sequence of
    // /// values is wanted it’s best to start with mpz_lucnum2_ui and iterate the defining Ln+1 =
    // /// Ln + Ln−1 or similar.
    // ///
    // /// The Fibonacci numbers and Lucas numbers are related sequences, so it’s never necessary
    // /// to call both mpz_fib2_ui and mpz_lucnum2_ui. The formulas for going from Fibonacci to
    // pub fn mpz_lucnum_ui (mpz t ln, mpir ui n);

    // TODO mpz_lucnum2_ui
    // /// mpz_lucnum_ui sets ln to to Ln, the n’th Lucas number. mpz_lucnum2_ui sets ln to Ln, and
    // /// lnsub1 to Ln−1.
    // ///
    // /// These functions are designed for calculating isolated Lucas numbers. When a sequence of
    // /// values is wanted it’s best to start with mpz_lucnum2_ui and iterate the defining Ln+1 =
    // /// Ln + Ln−1 or similar.
    // ///
    // /// The Fibonacci numbers and Lucas numbers are related sequences, so it’s never necessary
    // /// to call both mpz_fib2_ui and mpz_lucnum2_ui. The formulas for going from Fibonacci to
    // pub fn mpz_lucnum2_ui (mpz t ln, mpz t lnsub1, mpir ui n);

    // ---------------------------------------------------------------------------------------------
    // Comparison Functions

    /// Compare op1 and op2. Return a positive value if op1 > op2, zero if op1 = op2, or a negative
    /// value if op1 < op2.
    ///
    /// mpz_cmp_ui and mpz_cmp_si are macros and will evaluate their arguments more than once.
    /// mpz_cmp_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmp (op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;

    /// Compare op1 and op2. Return a positive value if op1 > op2, zero if op1 = op2, or a negative
    /// value if op1 < op2.
    ///
    /// mpz_cmp_ui and mpz_cmp_si are macros and will evaluate their arguments more than once.
    /// mpz_cmp_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmp_d (op1: mpz_srcptr, op2: c_double) -> c_int;

    /// Compare op1 and op2. Return a positive value if op1 > op2, zero if op1 = op2, or a negative
    /// value if op1 < op2.
    ///cros and will evaluate their arguments more than once.
    /// mpz_cmp_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmp_si (op1: mpz_srcptr, op2: c_long) -> c_int;

    /// Compare op1 and op2. Return a positive value if op1 > op2, zero if op1 = op2, or a negative
    /// value if op1 < op2.
    ///
    /// mpz_cmp_ui and mpz_cmp_si are macros and will evaluate their arguments more than once.
    /// mpz_cmp_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmp_ui (op1: mpz_srcptr, op2: c_ulong) -> c_int;

    /// Compare the absolute values of op1 and op2. Return a positive value if |op1| > |op2|, zero
    /// if |op1| = |op2|, or a negative value if |op1| < |op2|.
    ///
    /// mpz_cmpabs_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmpabs (op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;

    /// Compare the absolute values of op1 and op2. Return a positive value if |op1| > |op2|, zero
    /// if |op1| = |op2|, or a negative value if |op1| < |op2|.
    ///
    /// mpz_cmpabs_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmpabs_d (op1: mpz_srcptr, op2: c_double) -> c_int;

    /// Compare the absolute values of op1 and op2. Return a positive value if |op1| > |op2|, zero
    /// if |op1| = |op2|, or a negative value if |op1| < |op2|.
    ///
    /// mpz_cmpabs_d can be called with an infinity, but results are undefined for a NaN.
    pub fn mpz_cmpabs_ui (op1: mpz_srcptr, op2: c_ulong) -> c_int;

    /// Return +1 if op > 0, 0 if op = 0, and −1 if op < 0.
    /// This function is actually implemented as a macro. It evaluates its argument multiple times.
    fn mpz_sgn (op1: mpz_srcptr) -> c_int;

    // ---------------------------------------------------------------------------------------------
    // Logical and Bit Manipulation Functions

    /* These functions behave as if twos complement arithmetic were used (although sign-magnitude
    is the actual implementation). The least significant bit is number 0. */

    /// Set rop to op1 bitwise-and op2.
    pub fn mpz_and (rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to op1 bitwise inclusive-or op2.
    pub fn mpz_ior (rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to op1 bitwise exclusive-or op2.
    pub fn mpz_xor (rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);

    /// Set rop to the one’s complement of op.
    pub fn mpz_com (rop: mpz_ptr, op: mpz_srcptr);

    /// If op ≥ 0, return the population count of op, which is the number of 1 bits in the binary
    /// representation. If op < 0, the number of 1s is infinite, and the return value is ULONG MAX,
    /// the largest possible mp_bitcnt_t.
    pub fn mpz_popcount (rop: mpz_ptr) -> mp_bitcnt_t;

    /// If op1 and op2 are both ≥ 0 or both < 0, return the hamming distance between the two
    /// operands, which is the number of bit positions where op1 and op2 have different bit values.
    ///
    /// If one operand is ≥ 0 and the other < 0 then the number of bits different is infinite, and the
    /// return value is the largest possible imp_bitcnt_t.
    pub fn mpz_hamdist (op1: mpz_srcptr, op2: mpz_srcptr) -> mp_bitcnt_t;

    /// Scan op, starting from bit starting bit, towards more significant bits, until the first 0 or 1 bit
    /// (respectively) is found. Return the index of the found bit.
    ///
    /// If the bit at starting bit is already what’s sought, then starting bit is returned.
    /// If there’s no bit found, then the largest possible mp_bitcnt_t is returned. This will happen
    /// in mpz_scan0 past the end of a positive number, or mpz_scan1 past the end of a nonnegative
    /// number.
    pub fn mpz_scan0 (op: mpz_srcptr, starting_bit: mp_bitcnt_t) -> mp_bitcnt_t;

    /// Scan op, starting from bit starting bit, towards more significant bits, until the first 0 or 1 bit
    /// (respectively) is found. Return the index of the found bit.
    ///
    /// If the bit at starting bit is already what’s sought, then starting bit is returned.
    /// If there’s no bit found, then the largest possible mp_bitcnt_t is returned. This will happen
    /// in mpz_scan0 past the end of a positive number, or mpz_scan1 past the end of a nonnegative
    /// number.
    pub fn mpz_scan1 (op: mpz_srcptr, starting_bit: mp_bitcnt_t) -> mp_bitcnt_t;

    /// Set bit bit index in rop.
    pub fn mpz_setbit (rop: mpz_ptr, bit_index: mp_bitcnt_t);

    /// Clear bit bit index in rop.
    pub fn mpz_clrbit (rop: mpz_ptr, bit_index: mp_bitcnt_t);

    /// Complement bit bit index in rop.
    pub fn mpz_combit (rop: mpz_ptr, bit_index: mp_bitcnt_t);

    /// Test bit bit index in op and return 0 or 1 accordingly.
    pub fn mpz_tstbit (op: mpz_srcptr, starting_bit: mp_bitcnt_t) -> c_int;

    // ---------------------------------------------------------------------------------------------
    // Random Number Functions

    /*
    The random number functions of MPIR come in two groups; older function that rely on a global
    state, and newer functions that accept a state parameter that is read and modified. Please see
    the Chapter 9 [Random Number Functions], page 67 for more information on how to use and
    not to use random number functions.
    */

    // TODO mpz_urandomb
    // /// Generate a uniformly distributed random integer in the range 0 to 2n − 1, inclusive.
    // ///
    // /// The variable state must be initialized by calling one of the gmp_randinit functions
    // /// (Section 9.1 [Random State Initialization], page 67) before invoking this function.
    // pub fn mpz_urandomb (mpz t rop, gmp randstate t state, mp bitcnt t n);

    // TODO mpz_urandomm
    // /// Generate a uniform random integer in the range 0 to n − 1, inclusive.
    // ///
    // /// The variable state must be initialized by calling one of the gmp_randinit functions
    // /// (Section 9.1 [Random State Initialization], page 67) before invoking this function.
    // pub fn mpz_urandomm (mpz t rop, gmp randstate t state, mpz t n);

    // TODO mpz_rrandomb
    // /// Generate a random integer with long strings of zeros and ones in the binary representation.
    // ///
    // /// Useful for testing functions and algorithms, since this kind of random numbers have proven
    // /// to be more likely to trigger corner-case bugs. The random number will be in the range 0 to
    // /// 2n − 1, inclusive.
    // /// The variable state must be initialized by calling one of the gmp_randinit functions
    // /// (Section 9.1 [Random State Initialization], page 67) before invoking this function.
    // pub fn mpz_rrandomb (mpz t rop, gmp randstate t state, mp bitcnt t n);

    // ---------------------------------------------------------------------------------------------
    // Miscellaneous Functions

    /// Return non-zero iff the value of op fits in an unsigned long, long, unsigned int, signed
    /// int, unsigned short int, or signed short int, respectively. Otherwise, return zero.
    pub fn mpz_fits_ulong_p (op: mpz_srcptr) -> c_int;

    /// Return non-zero iff the value of op fits in an unsigned long, long, unsigned int, signed
    /// int, unsigned short int, or signed short int, respectively. Otherwise, return zero.
    pub fn mpz_fits_slong_p (op: mpz_srcptr) -> c_int;

    /// Return non-zero iff the value of op fits in an unsigned long, long, unsigned int, signed
    /// int, unsigned short int, or signed short int, respectively. Otherwise, return zero.
    pub fn mpz_fits_uint_p (op: mpz_srcptr) -> c_int;

    /// Return non-zero iff the value of op fits in an unsigned long, long, unsigned int, signed
    /// int, unsigned short int, or signed short int, respectively. Otherwise, return zero.
    pub fn mpz_fits_sint_p (op: mpz_srcptr) -> c_int;

    /// Return non-zero iff the value of op fits in an unsigned long, long, unsigned int, signed
    /// int, unsigned short int, or signed short int, respectively. Otherwise, return zero.
    pub fn mpz_fits_ushort_p (op: mpz_srcptr) -> c_int;

    /// Return non-zero iff the value of op fits in an unsigned long, long, unsigned int, signed
    /// int, unsigned short int, or signed short int, respectively. Otherwise, return zero.
    pub fn mpz_fits_sshort_p (op: mpz_srcptr) -> c_int;

    /// Determine whether op is odd or even, respectively. Return non-zero if yes, zero if no. These
    /// macros evaluate their argument more than once.
    pub fn mpz_odd_p (op: mpz_srcptr) -> c_int;

    /// Determine whether op is odd or even, respectively. Return non-zero if yes, zero if no. These
    /// macros evaluate their argument more than once.
    pub fn mpz_even_p (op: mpz_srcptr) -> c_int;

    /// Return the size of op measured in number of digits in the given base. base can vary from 2
    /// to 36.
    ///
    /// The sign of op is ignored, just the absolute value is used. The result will be either
    /// exact or 1 too big. If base is a power of 2, the result is always exact. If op is zero the return
    /// value is always 1.
    ///
    /// This function can be used to determine the space required when converting op to a string. The
    /// right amount of allocation is normally two more than the value returned by mpz_sizeinbase,
    /// one extra for a minus sign and one for the null-terminator.
    ///
    /// It will be noted that mpz_sizeinbase(op,2) can be used to locate the most significant 1 bit
    /// in op, counting from 1. (Unlike the bitwise functions which start from 0, See Section 5.11
    /// [Logical and Bit Manipulation Functions], page 39.)
    pub fn mpz_sizeinbase (op: mpz_srcptr, base: c_int) -> size_t;

    // ---------------------------------------------------------------------------------------------
    // Special Functions

    /* The functions in this section are for various special purposes. Most applications will not need
    them. */

    // TODO mpz_array_init
    // /// This is a special type of initialization. Fixed space of fixed num bits is allocated to each of
    // /// the array size integers in integer array. There is no way to free the storage allocated by this
    // /// function. Don’t call mpz_clear!
    // ///
    // /// The integer array parameter is the first mpz_t in the array. For example,
    // ///
    // /// mpz_t arr[20000];
    // /// mpz_array_init (arr[0], 20000, 512);
    // ///
    // /// This function is only intended for programs that create a large number of integers and need
    // /// to reduce memory usage by avoiding the overheads of allocating and reallocating lots of small
    // /// blocks. In normal programs this function is not recommended.
    // /// The space allocated to each integer by this function will not be automatically increased, unlike
    // /// the normal mpz_init, so an application must ensure it is sufficient for any value stored. The
    // /// following space requirements apply to various routines,
    // /// 44 MPIR 3.0.0
    // /// • mpz_abs, mpz_neg, mpz_set, mpz_set_si and mpz_set_ui need room for the value they
    // /// store.
    // /// • mpz_add, mpz_add_ui, mpz_sub and mpz_sub_ui need room for the larger of the two
    // /// operands, plus an extra mp_bits_per_limb.
    // /// • mpz_mul, mpz_mul_ui and mpz_mul_ui need room for the sum of the number of bits in
    // /// their operands, but each rounded up to a multiple of mp_bits_per_limb.
    // /// • mpz_swap can be used between two array variables, but not between an array and a
    // /// normal variable.
    // /// For other functions, or if in doubt, the suggestion is to calculate in a regular mpz_init variable
    // /// and copy the result to an array variable with mpz_set.
    // /// This function is obsolete. It will disappear from future MPIR releases.
    // pub fn mpz_array_init(mpz t integer_array, size t array_size, mp size t fixed_num_bits);

    // TODO _mpz_realloc
    // /// Change the space for integer to new alloc limbs. The value in integer is preserved if it fits,
    // /// or is set to 0 if not. The return value is not useful to applications and should be ignored.
    // /// mpz_realloc2 is the preferred way to accomplish allocation changes like this. mpz_realloc2
    // /// and _mpz_realloc are the same except that _mpz_realloc takes its size in limbs.
    // void * _mpz_realloc (mpz t integer, mp size t new_alloc)

    // TODO mpz_getlimbn
    // /// Return limb number n from op. The sign of op is ignored, just the absolute value is used.
    // /// The least significant limb is number 0.
    // /// mpz_size can be used to find how many limbs make up op. mpz_getlimbn returns zero if n
    // /// is outside the range 0 to mpz_size(op)-1.
    // mp_limb_t mpz_getlimbn (mpz t op, mp size t n)

    /// Return the size of op measured in number of limbs. If op is zero, the returned value will be
    /// zero.
    fn mpz_size (op: mpz_srcptr) -> c_size_t;

    // TODO mpz_limbs_read
    // /// Return a pointer to the limb array representing the absolute value of x.
    // /// The size of the array is mpz_size(x). Intended for read access only.
    // const mp_limb_t * mpz_limbs_read (const mpz t x)

    // TODO mpz_limbs_write
    // /// Return a pointer to the limb array, intended for write access. The array is reallocated as
    // /// needed, to make room for n limbs. Requires n > 0. The mpz_limbs_modify function returns
    // /// an array that holds the old absolute value of x, while mpz_limbs_write may destroy the old
    // /// value and return an array with unspecified contents.
    // mp_limb_t * mpz_limbs_write (mpz t x, mp size t n)

    // TODO mpz_limbs_modify
    // /// Return a pointer to the limb array, intended for write access. The array is reallocated as
    // /// needed, to make room for n limbs. Requires n > 0. The mpz_limbs_modify function returns
    // /// an array that holds the old absolute value of x, while mpz_limbs_write may destroy the old
    // /// value and return an array with unspecified contents.
    // mp_limb_t * mpz_limbs_modify (mpz t x, mp size t n)

    // TODO mpz_limbs_finish
    // /// Updates the internal size field of x. Used after writing to the limb array pointer returned
    // /// by mpz_limbs_write or mpz_limbs_modify is completed. The array should contain |s| valid
    // /// limbs, representing the new absolute value for x, and the sign of x is taken from the sign of
    // /// s. This function never reallocates x, so the limb pointer remains valid.
    // /// void foo (mpz_t x)
    // /// {
    // /// mp_size_t n, i;
    // /// mp_limb_t *xp;
    // /// Chapter 5: Integer Functions 45
    // /// n = mpz_size (x);
    // /// xp = mpz_limbs_modify (x, 2*n);
    // /// for (i = 0; i < n; i++)
    // /// xp[n+i] = xp[n-1-i];
    // /// mpz_limbs_finish (x, mpz_sgn (x) < 0 ? - 2*n : 2*n);
    // /// }
    // pub fn mpz_limbs_finish (mpz t x, mp size t s);

    // TODO mpz_roinit_n
    // /// Special initialization of x, using the given limb array and size. x should be treated as read-
    // /// only: it can be passed safely as input to any mpz function, but not as an output. The array
    // /// xp must point to at least a readable limb, its size is |xs|, and the sign of x is the sign of xs.
    // /// For convenience, the function returns x, but cast to a const pointer type.
    // /// void foo (mpz_t x)
    // /// {
    // /// static const mp_limb_t y[3] = { 0x1, 0x2, 0x3 };
    // /// mpz_t tmp;
    // /// mpz_add (x, x, mpz_roinit_n (tmp, y, 3));
    // /// }
    // mpz_srcptr mpz_roinit_n (mpz t x, const mp limb t *xp, mp size t xs);

    // TODO MPZ_ROINIT_N
    // /// This macro expands to an initializer which can be assigned to an mpz t variable. The
    // /// limb array xp must point to at least a readable limb, moreover, unlike the mpz_roinit_n
    // /// function, the array must be normalized: if xs is non-zero, then xp[|xs| − 1] must be non-zero.
    // /// Intended primarily for constant values. Using it for non-constant values requires a C compiler
    // /// supporting C99.
    // /// void foo (mpz_t x)
    // /// {
    // /// static const mp_limb_t ya[3] = { 0x1, 0x2, 0x3 };
    // /// static const mpz_t y = MPZ_ROINIT_N ((mp_limb_t *) ya, 3);
    // /// mpz_add (x, x, y);
    // /// }
    // mpz_t MPZ_ROINIT_N (mp limb t *xp, mp size t xs)

    // ---------------------------------------------------------------------------------------------
}

pub struct Mpz(mpz_struct);

impl Mpz {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {}
}
