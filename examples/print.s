	mov 	r1, start
	add 	lr, pc, 8
	b 		print
	mov 	r1, finish
	add 	lr, pc, 8
	b 		print
loop:
	b 		loop
print:
	mov 	r2, 0
	ldrb	r2, [r1]
	cmp		r0, r0, r2
	b.eq 	end
	csww 	320, r2
	add 	r1, r1, 1
	b 		print
end:
	b 		lr
start:
	.asciiz "Hello "
finish:
	.asciiz "world!\n"