// count_array – Count an array as if it was a number
// Copyright (C) 2017  Ben Wiederhake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(test)]

extern crate count_array;
extern crate test;

use test::Bencher;

#[bench]
fn bench_100000(b: &mut Bencher) {
    // Iterates over 10^5 steps.
    b.iter(|| {
        let mut i = 0;
        let mut counter = count_array::over(9, 5);
        while {
            i += 1;
            !counter.inc()
        } { }
        assert_eq!(i, 100_000);
    });
}

#[bench]
fn bench_001024(b: &mut Bencher) {
    // Iterates over 32^2 steps.
    b.iter(|| {
        let mut i = 0;
        let mut counter = count_array::over(31, 2);
        while {
            i += 1;
            !counter.inc()
        } { }
        assert_eq!(i, 1024);
    });
}

#[bench]
fn bench_000027(b: &mut Bencher) {
    // Iterates over 3^3 steps.
    b.iter(|| {
        let mut i = 0;
        let mut counter = count_array::over(2, 3);
        while {
            i += 1;
            !counter.inc()
        } { }
        assert_eq!(i, 27);
    });
}
