pub use crate::cpu::addressing_mode::AddressingMode as AdMode;
pub use crate::cpu::operation::Operation;
pub use crate::cpu::CPU;
pub use crate::memory::IRQ_VECTOR;

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

pub use adc::adc;
pub use anc::anc;
pub use and::and;
pub use arr::arr;
pub use asl::asl;
pub use asr::asr;
pub use bcc::bcc;
pub use bcs::bcs;
pub use beq::beq;
pub use bit::bit;
pub use bmi::bmi;
pub use bne::bne;
pub use bpl::bpl;
pub use brk::brk;
pub use bvc::bvc;
pub use bvs::bvs;
pub use clc::clc;
pub use cld::cld;
pub use cli::cli;
pub use clv::clv;
pub use cmp::cmp;
pub use cpx::cpx;
pub use cpy::cpy;
pub use dcp::dcp;
pub use dec::dec;
pub use dex::dex;
pub use dey::dey;
pub use eor::eor;
pub use inc::inc;
pub use inx::inx;
pub use iny::iny;
pub use isc::isc;
pub use jam::jam;
pub use jmp::jmp;
pub use jsr::jsr;
pub use las::las;
pub use lax::lax;
pub use lda::lda;
pub use ldx::ldx;
pub use ldy::ldy;
pub use lsr::lsr;
pub use nop::nop;
pub use ora::ora;
pub use pha::pha;
pub use php::php;
pub use pla::pla;
pub use plp::plp;
pub use rla::rla;
pub use rol::rol;
pub use ror::ror;
pub use rra::rra;
pub use rti::rti;
pub use rts::rts;
pub use sax::sax;
pub use sbc::sbc;
pub use sbx::sbx;
pub use sec::sec;
pub use sed::sed;
pub use sei::sei;
pub use sha::sha;
pub use shs::shs;
pub use shx::shx;
pub use shy::shy;
pub use slo::slo;
pub use sre::sre;
pub use sta::sta;
pub use stx::stx;
pub use sty::sty;
pub use tax::tax;
pub use tay::tay;
pub use tsx::tsx;
pub use txa::txa;
pub use txs::txs;
pub use tya::tya;
pub use xaa::xaa;
