use x86_64::instructions::port::Port;

const SCANCODE_TO_ASCII: [Option<u8>; 58] = [
    None,
    None,
    Some(b'1'),
    Some(b'2'),
    Some(b'3'),
    Some(b'4'),
    Some(b'5'),
    Some(b'6'),
    Some(b'7'),
    Some(b'8'),
    Some(b'9'),
    Some(b'0'),
    Some(b'-'),
    Some(b'='),
    None,
    None,
    Some(b'q'),
    Some(b'w'),
    Some(b'e'),
    Some(b'r'),
    Some(b't'),
    Some(b'y'),
    Some(b'u'),
    Some(b'i'),
    Some(b'o'),
    Some(b'p'),
    Some(b'['),
    Some(b']'),
    None,
    None,
    Some(b'a'),
    Some(b's'),
    Some(b'd'),
    Some(b'f'),
    Some(b'g'),
    Some(b'h'),
    Some(b'j'),
    Some(b'k'),
    Some(b'l'),
    Some(b';'),
    Some(b'\''),
    Some(b'`'),
    Some(b'\n'),
    Some(b'\\'),
    Some(b'z'),
    Some(b'x'),
    Some(b'c'),
    Some(b'v'),
    Some(b'b'),
    Some(b'n'),
    Some(b'm'),
    Some(b','),
    Some(b'.'),
    Some(b'/'),
    None,
    None,
    None,
    Some(b' '), // Space bar
];

fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    if scancode & 0x80 != 0 {
        return None;
    }
    if scancode < 58 {
        match scancode {
            0x1C => Some(b'\n'),   // Enter
            0x2A | 0x36 => None,   // Ignore Shift (handled separately)
            0x0E => Some(b'\x08'), // Backspace
            _ => SCANCODE_TO_ASCII.get(scancode as usize).copied().flatten(),
        }
    } else {
        None
    }
}

fn get_key() -> u8 {
    let mut port = Port::new(0x60);
    let mut status_port = Port::new(0x64);
    loop {
        let status: u8 = unsafe { status_port.read() };
        if status & 0x01 != 0 {
            break;
        }
    }

    unsafe { port.read() }
}

pub fn read_one() -> Option<u8> {
    scancode_to_ascii(get_key())
}

pub fn read_line(buffer: &mut [u8]) {
    for item in buffer.iter_mut() {
        if let Some(k) = scancode_to_ascii(get_key()) {
            if k == b'\n' {
                return;
            }
            *item = k;
        }
    }
}
