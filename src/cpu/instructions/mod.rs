// Load instructions - 12 total
mod las;
mod lax;
mod lda;
mod ldx;
mod ldy;
mod sax;
mod sha;
mod shx;
mod shy;
mod sta;
mod stx;
mod sty;

// Transfer instructions - 7 total
mod shs;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

// Stack instructions - 4 total
mod pha;
mod php;
mod pla;
mod plp;

// Shift instructions - 4 total
mod asl;
mod lsr;
mod rol;
mod ror;

// Logic instructions - 4 total
mod and;
mod bit;
mod eor;
mod ora;

// Arithmetic instructions - 16 total
mod adc;
mod anc;
mod arr;
mod asr;
mod cmp;
mod cpx;
mod cpy;
mod dcp;
mod isc;
mod rla;
mod rra;
mod sbc;
mod sbx;
mod slo;
mod sre;
mod xaa;

// Increment instructions - 6 total
mod dec;
mod dex;
mod dey;
mod inc;
mod inx;
mod iny;

// Control instructions - 5 total
mod brk;
mod jmp;
mod jsr;
mod rti;
mod rts;

// Branch instructions - 8 total
mod bcc;
mod bcs;
mod beq;
mod bmi;
mod bne;
mod bpl;
mod bvc;
mod bvs;

// Flag instructions - 7 total
mod clc;
mod cld;
mod cli;
mod clv;
mod sec;
mod sed;
mod sei;

// Halt the CPU instruction
mod jam;

// No operation instruction
mod nop;
