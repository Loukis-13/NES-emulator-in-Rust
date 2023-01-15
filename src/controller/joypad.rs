#[derive(Clone, Copy)]
pub enum JoypadButtons {
    A      = 0b00000001,
    B      = 0b00000010,
    SELECT = 0b00000100,
    START  = 0b00001000,
    UP     = 0b00010000,
    DOWN   = 0b00100000,
    LEFT   = 0b01000000,
    RIGHT  = 0b10000000,
}

pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: u8,
}

impl Joypad {
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

    pub fn set_button_pressed_status(&mut self, key: JoypadButtons, arg: bool) {
        let key = key as u8;

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
        joypad.set_button_pressed_status(JoypadButtons::A, true);
        for _x in 0..10 {
            assert_eq!(joypad.read(), 1);
        }
    }

    #[test]
    fn test_strobe_mode_on_off() {
        let mut joypad = Joypad::new();

        joypad.write(0);
        joypad.set_button_pressed_status(JoypadButtons::RIGHT, true);
        joypad.set_button_pressed_status(JoypadButtons::LEFT, true);
        joypad.set_button_pressed_status(JoypadButtons::SELECT, true);
        joypad.set_button_pressed_status(JoypadButtons::B, true);

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