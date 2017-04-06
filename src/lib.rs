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

/// A `Counter` contains the state of the counting, and doubles as in `Iterator`.
pub struct Counter {
    data: Vec<u32>,
    max: u32,
}

impl Counter {
    /**
     * Constructor.
     * Initializes a new counter with the given length over the given domain,
     * starting at the lowest value.
     * `max` represents the maximum value of a place.
     **/
    pub fn new(max: u32, len: usize) -> Counter {
        Counter {
            data: vec![0; len],
            max: max,
        }
    }

    /**
     * Low-level increment operation.
     * Returns whether wrap-around has happened.
     **/
    pub fn inc(&mut self) -> bool {
        for item in self.data.iter_mut() {
            if *item == self.max {
                *item = 0;
            } else {
                *item += 1;
                return false;
            }
        }
        true
    }

    /**
     * Low-level read access to current state.
     */
    pub fn read(&self) -> &[u32] {
        self.data.as_slice()
    }
}

/**
 * Constructor, calls `Counter::new`.
 * Initializes a new counter with the given length over the given domain,
 * starting at the lowest value.
 * `max` represents the maximum value of a place.
 **/
pub fn over(max: u32, len: usize) -> Counter {
    Counter::new(max, len)
}

#[test]
fn test_construction() {
    let c = Counter::new(3, 3);
    assert_eq!(c.max, 3);
    assert_eq!(c.data, vec![0, 0, 0]);
}

#[test]
fn test_construction_more() {
    let c = Counter::new(99, 3);
    assert_eq!(c.max, 99);
    assert_eq!(c.data, vec![0, 0, 0]);
}

#[test]
fn test_increment() {
    let mut c = Counter { data: vec![0, 0, 0], max: 3 };
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![1, 0, 0]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![2, 0, 0]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![3, 0, 0]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![0, 1, 0]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![1, 1, 0]);
}

#[test]
fn test_increment_end() {
    let mut c = Counter { data: vec![3, 2, 3], max: 3 };
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![0, 3, 3]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![1, 3, 3]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![2, 3, 3]);
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![3, 3, 3]);
    assert_eq!(true, c.inc());
    assert_eq!(c.data, vec![0, 0, 0]);
}
