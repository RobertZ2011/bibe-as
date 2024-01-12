	mov 	r1, start
	add 	lr, pc, 8
	j 		print
	mov 	r1, finish
	add 	lr, pc, 8
	j 		print
loop:
	j 		loop
print:
	mov 	r2, 0
	ldrb	r2, [r1]
	cmp		r0, r0, r2
	j.eq 	end
	csww 	320, r2
	add 	r1, r1, 1
	j 		print
end:
	j 		lr
start:
	.asciiz "Hello "
finish:
	.asciiz "world!\n"