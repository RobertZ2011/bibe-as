	mov r1, 4
	mov r10, 1
loop:
	cmp r8, r1
	j.ge end
	add r11, r9, r10
	mov r9, r10
	mov r10, r11
	add r8, r8, 1
	j loop
end:
	mov r1, r11
	csww 384, r1
	mov r1, 10
	csww 320, r1