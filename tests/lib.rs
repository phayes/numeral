/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![feature(inclusive_range_syntax)]

extern crate numeral;

use numeral::Cardinal;
use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! test_call_on_min_max {
    ($fn_name:ident, $numtype:ty) => (
        #[test]
        fn $fn_name() {
            <$numtype>::max_value().cardinal();
            <$numtype>::min_value().cardinal();
        }
    )
}

test_call_on_min_max!(call_on_min_max_i8, i8);
test_call_on_min_max!(call_on_min_max_u8, u8);
test_call_on_min_max!(call_on_min_max_i16, i16);
test_call_on_min_max!(call_on_min_max_u16, u16);
test_call_on_min_max!(call_on_min_max_i32, i32);
test_call_on_min_max!(call_on_min_max_u32, u32);
test_call_on_min_max!(call_on_min_max_i64, i64);
test_call_on_min_max!(call_on_min_max_u64, u64);
test_call_on_min_max!(call_on_min_max_isize, isize);
test_call_on_min_max!(call_on_min_max_usize, usize);

macro_rules! test_call_on_range {
    ($fn_name:ident, $numtype:ty) => (
        #[test]
        fn $fn_name() {
            for n in (<$numtype>::min_value())..=(<$numtype>::max_value()) {
                n.cardinal();
            }
        }
    )
}

test_call_on_range!(call_on_range_i8, i8);
test_call_on_range!(call_on_range_u8, u8);
test_call_on_range!(call_on_range_i16, i16);
test_call_on_range!(call_on_range_u16, u16);

macro_rules! test_call_on_critical_ranges {
    ($fn_name:ident, $numtype:ty) => (
        #[test]
        fn $fn_name() {
            for n in (<$numtype>::min_value())..=(<$numtype>::min_value()) + 130 {
                n.cardinal();
            }
            for n in (<$numtype>::max_value()) - 130..=(<$numtype>::max_value()) {
                n.cardinal();
            }
            if <$numtype>::min_value() != 0 {
                for n in -130..=130 {
                    n.cardinal();
                }
            }
        }
    )
}

test_call_on_critical_ranges!(call_on_critical_ranges_i32, i32);
test_call_on_critical_ranges!(call_on_critical_ranges_u32, u32);
test_call_on_critical_ranges!(call_on_critical_ranges_i64, i64);
test_call_on_critical_ranges!(call_on_critical_ranges_u64, u64);
test_call_on_critical_ranges!(call_on_critical_ranges_isize, isize);
test_call_on_critical_ranges!(call_on_critical_ranges_usize, usize);

#[test]
fn cardinal_value_m256_256() {
    let file = File::open("tests/cardinal_m256..=256.txt").unwrap();
    assert!(BufReader::new(file).lines()
            .map(|n_str| n_str.unwrap())
            .eq((-256..=256).map(|n: i32| n.cardinal())));
}

#[test]
fn cardinal_value_min_max_int() {
    let file = File::open("tests/cardinal_min_max.txt").unwrap();
    let mut lines = BufReader::new(file).lines().map(|n_str| n_str.unwrap());
    macro_rules! assert_eq_min_max {
        ($signed:ty, $unsigned:ty) => (
            assert_eq!(lines.next().unwrap(), <$signed>::min_value().cardinal());
            assert_eq!(lines.next().unwrap(), <$signed>::max_value().cardinal());
            assert_eq!(lines.next().unwrap(), <$unsigned>::max_value().cardinal());
        )
    }
    assert_eq!(lines.next().unwrap(), 0.cardinal());
    assert_eq_min_max!(i8, u8);
    assert_eq_min_max!(i16, u16);
    assert_eq_min_max!(i32, u32);
    assert_eq_min_max!(i64, u64);
}

#[test]
fn cardinal_value_min_max_ptr() {
    use std::mem::size_of;

    macro_rules! assert_eq_min_max_if_ptr_is {
        ($ptr:ty, $int:ty) => (
            if size_of::<$ptr>() == size_of::<$int>() {
                assert_eq!(<$ptr>::min_value().cardinal(), <$int>::min_value().cardinal());
                assert_eq!(<$ptr>::max_value().cardinal(), <$int>::max_value().cardinal());
            }
        )
    }
    assert_eq_min_max_if_ptr_is!(isize, i8);
    assert_eq_min_max_if_ptr_is!(isize, i16);
    assert_eq_min_max_if_ptr_is!(isize, i32);
    assert_eq_min_max_if_ptr_is!(isize, i64);
    assert_eq_min_max_if_ptr_is!(usize, u8);
    assert_eq_min_max_if_ptr_is!(usize, u16);
    assert_eq_min_max_if_ptr_is!(usize, u32);
    assert_eq_min_max_if_ptr_is!(usize, u64);
}
