pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: u8,
}

impl Joypad {
    pub const A: u8      = 0b00000001;
    pub const B: u8      = 0b00000010;
    pub const SELECT: u8 = 0b00000100;
    pub const START: u8  = 0b00001000;
    pub const UP: u8     = 0b00010000;
    pub const DOWN: u8   = 0b00100000;
    pub const LEFT: u8   = 0b01000000;
    pub const RIGHT: u8  = 0b10000000;

    pub fn new() -> Self {
        Self {
            strobe: false,
            button_index: 0,
            button_status: 0,
        }
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.button_index = 0
        }
    }

    pub fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }
        let response = (self.button_status & (1 << self.button_index)) >> self.button_index;
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }
        response
    }

    pub fn set_button_pressed_status(&mut self, key: u8, arg: bool) {
        if arg {
            self.button_status |= key;
        } else {
            self.button_status &= !key;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strobe_mode() {
        let mut joypad = Joypad::new();
        joypad.write(1);
        joypad.set_button_pressed_status(Joypad::A, true);
        for _x in 0..10 {
            assert_eq!(joypad.read(), 1);
        }
    }

    #[test]
    fn test_strobe_mode_on_off() {
        let mut joypad = Joypad::new();

        joypad.write(0);
        joypad.set_button_pressed_status(Joypad::RIGHT, true);
        joypad.set_button_pressed_status(Joypad::LEFT, true);
        joypad.set_button_pressed_status(Joypad::SELECT, true);
        joypad.set_button_pressed_status(Joypad::B, true);

        for _ in 0..=1 {
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 1);

            for _x in 0..10 {
                assert_eq!(joypad.read(), 1);
            }
            joypad.write(1);
            joypad.write(0);
        }
    }
}