use std::io;
use std::mem;

use crate::ansi;

use std::io::Read;
use std::io::Write;

pub struct Term<'main> {
    termios: libc::termios,
    stdin: io::StdinLock<'main>,
    stdout: io::BufWriter<io::StdoutLock<'main>>,
    buffer: [u8; 1],
}

impl<'main> Term<'main> {
    pub fn new(stdin: &'main mut io::Stdin, stdout: &'main mut io::Stdout) -> io::Result<Self> {
        let termios = unsafe {
            // Ensure that we have a tty device
            if libc::isatty(libc::STDIN_FILENO) != 1 || libc::isatty(libc::STDOUT_FILENO) != 1 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "[USER ERROR]: not a TTY",
                ));
            }

            // Get current settings
            let mut termios: libc::termios = mem::zeroed();
            test!(libc::tcgetattr(libc::STDIN_FILENO, &mut termios));

            // Change to non-canonical mode
            let mut set = termios;
            set.c_lflag &= !(libc::ICANON | libc::ECHO);
            set.c_cc[libc::VMIN] = 1;
            set.c_cc[libc::VTIME] = 0;
            test!(libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &set));

            // Save for restoring later
            termios
        };

        // Hold onto locks
        let stdin = stdin.lock();
        let mut stdout = io::BufWriter::new(stdout.lock());
        write!(stdout, "{}{}", ansi::HIDE, ansi::CLEAR)?;
        Ok(Term {
            termios,
            stdin,
            stdout,
            buffer: [0],
        })
    }

    pub fn read(&mut self) -> io::Result<char> {
        self.stdin.read_exact(&mut self.buffer)?;
        Ok(self.buffer[0] as char)
    }
}

impl<'main> io::Write for Term<'main> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl<'main> Drop for Term<'main> {
    /// Restore initial termios settings and clear the screen.
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.termios);
            write!(self.stdout, "{}{}", ansi::RESET, ansi::SHOW).ok();
            self.stdout.flush().ok();
        }
    }
}
