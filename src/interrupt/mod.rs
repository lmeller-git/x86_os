mod handlers;
mod idt;
mod pic;

pub fn init() {
    idt::init();
}
