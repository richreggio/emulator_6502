use super::*;

// LAX - Load Accumulator and Index Register X From Memory
// Operation: M → A, X
// The undocumented LAX instruction loads the accumulator and the index register X from memory.
// LAX does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it; sets N if the value loaded in bit 7 is a 1; otherwise N is reset, and affects only the X register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	LAX #$nn	$AB*	2	2
// Absolute	LAX $nnnn	$AF*	3	4
// Y-Indexed Absolute	LAX $nnnn,Y	$BF*	3	4+p
// Zero Page	LAX $nn	$A7*	2	3
// Y-Indexed Zero Page	LAX $nn,Y	$B7*	2	4
// X-Indexed Zero Page Indirect	LAX ($nn,X)	$A3*	2	6
// Zero Page Indirect Y-Indexed	LAX ($nn),Y	$B3*	2	5+p
// p: =1 if page is crossed.

pub fn lax(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
