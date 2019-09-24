use std::io;
use std::mem;

use crate::ansi;

pub struct Term<'main> {
    termios: libc::termios,
    stdin: io::StdinLock<'main>,
    stdout: io::StdoutLock<'main>,
    buffer: [u8; 1],
}

macro_rules! test {
    ($call:expr) => {
        if $call != 0 { return Err(io::Error::last_os_error()) }
    }
}

impl<'main> Term<'main> {

    pub fn new(stdin: &'main mut io::Stdin, stdout: &'main mut io::Stdout) -> io::Result<Self> {

        let termios = unsafe {

            // Ensure that we have a tty device
            if libc::isatty(libc::STDIN_FILENO) != 1
            || libc::isatty(libc::STDOUT_FILENO) != 1 {
                return Err(io::Error::new(io::ErrorKind::Other, "[USER ERROR]: not a TTY"))
            }

            // Get current settings
            let mut termios: libc::termios = mem::zeroed();
            test!(libc::tcgetattr(libc::STDIN_FILENO, &mut termios));

            // Change to non-canonical mode
            let mut set = termios.clone();
            set.c_lflag &= !(libc::ICANON | libc::ECHO);
            set.c_cc[libc::VMIN] = 1;
            set.c_cc[libc::VTIME] = 0;
            test!(libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &set));

            // Save for restoring later
            termios
        };

        // Hold onto locks
        let stdin = stdin.lock();
        let mut stdout = stdout.lock();
        use std::io::Write;
        write!(stdout, "{}{}{}", ansi::ALTERNATE, ansi::HIDE, ansi::CLEAR)?;
        Ok(Term { termios, stdin, stdout, buffer: [0] })
    }
}
