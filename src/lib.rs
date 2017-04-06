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
    domain_m_one: u32,
}

impl Counter {
    /**
     * Low-level constructor.
     * Initializes a new counter with the given length over the given domain.
     * `domain` means how many different values each position can take.
     **/
    pub fn new(domain: u32, len: usize) -> Counter {
        assert!(domain > 0, "domain must be positive, but was {}", domain);
        Counter {
            data: vec![0; len],
            domain_m_one: domain - 1,
        }
    }

    /**
     * Low-level increment operation.
     * Returns whether wrap-around has happened.
     **/
    pub fn inc(&mut self) -> bool {
        for item in self.data.iter_mut() {
            if *item == self.domain_m_one {
                *item = 0;
            } else {
                *item += 1;
                return false;
            }
        }
        true
    }
}

#[test]
#[should_panic(expected="domain must be positive, but was 0")]
fn test_invalid_panics() {
    Counter::new(0, 3);
}

#[test]
fn test_construction() {
    let c = Counter::new(4, 3);
    assert_eq!(c.domain_m_one, 3);
    assert_eq!(c.data, vec![0, 0, 0]);
}

#[test]
fn test_increment() {
    let mut c = Counter { data: vec![0, 0, 0], domain_m_one: 3 };
    assert_eq!(false, c.inc());
    assert_eq!(c.data, vec![1, 0, 0]);
}
