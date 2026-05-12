//! Opcode helpers; documented cycles are M-cycles after opcode fetch.

use super::Cpu;
use crate::bus::Bus;

#[inline]
pub fn fetch_u8(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let v = bus.read(cpu.regs.pc);
    cpu.regs.pc = cpu.regs.pc.wrapping_add(1);
    v
}

#[inline]
pub fn fetch_u16(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
    let lo = fetch_u8(cpu, bus) as u16;
    let hi = fetch_u8(cpu, bus) as u16;
    lo | (hi << 8)
}

#[inline]
pub fn cc(cpu: &Cpu, i: u8) -> bool {
    match i {
        0 => !cpu.regs.flag_z(),
        1 => cpu.regs.flag_z(),
        2 => !cpu.regs.flag_c(),
        3 => cpu.regs.flag_c(),
        _ => false,
    }
}

#[inline]
pub fn get_r8(cpu: &mut Cpu, bus: &mut Bus, r: u8) -> u8 {
    match r {
        0 => cpu.regs.b,
        1 => cpu.regs.c,
        2 => cpu.regs.d,
        3 => cpu.regs.e,
        4 => cpu.regs.h,
        5 => cpu.regs.l,
        6 => bus.read(cpu.regs.hl()),
        7 => cpu.regs.a,
        _ => 0,
    }
}

#[inline]
pub fn set_r8(cpu: &mut Cpu, bus: &mut Bus, r: u8, v: u8) {
    match r {
        0 => cpu.regs.b = v,
        1 => cpu.regs.c = v,
        2 => cpu.regs.d = v,
        3 => cpu.regs.e = v,
        4 => cpu.regs.h = v,
        5 => cpu.regs.l = v,
        6 => bus.write(cpu.regs.hl(), v),
        7 => cpu.regs.a = v,
        _ => {}
    }
}

#[inline]
pub fn op_nop() -> u8 {
    0
}

pub fn op_ld_rr(cpu: &mut Cpu, bus: &mut Bus, d: u8, s: u8) -> u8 {
    let v = get_r8(cpu, bus, s);
    set_r8(cpu, bus, d, v);
    if d == 6 || s == 6 {
        1
    } else {
        0
    }
}

pub fn op_ld_r_d8(cpu: &mut Cpu, bus: &mut Bus, d: u8) -> u8 {
    let v = fetch_u8(cpu, bus);
    set_r8(cpu, bus, d, v);
    if d == 6 {
        2
    } else {
        1
    }
}

pub fn op_ld_a_memrr(cpu: &mut Cpu, bus: &mut Bus, pair: u8) -> u8 {
    let addr = match pair {
        0 => cpu.regs.bc(),
        1 => cpu.regs.de(),
        _ => 0,
    };
    cpu.regs.a = bus.read(addr);
    1
}

pub fn op_ld_memrr_a(cpu: &mut Cpu, bus: &mut Bus, pair: u8) -> u8 {
    let addr = match pair {
        0 => cpu.regs.bc(),
        1 => cpu.regs.de(),
        _ => 0,
    };
    bus.write(addr, cpu.regs.a);
    1
}

pub fn op_inc_r(cpu: &mut Cpu, bus: &mut Bus, r: u8) -> u8 {
    if r == 6 {
        let addr = cpu.regs.hl();
        let v = bus.read(addr).wrapping_add(1);
        bus.write(addr, v);
        cpu.regs.set_z(v == 0);
        cpu.regs.set_n(false);
        cpu.regs.set_h((v & 0x0F) == 0);
        2
    } else {
        let v = get_r8(cpu, bus, r).wrapping_add(1);
        set_r8(cpu, bus, r, v);
        cpu.regs.set_z(v == 0);
        cpu.regs.set_n(false);
        cpu.regs.set_h((v & 0x0F) == 0);
        0
    }
}

