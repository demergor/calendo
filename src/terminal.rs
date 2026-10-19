use std::{
    io::{
        self,
        Write
    },
    mem::MaybeUninit
};

const ENTER_ALT_SCREEN: &str = "\x1b[?1049h";
const LEAVE_ALT_SCREEN: &str = "\x1b[?1049l";

struct Terminal {
    width: u16,
    height: u16,
    og_termios: libc::termios,
}

impl Terminal {
    pub fn new() -> Result<Terminal, TermPrepError> {
        let (width, height) = term_dimensions()?;
        let og_termios = term_attrs()?;
        let mut raw = og_termios.clone();

        unsafe { libc::cfmakeraw(&mut raw) };
        std::io::stdout().write_all(ENTER_ALT_SCREEN.as_bytes());

        Ok(Terminal {
            width,
            height,
            og_termios,
        })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        todo!("Implement raw terminal destructor");

    }
}

enum TermPrepError {
    BadTermSz,
    Io(io::Error),
}

fn term_attrs() -> Result<libc::termios, TermPrepError> {

}

fn term_dimensions() -> Result<(u16, u16), TermPrepError> {
    let mut winsz = MaybeUninit::<libc::winsize>::uninit();
    let result = unsafe {
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, winsz.as_mut_ptr())
    };

    if result == -1 {
        return Err(TermPrepError::Io(io::Error::last_os_error()));
    }

    let winsz = unsafe {winsz.assume_init() };
    if winsz.ws_col <= 0 || winsz.ws_row <= 0 {
        return Err(TermPrepError::BadTermSz);
    }

    Ok((winsz.ws_col, winsz.ws_row))
}
