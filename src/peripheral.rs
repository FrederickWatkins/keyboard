use cortex_m::singleton;
use embedded_hal::digital::*;

use cortex_m_semihosting::{debug, hprintln};

#[derive(Copy, Clone)]
enum GPIOReg {
    MODER = 0x00,
    OTYPER = 0x04,
    OSPEEDR = 0x08,
    PUPDR = 0x0C,
    IDR = 0x10,
    ODR = 0x14,
    BSRR = 0x18,
    LCKR = 0x1C,
    AFRL = 0x20,
    AFRH = 0x24,
    BRR = 0x28,
}

#[derive(Copy, Clone)]
enum PortOffset {
    A = 0x48000000,
    B = 0x48000400,
    C = 0x48000800,
    D = 0x48000C00,
    E = 0x48001000,
    F = 0x48001400,
}

pub enum PinMode {
    GPOut = 0b01,
    GPIn = 0b00,
    AltFunc = 0b10,
    Analogue = 0b11,
}

pub struct Peripherals {
    pub port_a: Port,
    pub port_b: Port,
    pub port_c: Port,
    pub port_d: Port,
    pub port_e: Port,
    pub port_f: Port,
}
impl Peripherals {
    fn new() -> Self {
        Self {
            port_a: Port {
                offset: PortOffset::A,
            },
            port_b: Port {
                offset: PortOffset::B,
            },
            port_c: Port {
                offset: PortOffset::C,
            },
            port_d: Port {
                offset: PortOffset::D,
            },
            port_e: Port {
                offset: PortOffset::E,
            },
            port_f: Port {
                offset: PortOffset::F,
            },
        }
    }
    pub fn take() -> &'static mut Self {
        singleton!(: Peripherals = Self::new()).unwrap()
    }
}

pub struct Port {
    offset: PortOffset,
}

impl Port {
    fn read(&self, register: GPIOReg) -> u32 {
        let p = (register as usize + self.offset as usize) as *const u32;
        unsafe { *p }
    }

    fn write(&mut self, register: GPIOReg, data: u32) {
        let p = (register as usize + self.offset as usize) as *mut u32;
        hprintln!("Wrote {:32b} to address {:8x}", data, p as usize);
        unsafe { *p = data };
    }

    pub fn set_mode(&mut self, modes: [PinMode; 16]) {
        let mut x: u32 = 0;
        for mode in modes {
            x = x | mode as u32;
            x = x.rotate_right(2);
        }
        self.write(GPIOReg::MODER, x);
    }
}
