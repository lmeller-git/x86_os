use core::fmt;

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

pub struct PrintBuilder<'a> {
    args: fmt::Arguments<'a>,
    bg: crate::io::Color,
    fg: crate::io::Color,
}
impl<'a> PrintBuilder<'a> {
    pub fn new(args: fmt::Arguments<'a>) -> Self {
        Self {
            args,
            bg: crate::io::Color::Black,
            fg: crate::io::Color::White,
        }
    }

    pub fn bg(mut self, bg: crate::io::Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn fg(mut self, fg: crate::io::Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn print(&self) {
        crate::io::_print_with_args(self.args, self.bg, self.fg);
    }
}
