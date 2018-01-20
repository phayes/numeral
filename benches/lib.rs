/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![feature(test)]
#![feature(inclusive_range_syntax)]

extern crate numeral;
extern crate test;

use numeral::Cardinal;
use test::Bencher;

macro_rules! bench_call_on_range {
    ($fn_name:ident, $numtype:ty) => (
        #[bench]
        fn $fn_name(b: &mut Bencher) {
            b.iter(|| {
                for n in (<$numtype>::min_value())..=(<$numtype>::max_value()) {
                    n.cardinal();
                }
            })
        }
    )
}

bench_call_on_range!(bench_call_on_range_i8, i8);
bench_call_on_range!(bench_call_on_range_u8, u8);

macro_rules! bench_call_on_min_max {
    ($fn_name:ident, $numtype:ty) => (
        #[bench]
        fn $fn_name(b: &mut Bencher) {
            b.iter(|| {
                <$numtype>::max_value().cardinal();
                <$numtype>::min_value().cardinal();
            })
        }
    )
}

bench_call_on_min_max!(bench_call_on_min_max_i8, i8);
bench_call_on_min_max!(bench_call_on_min_max_u8, u8);
bench_call_on_min_max!(bench_call_on_min_max_i16, i16);
bench_call_on_min_max!(bench_call_on_min_max_u16, u16);
bench_call_on_min_max!(bench_call_on_min_max_i32, i32);
bench_call_on_min_max!(bench_call_on_min_max_u32, u32);
bench_call_on_min_max!(bench_call_on_min_max_i64, i64);
bench_call_on_min_max!(bench_call_on_min_max_u64, u64);
bench_call_on_min_max!(bench_call_on_min_max_isize, isize);
bench_call_on_min_max!(bench_call_on_min_max_usize, usize);
