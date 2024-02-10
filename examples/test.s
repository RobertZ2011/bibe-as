mov %r1, 128
mov %r2, 255
strw [%r1, %z], %r2
ldrw %r3, [%r1, %z]
shl %r2, %r2, 8
strw [%r1, %z], %r2
ldrw %r4, [%r1, %z]
shl %r2, %r2, 8
strw [%r1, %z], %r2
ldrw %r5, [%r1, %z]
shl %r2, %r2, 8
strw [%r1, %z], %r2
ldrw %r6, [%r1, %z]