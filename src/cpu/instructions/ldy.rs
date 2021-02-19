// LDY - Load Index Register Y From Memory

// Operation: M → Y

// Load the index register Y from memory.

// LDY does not affect the C or V flags, sets the N flag if the value loaded in bit 7 is a 1, otherwise resets N, sets Z flag if the loaded value is zero otherwise resets Z and only affects the Y register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	LDY #$nn	$A0	2	2
// Absolute	LDY $nnnn	$AC	3	4
// X-Indexed Absolute	LDY $nnnn,X	$BC	3	4+p
// Zero Page	LDY $nn	$A4	2	3
// X-Indexed Zero Page	LDY $nn,X	$B4	2	4

// p: =1 if page is crossed.
