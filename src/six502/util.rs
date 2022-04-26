use super::addressing;
use super::Six502;
use std::ops::AddAssign;

impl Six502 {
    //sets the zero and negative flags as is appropriate
    fn update_flags_lda(&mut self, v: u8) {
        if self.x == 0 {
            self.p.add_assign(0b0000_0010);
        } else {
            self.p.add_assign(0b1111_1101);
        };

        if self.a & 0b1000_0000 != 0 {
            // MSB is set
            self.p.add_assign(0b1000_0000);
        } else {
            self.p.add_assign(0b0111_1111);
        };
    }
}