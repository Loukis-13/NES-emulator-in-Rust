// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Show background
// |||+------ 1: Show sprites
// ||+------- Emphasize red (green on PAL/Dendy)
// |+-------- Emphasize green (red on PAL/Dendy)
// +--------- Emphasize blue

// const GRAYSCALE: u8 = 0b0000_0001;
// const LEFTMOST_BACKGROUND: u8 = 0b0000_0010;
// const LEFTMOST_SPRITES: u8 = 0b0000_0100;
// const SHOW_BACKGROUND: u8 = 0b0000_1000;
const SHOW_SPRITES: u8 = 0b0001_0000;
// const EMPHASISE_RED: u8 = 0b0010_0000;
// const EMPHASISE_GREEN: u8 = 0b0100_0000;
// const EMPHASISE_BLUE: u8 = 0b1000_0000;

pub struct MaskRegister {
    bits: u8,
}

impl MaskRegister {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

    // fn set_bit(&mut self, flag: u8, set: bool) {
    //     self.bits = if set {
    //         self.bits | flag
    //     } else {
    //         self.bits & !flag
    //     };
    // }

    pub fn show_sprites(&self) -> bool {
        self.bits & SHOW_SPRITES != 0
    }
}
