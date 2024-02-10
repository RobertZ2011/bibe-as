	mov %a0, 5
	mov %l0, 0
	mov %l1, 0
	mov %l2, 1
loop:
	cmp %l0, %a0
	b.ge end
	add %l3, %l1, %l2
	mov %l1, %l2
	mov %l2, %l3
	add %l0, %l0, 1
	b loop
end:
	mov %o0, %l1
	swi