use core::fmt;

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));

}

#[derive(Debug)]
pub struct PrintBuilder<'a> {
    args: fmt::Arguments<'a>,
    bg: crate::io::Color,
    fg: crate::io::Color,
}

#[allow(dead_code)]
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

impl<'a> PartialEq for PrintBuilder<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.bg.eq(&other.bg) && self.fg.eq(&other.fg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    #[test_case]
    fn test_print() -> Result<()> {
        println!();
        print!();
        print!("test2");
        println!("test");
        Ok(())
    }

    #[test_case]
    fn test_print_builder() -> Result<()> {
        let p = PrintBuilder::new(format_args!(""))
            .bg(crate::io::Color::Black)
            .fg(crate::io::Color::White);
        assert_eq!(
            p,
            PrintBuilder {
                args: format_args!(""),
                bg: crate::io::Color::Black,
                fg: crate::io::Color::White
            }
        );
        p.print();
        Ok(())
    }
}
