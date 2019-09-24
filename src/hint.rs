macro_rules! cartesian {
    (@inner) => {};
    (@inner $head:expr, ($($tail:expr),*)) => {
        [$(concat!($head, $tail)),*]
    };
    (@outer) => {};
    (@outer $($iter:expr),* => $all:tt) => {
        [$(cartesian!(@inner $iter, $all)),*]
    };
    ($($all:expr),+ $(,)?) => {
        cartesian!(@outer $($all),+ => ($($all),+))
    };
}

const SHORT: [&str; 10] = ["a", "s", "d", "f", "g", "h", "j", "k", "l", ";"];
const LONG: [[&str; 10]; 10] = cartesian!("a", "s", "d", "f", "g", "h", "j", "k", "l", ";");

pub fn hint(count: usize) -> impl Iterator<Item = &'static str> {
    match count {
    |  0 ..=  10 => Or::L(Short(0)),
    | 10 ..= 100 => Or::R(Long(0)),
    | _ => unimplemented!(),
    }
}

enum Or<L, R> {
    L(L),
    R(R),
}

impl<L, R, T> Iterator for Or<L, R>
    where L: Iterator<Item = T>,
          R: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
        | Or::L(l) => l.next(),
        | Or::R(r) => r.next(),
        }
    }
}

struct Short(usize);

impl Iterator for Short {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        let next = &SHORT[self.0];
        self.0 += 1;
        Some(next)
    }
}

struct Long(usize);

impl Iterator for Long {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        let next = &LONG[self.0 / 10][self.0 % 10];
        self.0 += 1;
        Some(next)
    }
}
