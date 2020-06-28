	CPU 8085UNDOC
	ORG 0h
flow:
	jmp 1337h
	jx5 1337h
	jnx5 1337h
	jnz 1337h
	jz 1337h
	jnc 1337h
	jc 1337h
	jpo 1337h
	jpe 1337h
	jp 1337h
	jm 1337h
	call 1337h
	cnz 1337h
	cz 1337h
	cnc 1337h
	cc 1337h
	cpo 1337h
	cpe 1337h
	cp 1337h
	cm 1337h
	ret
	rnz
	rz
	rnc
	rc
	rpo
	rpe
	rp
	rm
	rst 3h
	pchl
lxi:
	lxi b, 1137h
	lxi d, 1137h
	lxi h, 1137h
	lxi sp, 1137h
mvi:
	mvi a, 0aah
	mvi b, 0aah
	mvi c, 0aah
	mvi d, 0aah
	mvi e, 0aah
	mvi h, 0aah
	mvi l, 0aah
	mvi m, 0aah
reg_arith:
	add a
	add b
	add c
	add d
	add e
	add h
	add l
	add m
	adc a
	adc b
	adc c
	adc d
	adc e
	adc h
	adc l
	adc m
	sub a
	sub b
	sub c
	sub d
	sub e
	sub h
	sub l
	sub m
	sbb a
	sbb b
	sbb c
	sbb d
	sbb e
	sbb h
	sbb l
	sbb m
	inr a
	inr b
	inr c
	inr d
	inr e
	inr h
	inr l
	inr m
	dcr a
	dcr b
	dcr c
	dcr d
	dcr e
	dcr h
	dcr l
	dcr m
	ana a
	ana b
	ana c
	ana d
	ana e
	ana h
	ana l
	ana m
	ora a
	ora b
	ora c
	ora d
	ora e
	ora h
	ora l
	ora m
	xra a
	xra b
	xra c
	xra d
	xra e
	xra h
	xra l
	xra m
	cmp a
	cmp b
	cmp c
	cmp d
	cmp e
	cmp h
	cmp l
	cmp m
imm_arith:
	adi 0aah
	aci 0aah
	sui 0aah
	sbi 0aah
	ani 0aah
	ori 0aah
	xri 0aah
	cpi 0aah
io:
	in 0aah
	out 0aah
stack:
	push b
	push d
	push h
	push psw
	pop b
	pop d
	pop h
	pop psw
mem:
	lda 1337h
	sta 1337h
	lhld 1337h
	shld 1337h
mem_rp:
	ldax b
	ldax d
	stax b
	stax d
arith_rp:
	inx b
	inx d
	inx h
	dcx b
	dcx d
	dcx h
	dad b
	dad d
	dad h
singlets:
	nop
	hlt
	rlc
	ral
	rrc
	rar
	ei
	di
	rim
	sim
	daa
	stc
	cma
	cmc
	xthl
	xchg
	pchl
	sphl
	rstv
	dsub
	arhl
	rdel
	shlx
	lhlx
