use super::addressing;
use super::{Six502, STACK_OFFSET};
use crate::bus::{ByteAccess, WordAccess};
use std::ops::{Add, AddAssign};

impl Six502 {
    // stack helpers
    pub(super) fn push_u8(&mut self, b: u8) {
        let addr = u16::from(STACK_OFFSET + self.s as u16);
        self.store_u8(addr, b);
        self.s.wrapping_sub(1);
    }

    pub(super) fn pull_u8(&mut self) -> u8 {
        let addr = u16::from(STACK_OFFSET + self.s as u16) + 1;
        let v = self.load_u8(addr);
        self.s.wrapping_add(1);
        v
    }

    pub(super) fn push_u16(&mut self, w: u16) {
        let addr = u16::from(STACK_OFFSET + (self.s - 1) as u16);
        self.store_u16(addr, w);
        self.s.wrapping_sub(2);
    }

    pub(super) fn pull_u16(&mut self) -> u16 {
        let addr = u16::from(STACK_OFFSET + self.s as u16) + 1;
        let v = self.load_u16(addr);
        self.s.wrapping_add(2);
        v
    }

    pub(super) fn nop(&self) -> bool {
        false
    }
}

// pub fn load_u8(&mut self, addr: u16) -> u8 {
//     match addr {
//         0x000..=0x1fff => self.ram.load_u8(addr),
//         0x2000..=0x3fff => todo!("ppu"),
//         0x4015 => todo!("apu"),
//         0x4016 => todo!("controller"),
//         0x4018 => todo!("apu"),
//         0x4020..=0xffff => todo!("mapper"),
//         _ => panic!("invalid load from: {:02x}", addr),
//     }
// }

// pub fn load_u16(&mut self, addr: u16) -> u16 {
//     u16::from_le_bytes([self.load_u8(addr), self.load_u8(addr + 1)])
// }

// pub fn load_u16_no_carry(&self, addr: u8) -> u16 {
//     u16::from_le_bytes([self.load_u8(addr as u16), self.load_u8(addr as u16)])
// }

// pub fn store_u8(&mut self, addr: u16, val: u8) {
//     match addr {
//         0x0000..=0x1fff => self.ram.store_u8(addr, val),
//         0x2000..=0x3fff => todo!("ppu"),
//         0x4016 => todo!("controller"),
//         0x4000..=0x4017 => todo!("apu"),
//         0x4020..=0xFFFF => todo!("mapper"),
//         _ => panic!("invalid store to {:02x}", addr),
//     }
// }

// pub fn store_u16(&mut self, addr: u16, val: u16) {
//     self.store_u8(addr, val as u8);
//     self.store_u8(addr + 1, (val >> 8) as u8);
// }

// Source: https://web.archive.org/web/20210428044647/http://www.obelisk.me.uk/6502/reference.html
// pub enum OpCode {
//     ADC,        // add with carry
//     AND,        // logical and
//     ASL,        // Arithmetic shift left
//     BCC = 0x90, // bramch if carry c;ear
//     BCS = 0xb0, // branch if carry set
//     BEQ = 0xf0, // branch if equla
//     BIT,        // bit test
//     BMI = 0x30, // branch if minus
//     BNE = 0xd0, // branch if not equal
//     BPL = 0x10, // branch if positive
//     BRK = 0x00, // force interrupt
//     BVC = 0x50, // branch if overflow clear
//     BVS = 0x70, // branch if overflow set
//     CLC = 0x18, // clear carry flag
//     CLD = 0xd8, // clear decimal node
//     CLI = 0x58, // clear interrupt disable
//     CLV = 0xb8, // clear overflow flag
//     CMP,        // compare
//     CPX,        // compare x register
//     CPY,        // cmpare y register
//     DEC,        // decrement memory
//     DEX = 0xca, // decrement x register
//     DEY = 0x88, // decrement y register
//     EOR,        // exclusive or
//     INC,        // increment memory
//     INX = 0xe8, // increment x register
//     INY = 0xc8, // increment y register
//     JMP = 0x4c, // jump
//     JSR = 0x20, // jump to subroutine
//     LDA,        // load accumulator
//     LDX,        // load x register
//     LDY,        // load y register
//     LSR,        // logical shift right
//     NOP = 0xEA, // no-op
//     ORA,        // logical inclusive or
//     PHA = 0x48, // push accumulator
//     PHP = 0x08, // push processor status
//     PLA = 0x68, // pull accumulator
//     PLP = 0x28, // pull processor status
//     ROL,        // rotate left
//     ROR,        // rotate right
//     RTI = 0x40, // return from interrupt
//     RTS = 0x60, // return from subroutine
//     SBC,        // subtract with carry
//     SEC = 0x38, // set carry flag
//     SED = 0xf8, // set decimal flag
//     SEI = 0x78, // set interrupt disable
//     STA,        // store accumulator
//     STX,        // store x register
//     STY,        // store y register
//     TAX = 0xaa, // transfer accumulator to x
//     TAY = 0xa8, // transfer accumulator to y
//     TSX = 0xba, // transfer stack pointer to x
//     TXA = 0x8a, // transfer x to accumulator
//     TXS = 0x9a, // transfer x to stack pointer
//     TYA = 0x98, // transfer y to accumulator
// }
