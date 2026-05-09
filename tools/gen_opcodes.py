#!/usr/bin/env python3
"""Generate src/cpu/opcodes_gen.rs from compact opcode metadata."""
from __future__ import annotations

import textwrap

# M-cycle counts for unprefixed opcodes (index 0x00-0xFF). Source: Pan Docs / Mooneye.
# CB-prefixed handled separately (always 2 m-cycles for CB fetch + inner timing).

M: list[int] = [0] * 256

def set_range(start: int, end: int, c: int):
    for i in range(start, end + 1):
        M[i] = c

# Defaults / bulk patterns filled below
for i in range(256):
    M[i] = 1

# NOP, HALT, STOP-ish
M[0x00] = 1
M[0x10] = 1  # STOP treated as 1 for timing stub
M[0x76] = 1

# 8-bit immediate loads
for op in range(0x06, 0x40, 8):
    M[op] = 2  # LD B,d8 etc
M[0x36] = 3  # LD (HL),d8

# 16-bit loads
for op in [0x01, 0x11, 0x21, 0x31]:
    M[op] = 3

# LD (nn), SP
M[0x08] = 5

# INC/DEC 16
for op in [0x03, 0x13, 0x23, 0x33, 0x0B, 0x1B, 0x2B, 0x3B]:
    M[op] = 2

# INC/DEC 8 (not (HL))
for op in list(range(0x04, 0x40, 8)) + list(range(0x05, 0x40, 8)):
    if op in (0x34, 0x35):
        continue
    M[op] = 1
M[0x34] = 3  # INC (HL)
M[0x35] = 3  # DEC (HL)

# LD r,r block 0x40-0x7F except 0x76
for op in range(0x40, 0x80):
    if op == 0x76:
        continue
    dst = (op >> 3) & 7
    src = op & 7
    if dst == 6 and src == 6:
        pass
    elif dst == 6 or src == 6:
        M[op] = 2
    else:
        M[op] = 1

# ALU A,r 0x80-BF
for op in range(0x80, 0xC0):
    if (op & 7) == 6:
        M[op] = 2
    else:
        M[op] = 1

# PUSH/POP
for op in [0xC5, 0xD5, 0xE5, 0xF5]:
    M[op] = 4
for op in [0xC1, 0xD1, 0xE1, 0xF1]:
    M[op] = 3

# JP nn, CALL nn
M[0xC3] = 4
M[0xCD] = 6

# JP (HL) is 1
M[0xE9] = 1

# JP cc nn / CALL cc nn
for op in range(0xC2, 0xE0, 8):
    M[op] = 4  # JP NZ, etc
for op in range(0xC4, 0xE0, 8):
    M[op] = 6  # CALL NZ, etc

# RET
M[0xC9] = 4
M[0xD9] = 4  # RETI
for op in range(0xC0, 0xE0, 8):
    if op in (0xC8, 0xD8, 0xE8, 0xF8):
        M[op] = 5  # RET cc
        continue
M[0xC8] = M[0xD8] = M[0xE8] = M[0xF8] = 5

# RST
for op in range(0xC7, 0x100, 8):
    M[op] = 4

# JR r8
M[0x18] = 3
for op in range(0x20, 0x40, 8):
    M[op] = 3  # JR cc

# DAA, CPL, SCF, CCF
for op in [0x27, 0x2F, 0x37, 0x3F]:
    M[op] = 1

# DI, EI
M[0xF3] = M[0xFB] = 1

# RLCA/RRCA/RRA/RRLA
for op in [0x07, 0x0F, 0x17, 0x1F]:
    M[op] = 1

# ADD HL,rr
for op in [0x09, 0x19, 0x29, 0x39]:
    M[op] = 2

# LD A,(BC/DE/nn)
M[0x0A] = M[0x1A] = 2
M[0xFA] = 4

# LD (BC/DE),A
M[0x02] = M[0x12] = 2
M[0xEA] = 4

# LDH
M[0xE0] = M[0xF0] = 3
M[0xE2] = M[0xF2] = 2

# LD SP, HL
M[0xF9] = 2

# LD HL, SP+r8
M[0xF8] = 3

# ADD SP,r8
M[0xE8] = 4

# misc
M[0x22] = M[0x2A] = M[0x32] = M[0x3A] = 2  # LD (HL+/-), A etc

# CB prefix
M[0xCB] = 2  # plus inner - handled in CPU

# Fix RET cc - already set in loop wrongly - check
for op in range(0xC0, 0x100, 8):
    if op in (0xC0, 0xC8, 0xD0, 0xD8, 0xE0, 0xE8, 0xF0, 0xF8):
        M[op] = 5  # RET cc

# RET unconditional
M[0xC9] = 4

# POP already

# Verify some
assert M[0x00] == 1
assert M[0xCB] == 2


def emit_rust():
    # This script originally intended full codegen; the repo uses hand-written cpu/opcodes.rs instead.
    print("// Deprecated generator stub — opcodes are in cpu/opcodes.rs")
    print(f"pub const M_CYCLES_TABLE: [u8; 256] = {M!r};")


if __name__ == "__main__":
    emit_rust()
