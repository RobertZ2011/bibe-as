	mov r1, 5
	mov r9, 0
	mov r10, 1
	mov r8, 0
loop:
	cmp r8, r1
	b.ge end
	add r11, r9, r10
	mov r9, r10
	mov r10, r11
	add r8, r8, 1
	b loop
end:
	mov r8, r9