pub fn op_dec_r(cpu: &mut Cpu, bus: &mut Bus, r: u8) -> u8 {
    if r == 6 {
        let addr = cpu.regs.hl();
        let v = bus.read(addr).wrapping_sub(1);
        bus.write(addr, v);
        cpu.regs.set_z(v == 0);
        cpu.regs.set_n(true);
        cpu.regs.set_h((v & 0x0F) == 0x0F);
        2
    } else {
        let v = get_r8(cpu, bus, r).wrapping_sub(1);
        set_r8(cpu, bus, r, v);
        cpu.regs.set_z(v == 0);
        cpu.regs.set_n(true);
        cpu.regs.set_h((v & 0x0F) == 0x0F);
        0
    }
}

pub fn op_inc_rr(cpu: &mut Cpu, pair: u8) -> u8 {
    match pair {
        0 => cpu.regs.set_bc(cpu.regs.bc().wrapping_add(1)),
        1 => cpu.regs.set_de(cpu.regs.de().wrapping_add(1)),
        2 => cpu.regs.set_hl(cpu.regs.hl().wrapping_add(1)),
        3 => cpu.regs.sp = cpu.regs.sp.wrapping_add(1),
        _ => {}
    }
    1
}

pub fn op_dec_rr(cpu: &mut Cpu, pair: u8) -> u8 {
    match pair {
        0 => cpu.regs.set_bc(cpu.regs.bc().wrapping_sub(1)),
        1 => cpu.regs.set_de(cpu.regs.de().wrapping_sub(1)),
        2 => cpu.regs.set_hl(cpu.regs.hl().wrapping_sub(1)),
        3 => cpu.regs.sp = cpu.regs.sp.wrapping_sub(1),
        _ => {}
    }
    1
}

pub fn op_ld_rr_d16(cpu: &mut Cpu, bus: &mut Bus, pair: u8) -> u8 {
    let v = fetch_u16(cpu, bus);
    match pair {
        0 => cpu.regs.set_bc(v),
        1 => cpu.regs.set_de(v),
        2 => cpu.regs.set_hl(v),
        3 => cpu.regs.sp = v,
        _ => {}
    }
    2
}

pub fn op_add_hl_rr(cpu: &mut Cpu, pair: u8) -> u8 {
    let v = match pair {
        0 => cpu.regs.bc(),
        1 => cpu.regs.de(),
        2 => cpu.regs.hl(),
        3 => cpu.regs.sp,
        _ => 0,
    };
    let hl = cpu.regs.hl();
    let r = hl.wrapping_add(v);
    cpu.regs.set_hl(r);
    cpu.regs.set_n(false);
    cpu.regs.set_h((hl & 0xFFF).wrapping_add(v & 0xFFF) > 0xFFF);
    cpu.regs.set_c(hl as u32 + v as u32 > 0xFFFF);
    1
}

pub(crate) fn push_u16(cpu: &mut Cpu, bus: &mut Bus, v: u16) {
    cpu.regs.sp = cpu.regs.sp.wrapping_sub(1);
    bus.write(cpu.regs.sp, (v >> 8) as u8);
    cpu.regs.sp = cpu.regs.sp.wrapping_sub(1);
    bus.write(cpu.regs.sp, v as u8);
}

fn pop_u16(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
    let lo = bus.read(cpu.regs.sp) as u16;
    cpu.regs.sp = cpu.regs.sp.wrapping_add(1);
    let hi = bus.read(cpu.regs.sp) as u16;
    cpu.regs.sp = cpu.regs.sp.wrapping_add(1);
    lo | (hi << 8)
}

pub fn op_push_rr(cpu: &mut Cpu, bus: &mut Bus, pair: u8) -> u8 {
    let v = match pair {
        0 => cpu.regs.bc(),
        1 => cpu.regs.de(),
        2 => cpu.regs.hl(),
        3 => cpu.regs.af(),
        _ => 0,
    };
    push_u16(cpu, bus, v);
    3
}

pub fn op_pop_rr(cpu: &mut Cpu, bus: &mut Bus, pair: u8) -> u8 {
    let v = pop_u16(cpu, bus);
    match pair {
        0 => cpu.regs.set_bc(v),
        1 => cpu.regs.set_de(v),
        2 => cpu.regs.set_hl(v),
        3 => cpu.regs.set_af(v),
        _ => {}
    }
    2
}

