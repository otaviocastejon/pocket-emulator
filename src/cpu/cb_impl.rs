//! CB-prefixed instruction implementations.
//! Return value: **extra M-cycles after the CB opcode byte and CB operand byte have been fetched** (2 fetches already counted in `Cpu::step`).

use crate::bus::Bus;
use crate::cpu::helpers::{get_r8, set_r8};
use crate::cpu::Cpu;

pub fn cb_rot(cpu: &mut Cpu, bus: &mut Bus, kind: u8, r: u8) -> u8 {
    let hl = r == 6;
    let v = get_r8(cpu, bus, r);
    let out = match kind {
        0 => {
            // RLC
            let c = (v & 0x80) != 0;
            let r = v.rotate_left(1);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        1 => {
            // RRC
            let c = (v & 1) != 0;
            let r = v.rotate_right(1);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        2 => {
            // RL
            let c = (v & 0x80) != 0;
            let r = (v << 1) | (cpu.regs.flag_c() as u8);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        3 => {
            // RR
            let c = (v & 1) != 0;
            let r = (v >> 1) | ((cpu.regs.flag_c() as u8) << 7);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        4 => {
            // SLA
            let c = (v & 0x80) != 0;
            let r = v << 1;
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        5 => {
            // SRA (arithmetic)
            let c = (v & 1) != 0;
            let r = (v >> 1) | (v & 0x80);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        6 => {
            // SWAP
            let r = v.rotate_left(4);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(false);
            r
        }
        7 => {
            // SRL
            let c = (v & 1) != 0;
            let r = v >> 1;
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(c);
            r
        }
        _ => v,
    };
    set_r8(cpu, bus, r, out);
    if hl {
        2
    } else {
        0
    }
}

pub fn cb_bit(cpu: &mut Cpu, bus: &mut Bus, bit: u8, r: u8) -> u8 {
    let v = get_r8(cpu, bus, r);
    let z = (v & (1 << bit)) == 0;
    cpu.regs.set_z(z);
    cpu.regs.set_n(false);
    cpu.regs.set_h(true);
    if r == 6 {
        1
    } else {
        0
    }
}

pub fn cb_res(cpu: &mut Cpu, bus: &mut Bus, bit: u8, r: u8) -> u8 {
    let v = get_r8(cpu, bus, r) & !(1 << bit);
    set_r8(cpu, bus, r, v);
    if r == 6 {
        2
    } else {
        0
    }
}

pub fn cb_set(cpu: &mut Cpu, bus: &mut Bus, bit: u8, r: u8) -> u8 {
    let v = get_r8(cpu, bus, r) | (1 << bit);
    set_r8(cpu, bus, r, v);
    if r == 6 {
        2
    } else {
        0
    }
}
