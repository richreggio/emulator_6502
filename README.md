A full implementation of the 6502 microprocessor completely written in Rust.

Includes the 151 documented opcodes as well as the 105 undocumented opcodes.

## OPCODES

All documented opcodes are currently implemented

The following undocumented opcodes are not implemented:

1. ISC
2. RLA
3. RRA
4. SBX
5. SHA
6.  SHS
7.  SHX
8.  SHY
9.  SLO
10. SRE
11. XAA

The following references were used:

I know the C64 used the 6510 but it was the same as the 6502 except it had an onboard 8-bit I/O port
[Ultimate C64 Reference](https://www.pagetable.com/c64ref/6502/)

[6502 Reference](http://www.obelisk.me.uk/6502/index.html)



## TODO LIST

1. Add a GUI to the program
2. Add disassembler
3. Refactor Memory to include RAM module
4. Maybe change registers fields to not be public, better encapsulation
5. Add Decimal mode and binary mode