pub fn op_alu_a_r(cpu: &mut Cpu, bus: &mut Bus, op: u8, r: u8) -> u8 {
    let v = get_r8(cpu, bus, r);
    alu_a_val(cpu, op, v);
    if r == 6 {
        1
    } else {
        0
    }
}

pub fn op_alu_a_d8(cpu: &mut Cpu, bus: &mut Bus, op: u8) -> u8 {
    let v = fetch_u8(cpu, bus);
    alu_a_val(cpu, op, v);
    1
}

fn alu_a_val(cpu: &mut Cpu, op: u8, v: u8) {
    let a = cpu.regs.a;
    match op {
        0 => {
            // ADD
            let r = a.wrapping_add(v);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h((a & 0xF) + (v & 0xF) > 0xF);
            cpu.regs.set_c(a as u16 + v as u16 > 0xFF);
            cpu.regs.a = r;
        }
        1 => {
            // ADC
            let c = cpu.regs.flag_c() as u8;
            let r = a.wrapping_add(v).wrapping_add(c);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            let h = (a & 0xF) + (v & 0xF) + c > 0xF;
            cpu.regs.set_h(h);
            cpu.regs.set_c(a as u16 + v as u16 + c as u16 > 0xFF);
            cpu.regs.a = r;
        }
        2 => {
            // SUB
            let r = a.wrapping_sub(v);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(true);
            cpu.regs.set_h((a & 0xF) < (v & 0xF));
            cpu.regs.set_c((a as u16) < (v as u16));
            cpu.regs.a = r;
        }
        3 => {
            // SBC
            let c = cpu.regs.flag_c() as u16;
            let r = a as u16 - v as u16 - c;
            let rb = r as u8;
            cpu.regs.set_z(rb == 0);
            cpu.regs.set_n(true);
            cpu.regs.set_h(((a ^ v ^ rb) & 0x10) != 0);
            cpu.regs.set_c(r > 0xFF);
            cpu.regs.a = rb;
        }
        4 => {
            let r = a & v;
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(true);
            cpu.regs.set_c(false);
            cpu.regs.a = r;
        }
        5 => {
            let r = a ^ v;
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(false);
            cpu.regs.a = r;
        }
        6 => {
            let r = a | v;
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_h(false);
            cpu.regs.set_c(false);
            cpu.regs.a = r;
        }
        7 => {
            let r = a.wrapping_sub(v);
            cpu.regs.set_z(r == 0);
            cpu.regs.set_n(true);
            cpu.regs.set_h((a & 0xF) < (v & 0xF));
            cpu.regs.set_c((a as u16) < (v as u16));
        }
        _ => {}
    }
}

pub fn op_rlca(cpu: &mut Cpu) -> u8 {
    let c = (cpu.regs.a & 0x80) != 0;
    cpu.regs.a = cpu.regs.a.rotate_left(1);
    cpu.regs.set_z(false);
    cpu.regs.set_n(false);
    cpu.regs.set_h(false);
    cpu.regs.set_c(c);
    0
}

pub fn op_rrca(cpu: &mut Cpu) -> u8 {
    let c = (cpu.regs.a & 1) != 0;
    cpu.regs.a = cpu.regs.a.rotate_right(1);
    cpu.regs.set_z(false);
    cpu.regs.set_n(false);
    cpu.regs.set_h(false);
    cpu.regs.set_c(c);
    0
}

pub fn op_rla(cpu: &mut Cpu) -> u8 {
    let old_c = cpu.regs.flag_c();
    let c = (cpu.regs.a & 0x80) != 0;
    cpu.regs.a = (cpu.regs.a << 1) | (old_c as u8);
    cpu.regs.set_z(false);
    cpu.regs.set_n(false);
    cpu.regs.set_h(false);
    cpu.regs.set_c(c);
    0
}

