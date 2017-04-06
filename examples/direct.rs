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

extern crate count_array;

fn main() {
    let mylength = 2;
    let myelements = 3;
    let mut counter = count_array::over(myelements, mylength);
    while let Some(myslice) = counter.next() {
        println!("Found: {:?}", &myslice);
    }
    println!("Done!");
}
