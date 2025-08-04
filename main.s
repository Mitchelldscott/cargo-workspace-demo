
main:	file format elf32-littlearm

Disassembly of section .text:

000200e4 <main::matrix_multiply::h36199f4376f43e57>:
   200e4:      	push	{r4, r5, r6, r7, lr}
   200e6:      	add	r7, sp, #0xc
   200e8:      	push.w	{r8, r9, r11}
   200ec:      	mov.w	r12, #0x0
   200f0:      	vldr	s0, [pc, #92]           @ 0x20150 <main::matrix_multiply::h36199f4376f43e57+0x6c>
   200f4:      	cmp.w	r12, #0x3
   200f8:      	beq	0x2014a <main::matrix_multiply::h36199f4376f43e57+0x66> @ imm = #0x4e
   200fa:      	add.w	lr, r2, r12, lsl #2
   200fe:      	mov.w	r9, #0x0
   20102:      	mov	r6, r1
   20104:      	cmp.w	r9, #0x2
   20108:      	beq	0x20142 <main::matrix_multiply::h36199f4376f43e57+0x5e> @ imm = #0x36
   2010a:      	vmov.f32	s2, s0
   2010e:      	add.w	r3, r9, r9, lsl #1
   20112:      	movs	r4, #0x0
   20114:      	lsl.w	r8, r3, #0x2
   20118:      	mov	r3, r0
   2011a:      	cmp	r4, #0x10
   2011c:      	beq	0x20132 <main::matrix_multiply::h36199f4376f43e57+0x4e> @ imm = #0x12
   2011e:      	adds	r5, r6, r4
   20120:      	vldr	s6, [r3]
   20124:      	adds	r3, #0xc
   20126:      	adds	r4, #0x4
   20128:      	vldr	s4, [r5]
   2012c:      	vmla.f32	s2, s6, s4
   20130:      	b	0x2011a <main::matrix_multiply::h36199f4376f43e57+0x36> @ imm = #-0x1a
   20132:      	add.w	r3, lr, r8
   20136:      	adds	r6, #0x10
   20138:      	add.w	r9, r9, #0x1
   2013c:      	vstr	s2, [r3]
   20140:      	b	0x20104 <main::matrix_multiply::h36199f4376f43e57+0x20> @ imm = #-0x40
   20142:      	adds	r0, #0x4
   20144:      	add.w	r12, r12, #0x1
   20148:      	b	0x200f4 <main::matrix_multiply::h36199f4376f43e57+0x10> @ imm = #-0x58
   2014a:      	pop.w	{r8, r9, r11}
   2014e:      	pop	{r4, r5, r6, r7, pc}
   20150: 00 00 00 00  	.word	0x00000000

00020154 <_start>:
   20154:      	push	{r7, lr}
   20156:      	mov	r7, sp
   20158:      	sub	sp, #0x68
   2015a:      	ldr	r0, [pc, #0x64]         @ 0x201c0 <_start+0x6c>
   2015c:      	mov.w	r12, #0x41000000
   20160:      	str	r0, [sp, #0x2c]
   20162:      	mov.w	r4, #0x40800000
   20166:      	ldr	r0, [pc, #0x5c]         @ 0x201c4 <_start+0x70>
   20168:      	mov.w	r6, #0x40000000
   2016c:      	str	r0, [sp, #0x28]
   2016e:      	mov.w	r1, #0x3f800000
   20172:      	ldr	r0, [pc, #0x54]         @ 0x201c8 <_start+0x74>
   20174:      	ldr	r2, [pc, #0x5c]         @ 0x201d4 <_start+0x80>
   20176:      	str	r0, [sp, #0x24]
   20178:      	ldr	r0, [pc, #0x50]         @ 0x201cc <_start+0x78>
   2017a:      	ldr	r3, [pc, #0x5c]         @ 0x201d8 <_start+0x84>
   2017c:      	ldr	r5, [pc, #0x5c]         @ 0x201dc <_start+0x88>
   2017e:      	str	r0, [sp, #0x20]
   20180:      	ldr	r0, [pc, #0x4c]         @ 0x201d0 <_start+0x7c>
   20182:      	str.w	r12, [sp, #0x1c]
   20186:      	str	r0, [sp, #0x18]
   20188:      	strd	r0, r12, [sp, #72]
   2018c:      	movs	r0, #0x0
   2018e:      	str	r2, [sp, #0x14]
   20190:      	strd	r3, r2, [sp, #64]
   20194:      	movs	r2, #0x0
   20196:      	str	r3, [sp, #0x10]
   20198:      	str	r4, [sp, #0xc]
   2019a:      	str	r5, [sp, #0x8]
   2019c:      	str	r6, [sp, #0x4]
   2019e:      	str	r1, [sp]
   201a0:      	strd	r5, r4, [sp, #56]
   201a4:      	strd	r1, r6, [sp, #48]
   201a8:      	add	r1, sp, #0x50
   201aa:      	cmp	r2, #0x18
   201ac:      	beq	0x201b4 <_start+0x60>   @ imm = #0x4
   201ae:      	str	r0, [r1, r2]
   201b0:      	adds	r2, #0x4
   201b2:      	b	0x201aa <_start+0x56>   @ imm = #-0xc
   201b4:      	add	r1, sp, #0x30
   201b6:      	add	r2, sp, #0x50
   201b8:      	mov	r0, sp
   201ba:      	bl	0x200e4 <main::matrix_multiply::h36199f4376f43e57> @ imm = #-0xda
   201be:      	b	0x201be <_start+0x6a>   @ imm = #-0x4
   201c0: 00 00 40 41  	.word	0x41400000
   201c4: 00 00 30 41  	.word	0x41300000
   201c8: 00 00 20 41  	.word	0x41200000
   201cc: 00 00 10 41  	.word	0x41100000
   201d0: 00 00 e0 40  	.word	0x40e00000
   201d4: 00 00 c0 40  	.word	0x40c00000
   201d8: 00 00 a0 40  	.word	0x40a00000
   201dc: 00 00 40 40  	.word	0x40400000
