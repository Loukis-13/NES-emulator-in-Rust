// 7  bit  0
// ---- ----
// VSO. ....
// |||| ||||
// |||+-++++- PPU open bus. Returns stale PPU bus contents.
// ||+------- Sprite overflow. The intent was for this flag to be set
// ||         whenever more than eight sprites appear on a scanline, but a
// ||         hardware bug causes the actual behavior to be more complicated
// ||         and generate false positives as well as false negatives; see
// ||         PPU sprite evaluation. This flag is set during sprite
// ||         evaluation and cleared at dot 1 (the second dot) of the
// ||         pre-render line.
// |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
// |          a nonzero background pixel; cleared at dot 1 of the pre-render
// |          line.  Used for raster timing.
// +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
//            Set at dot 1 of line 241 (the line *after* the post-render
//            line); cleared after reading $2002 and at dot 1 of the
//            pre-render line.

// const SPRITE_OVERFLOW: u8 = 0b0010_0000;
const SPRITE_ZERO_HIT: u8 = 0b0100_0000;
const VBLANK_STARTED: u8  = 0b1000_0000;

pub struct StatusRegister {
    bits: u8,
}

impl StatusRegister {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    fn set_flag(&mut self, flag: u8, status: bool) {
        if status {
            self.bits |= flag;
        } else {
            self.bits &= !flag;
        }
    }

    // pub fn set_sprite_overflow(&mut self, status: bool) {
    //     self.set_flag(SPRITE_OVERFLOW, status);
    // }

    pub fn set_sprite_zero_hit(&mut self, status: bool) {
        self.set_flag(SPRITE_ZERO_HIT, status);
    }

    pub fn set_vblank_status(&mut self, status: bool) {
        self.set_flag(VBLANK_STARTED, status);
    }

    pub fn get_bits(&self) -> u8 {
        self.bits
    }

    pub fn is_in_vblank(&self) -> bool {
        self.bits & VBLANK_STARTED != 0
    }
}
