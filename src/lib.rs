pub mod cli;
pub mod keys;
pub mod newkeys;
pub mod init;

use keys::Key;
use sdl2::keyboard::Scancode;
use serialport::SerialPort;
use std::collections::HashSet;

pub struct KbReport {
    modifiers: u8,
    pressed: HashSet<u8>,
    port: Box<dyn SerialPort>,

}

impl KbReport {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            modifiers: 0,
            pressed: HashSet::new(),
            port
        }
    }

    fn send(&mut self) -> std::io::Result<()> {
        let mut buffer = [0_u8; 7];
        buffer[0] = self.modifiers;
        for (i, &k) in self.pressed.iter().enumerate() {
            buffer[i+1] = k;
        }
        log::debug!("modifiers: {:08b}", self.modifiers);
        log::debug!("pressed: {:?}", &buffer[1..7]);
        self.port.write_all(&buffer)
    }

    pub fn press(&mut self, key: Scancode) -> std::io::Result<()> {
        match key.into() {
            Key::Modifier(m) => {
                self.modifiers |= m;
            }
            Key::K(k) => {
                if self.pressed.len() >= 6 { return Ok(()) }
                self.pressed.insert(k);
            }
        }
        self.send()
    }

    pub fn release(&mut self, key: Scancode) -> std::io::Result<()> {
        match key.into() {
            Key::Modifier(m) => {
                self.modifiers &= !m;
            }
            Key::K(k) => {
                self.pressed.remove(&k);
            }
        }
        self.send()
    }

    pub fn release_all(&mut self) -> std::io::Result<()> {
        self.modifiers = 0;
        self.pressed.clear();
        self.send()
    }


}