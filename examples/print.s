	mov 	%a0, start
	add 	%lr, %pc, 8
	b 		print
	mov 	%a0, finish
	add 	%lr, %pc, 8
	b 		print
loop:
	b 		loop
print:
	mov 	%l0, 0
	ldrb	%l0, [%a0]
	cmp		%z, %l0
	b.eq 	end
	csww 	320, %l0
	add 	%a0, %a0, 1
	b 		print
end:
	b 		%lr
start:
	.asciiz "Hello "
finish:
	.asciiz "world!\n"