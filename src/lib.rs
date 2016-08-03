/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! A Rust library providing the written english form of a number.
//!
//! # Example
//!
//! ```rust
//! use numeral::Numeral;
//!
//! let n = 127;
//! println!("{} is written: {}", n, n.ordinal());
//! ```


#![warn(missing_docs)]

const NUMBER: [&'static str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&'static str; 10] = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

const MULTIPLIER: [&'static str; 9] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
];

/// The `Numeral` trait provides the written form of a number.
pub trait Numeral {
    /// Yields the ordinal form of a number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use numeral::Numeral;
    /// let written_form = 127.ordinal();
    /// assert_eq!(written_form, "one hundred twenty-seven");
    /// ```
    fn ordinal(&self) -> String;
}

macro_rules! impl_numeral_signed {
    ($($numtype: ty),*) => ($(
        impl Numeral for $numtype {
            fn ordinal(&self) -> String {
                ordinal_int(i64::from(*self).abs() as u64, *self < 0).concat()
            }
        }
    )*)
}

impl_numeral_signed!(i8, i16, i32);

impl Numeral for i64 {
    fn ordinal(&self) -> String {
        let n_abs = if *self == i64::min_value() { *self as u64 } else { self.abs() as u64 };
        ordinal_int(n_abs, *self < 0).concat()
    }
}

macro_rules! impl_numeral_unsigned {
    ($($numtype: ty),*) => ($(
        impl Numeral for $numtype {
            fn ordinal(&self) -> String {
                ordinal_int(u64::from(*self), false).concat()
            }
        }
    )*)
}

impl_numeral_unsigned!(u8, u16, u32, u64);

/// Returns the written form of any 64-bit integer
/// as a vector of strings.
#[inline]
fn ordinal_int(n: u64, negative: bool) -> Vec<&'static str> {
    if n == 0 { return vec![NUMBER[0]] }
    let multiple_order = ((n as f32).log10() as u32) / 3;
    let max_len = multiple_order as usize * 8 + 6;
    let mut numeral = Vec::with_capacity(max_len);
    if negative { numeral.push("minus "); }
    compose_ordinal_int(n, multiple_order, &mut numeral);
    debug_assert!(numeral.len() <= max_len);
    numeral
}

macro_rules! push {
    ($table: ident[$index: ident] on $vec: ident) => (
        debug_assert!(($index as usize) < $table.len(), format!("{} out of {}'s range", stringify!($index), stringify!($table)));
        $vec.push($table[$index as usize]);
    )
}

/// Pushes the strings composing the ordinal form of any unsigned 64-bit number
/// on a vector. Zero is ignored.
#[inline]
fn compose_ordinal_int(n: u64, multiple_order: u32, numeral: &mut Vec<&'static str>) {
    debug_assert!(multiple_order == ((n as f32).log10() as u32) / 3, "wrong value for multiple_order in compose_ordinal_int()");
    let (mut n, mut multiple_order) = (n, multiple_order);
    let mut multiplier = 10u64.pow(multiple_order * 3);
    let mut multiplicand;
    if multiple_order > 0 {
        loop {
            multiplicand = n / multiplier;
            n %= multiplier;
            if multiplicand != 0 {
                push_triplet(multiplicand, numeral);
                numeral.push(" ");
                push!(MULTIPLIER[multiple_order] on numeral);
                if n != 0 { numeral.push(" "); }
            }
            multiple_order -= 1;
            if multiple_order == 0 { break }
            multiplier = multiplier / 1000;
        }
    }
    push_triplet(n, numeral);
}

/// Takes a three-digit integer (n in [1,999]) and adds it's written form
/// to a numeral in construction. Zero is ignored.
#[inline]
fn push_triplet(n: u64, numeral: &mut Vec<&'static str>) {
    debug_assert!(n < 1000, "n >= 1000 in push_triplet()");
    let hundreds = n / 100;
    let rest = n % 100;
    if hundreds != 0 {
        push!(NUMBER[hundreds] on numeral);
        if rest == 0 { numeral.push(" hundred"); }
        else { numeral.push(" hundred "); }
    }
    push_doublet(rest, numeral);
}

/// Takes a two-digit integer (n in [1,99]) and adds it's written form
/// to a numeral in construction. Zero is ignored.
#[inline]
fn push_doublet(n: u64, numeral: &mut Vec<&'static str>) {
    debug_assert!(n < 100, "n >= 100 in push_doublet()");
    if n == 0 { return }
    else if n < 20  {
        push!(NUMBER[n] on numeral);
    }
    else {
        let tens = n / 10;
        let ones = n % 10;
        push!(TENS[tens] on numeral);
        if ones != 0 {
            numeral.push("-");
            push!(NUMBER[ones] on numeral);
        }
    }
}
