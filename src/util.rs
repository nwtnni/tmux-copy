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

macro_rules! command {
    ($name:expr, $($arg:expr),* $(,)?) => {
        {
            let mut command = process::Command::new($name);
            $(command.arg($arg);)*
            command
        }
    }
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
