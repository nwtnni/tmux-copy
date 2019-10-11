use std::cell;

macro_rules! cartesian {
    (@inner $head:expr, ($($tail:expr),*)) => {
        [$(concat!($head, $tail)),*]
    };
    (@outer $($iter:expr),* => $all:tt) => {
        [$(cartesian!(@inner $iter, $all)),*]
    };
    ($($all:expr),+ $(,)?) => {
        cartesian!(@outer $($all),+ => ($($all),+))
    };
}

macro_rules! command {
    ($name:expr, $($arg:expr),* $(,)?) => {
        {
            let mut command = process::Command::new($name);
            $(command.arg($arg);)*
            command
        }
    }
}

macro_rules! count {
    () => {
        0usize
    };
    ($head:tt $(, $tail:tt)* $(,)?) => {
        1usize + count!($($tail),*) 
    };
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 { return Err(io::Error::last_os_error()) }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Or<L, R> {
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

pub struct Lazy<T: Copy, F: Fn() -> T> {
    thunk: F,
    value: cell::Cell<Option<T>>,
}

impl<T: Copy, F: Fn() -> T> Lazy<T, F> {
    pub fn new(thunk: F) -> Self {
        Lazy {
            thunk,
            value: cell::Cell::new(None),
        }
    }

    pub fn force(&self) -> T {
        match self.value.get() {
        | Some(value) => value,
        | None => {
            self.value.set(Some((self.thunk)()));
            self.force()
        }
        }
    }
}