pub fn op_rra(cpu: &mut Cpu) -> u8 {
    let old_c = cpu.regs.flag_c();
    let c = (cpu.regs.a & 1) != 0;
    cpu.regs.a = (cpu.regs.a >> 1) | ((old_c as u8) << 7);
    cpu.regs.set_z(false);
    cpu.regs.set_n(false);
    cpu.regs.set_h(false);
    cpu.regs.set_c(c);
    0
}

pub fn op_daa(cpu: &mut Cpu) -> u8 {
    let mut a = cpu.regs.a;
    let mut c = cpu.regs.flag_c();
    if !cpu.regs.flag_n() {
        if cpu.regs.flag_h() || (a & 0x0F) > 9 {
            let (v, co) = a.overflowing_add(0x06);
            a = v;
            c = c || co;
        }
        if c || a > 0x99 {
            let (v, co) = a.overflowing_add(0x60);
            a = v;
            c = c || co;
        }
    } else {
        if cpu.regs.flag_h() {
            a = a.wrapping_sub(0x06);
        }
        if c {
            a = a.wrapping_sub(0x60);
        }
    }
    cpu.regs.set_z(a == 0);
    cpu.regs.set_h(false);
    cpu.regs.set_c(c);
    cpu.regs.a = a;
    0
}

pub fn op_cpl(cpu: &mut Cpu) -> u8 {
    cpu.regs.a = !cpu.regs.a;
    cpu.regs.set_n(true);
    cpu.regs.set_h(true);
    0
}

pub fn op_scf(cpu: &mut Cpu) -> u8 {
    cpu.regs.set_n(false);
    cpu.regs.set_h(false);
    cpu.regs.set_c(true);
    0
}

pub fn op_ccf(cpu: &mut Cpu) -> u8 {
    let c = cpu.regs.flag_c();
    cpu.regs.set_n(false);
    cpu.regs.set_h(false);
    cpu.regs.set_c(!c);
    0
}

pub fn op_ld_hlp_a(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    bus.write(hl, cpu.regs.a);
    cpu.regs.set_hl(hl.wrapping_add(1));
    1
}

pub fn op_ld_a_hlp(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    cpu.regs.a = bus.read(hl);
    cpu.regs.set_hl(hl.wrapping_add(1));
    1
}

pub fn op_ld_hlm_a(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    bus.write(hl, cpu.regs.a);
    cpu.regs.set_hl(hl.wrapping_sub(1));
    1
}

pub fn op_ld_a_hlm(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let hl = cpu.regs.hl();
    cpu.regs.a = bus.read(hl);
    cpu.regs.set_hl(hl.wrapping_sub(1));
    1
}

pub fn op_ld_nn_sp(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = fetch_u16(cpu, bus);
    bus.write(addr, cpu.regs.sp as u8);
    bus.write(addr.wrapping_add(1), (cpu.regs.sp >> 8) as u8);
    4
}

pub fn op_jp_hl(cpu: &mut Cpu) -> u8 {
    cpu.regs.pc = cpu.regs.hl();
    0
}

pub fn op_ld_sp_hl(cpu: &mut Cpu) -> u8 {
    cpu.regs.sp = cpu.regs.hl();
    1
}

pub fn op_di(cpu: &mut Cpu) -> u8 {
    cpu.ime = false;
    cpu.ei_latch = false;
    0
}

pub fn op_ei(_cpu: &mut Cpu) -> u8 {
    0
}

pub fn op_reti(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.regs.pc = pop_u16(cpu, bus);
    cpu.ime = true;
    cpu.ei_latch = false;
    3
}

pub fn op_ret(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.regs.pc = pop_u16(cpu, bus);
    3
}

pub fn op_add_sp_r8(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let v = fetch_u8(cpu, bus) as i8 as i16 as u16;
    let sp = cpu.regs.sp;
    let r = sp.wrapping_add(v);
    cpu.regs.set_z(false);
    cpu.regs.set_n(false);
    cpu.regs.set_h((sp & 0xF) + (v & 0xF) > 0xF);
    cpu.regs.set_c((sp & 0xFF) + (v & 0xFF) > 0xFF);
    cpu.regs.sp = r;
    3
}

