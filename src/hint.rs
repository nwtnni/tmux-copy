use crate::util;

const SHORT: [&str; 10] = ["a", "s", "d", "f", "g", "h", "j", "k", "l", ";"];
const LONG: [[&str; 10]; 10] = cartesian!("a", "s", "d", "f", "g", "h", "j", "k", "l", ";");

pub fn hints(count: usize) -> util::Or<Short, Long> {
    match count {
    |  0 ..=  10 => util::Or::L(Short(0)),
    | 10 ..= 100 => util::Or::R(Long(0)),
    | _ => unimplemented!(),
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Short(usize);

impl Iterator for Short {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        let next = &SHORT[self.0];
        self.0 += 1;
        Some(next)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Long(usize);

impl Iterator for Long {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        let next = &LONG[self.0 / 10][self.0 % 10];
        self.0 += 1;
        Some(next)
    }
}
