/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate numeral;

use numeral::Numeral;
use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! test_call_on_min_max {
    ($fn_name: ident, $numtype: ty) => (
        #[test]
        fn $fn_name() {
            <$numtype>::max_value().ordinal();
            <$numtype>::min_value().ordinal();
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

macro_rules! test_call_on_range {
    ($fn_name: ident, $numtype: ty) => {
        #[test]
        fn $fn_name() {
            for n in (<$numtype>::min_value())..(<$numtype>::max_value()) {
                n.ordinal();
            }
            // waiting for inclusive ranges
            <$numtype>::max_value().ordinal();
        }
    }
}

test_call_on_range!(call_on_range_i8, i8);
test_call_on_range!(call_on_range_u8, u8);
test_call_on_range!(call_on_range_i16, i16);
test_call_on_range!(call_on_range_u16, u16);

macro_rules! test_call_on_critical_ranges {
    ($fn_name: ident, $numtype: ty) => (
        #[test]
        fn $fn_name() {
            for n in (<$numtype>::min_value())..(<$numtype>::min_value()) + 130 {
                n.ordinal();
            }
            for n in (<$numtype>::max_value()) - 130..(<$numtype>::max_value()) {
                n.ordinal();
            }
            // waiting for inclusive ranges
            <$numtype>::max_value().ordinal();
            if <$numtype>::min_value() != 0 {
                for n in -130..130 {
                    n.ordinal();
                }
            }
        }
    )
}

test_call_on_critical_ranges!(call_on_critical_ranges_i32, i32);
test_call_on_critical_ranges!(call_on_critical_ranges_u32, u32);
test_call_on_critical_ranges!(call_on_critical_ranges_i64, i64);
test_call_on_critical_ranges!(call_on_critical_ranges_u64, u64);

#[test]
fn ordinal_int_m256_257() {
    let file = File::open("tests/ordinal_m256..257.txt").unwrap();
    assert!(BufReader::new(file).lines()
            .map(|n| n.unwrap())
            .eq((-256..257).map(|n| n.ordinal())));
}

#[test]
fn ordinal_int_min_max() {
    let file = File::open("tests/ordinal_min_max.txt").unwrap();
    let mut lines = BufReader::new(file).lines().map(|l| l.unwrap());
    macro_rules! assert_eq_min_max {
        ($signed: ty, $unsigned: ty) => {
            assert_eq!(lines.next().unwrap(), <$signed>::min_value().ordinal());
            assert_eq!(lines.next().unwrap(), <$signed>::max_value().ordinal());
            assert_eq!(lines.next().unwrap(), <$unsigned>::max_value().ordinal());
        }
    }
    assert_eq!(lines.next().unwrap(), 0.ordinal());
    assert_eq_min_max!(i8, u8);
    assert_eq_min_max!(i16, u16);
    assert_eq_min_max!(i32, u32);
    assert_eq_min_max!(i64, u64);
}
