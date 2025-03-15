pub mod io;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
