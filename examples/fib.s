.origin 0
	mov r1, 5
	mov r8, 0
	mov r9, 0
	mov r10, 1
loop:
	cmp r0, r8, r1
	j.ge end
	add r11, r9, r10
	mov r9, r10
	mov r10, r11
	add r8, r8, 1
	j loop
end:
	mov r1, r11