pub fn op_ld_hl_sp_r8(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let v = fetch_u8(cpu, bus) as i8 as i16 as u16;
    let sp = cpu.regs.sp;
    let r = sp.wrapping_add(v);
    cpu.regs.set_hl(r);
    cpu.regs.set_z(false);
    cpu.regs.set_n(false);
    cpu.regs.set_h((sp & 0xF) + (v & 0xF) > 0xF);
    cpu.regs.set_c((sp & 0xFF) + (v & 0xFF) > 0xFF);
    2
}

pub fn op_jp_nn(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.regs.pc = fetch_u16(cpu, bus);
    3
}

pub fn op_call_nn(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let nn = fetch_u16(cpu, bus);
    push_u16(cpu, bus, cpu.regs.pc);
    cpu.regs.pc = nn;
    5
}

pub fn op_jr_r8(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let d = fetch_u8(cpu, bus) as i8;
    cpu.regs.pc = cpu.regs.pc.wrapping_add(d as u16);
    2
}

pub fn op_jp_cc_nn(cpu: &mut Cpu, bus: &mut Bus, i: u8) -> u8 {
    let nn = fetch_u16(cpu, bus);
    if cc(cpu, i) {
        cpu.regs.pc = nn;
    }
    3
}

pub fn op_call_cc_nn(cpu: &mut Cpu, bus: &mut Bus, i: u8) -> u8 {
    let nn = fetch_u16(cpu, bus);
    if cc(cpu, i) {
        push_u16(cpu, bus, cpu.regs.pc);
        cpu.regs.pc = nn;
        5
    } else {
        2
    }
}

pub fn op_ret_cc(cpu: &mut Cpu, bus: &mut Bus, i: u8) -> u8 {
    if cc(cpu, i) {
        cpu.regs.pc = pop_u16(cpu, bus);
        4
    } else {
        1
    }
}

pub fn op_jr_cc_r8(cpu: &mut Cpu, bus: &mut Bus, i: u8) -> u8 {
    let d = fetch_u8(cpu, bus) as i8;
    if cc(cpu, i) {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(d as u16);
        2
    } else {
        1
    }
}

pub fn op_rst(cpu: &mut Cpu, bus: &mut Bus, addr: u8) -> u8 {
    push_u16(cpu, bus, cpu.regs.pc);
    cpu.regs.pc = addr as u16;
    3
}

pub fn op_ld_a_nn(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = fetch_u16(cpu, bus);
    cpu.regs.a = bus.read(addr);
    3
}

pub fn op_ld_nn_a(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let addr = fetch_u16(cpu, bus);
    bus.write(addr, cpu.regs.a);
    3
}

pub fn op_ldh_n_a(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let n = fetch_u8(cpu, bus);
    bus.write(0xFF00 | n as u16, cpu.regs.a);
    2
}

pub fn op_ldh_a_n(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let n = fetch_u8(cpu, bus);
    cpu.regs.a = bus.read(0xFF00 | n as u16);
    2
}

pub fn op_ld_c_a(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    bus.write(0xFF00 | cpu.regs.c as u16, cpu.regs.a);
    1
}

pub fn op_ld_a_c(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.regs.a = bus.read(0xFF00 | cpu.regs.c as u16);
    1
}

/// `STOP` is a 2-byte opcode (`0x10`, operand). Operand fetch costs 1 M-cycle.
/// CGB: if KEY1 prepare is set, completes speed switch (handled on `Bus`).
pub fn op_stop(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let _operand = fetch_u8(cpu, bus);
    bus.cgb_stop_speed_switch();
    1
}

pub fn op_halt(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.halted = true;
    0
}

pub fn op_illegal(_cpu: &mut Cpu, _bus: &mut Bus, _opc: u8) -> u8 {
    // Undocumented: consume no extra bytes, 1 M-cycle total
    0
}
