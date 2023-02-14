mov r1, 128
mov r2, 255
strw [r1, r0], r2
ldrw r3, [r1, r0]
shl r2, r2, 8
strw [r1, r0], r2
ldrw r4, [r1, r0]
shl r2, r2, 8
strw [r1, r0], r2
ldrw r5, [r1, r0]
shl r2, r2, 8
strw [r1, r0], r2
ldrw r6, [r1, r0]