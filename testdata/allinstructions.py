singlet_ops = ['nop', 'hlt', 'rlc', 'ral', 'rrc', 'rar', 'ei', 'di', 'rim',
    'sim', 'daa', 'stc', 'cma', 'cmc', 'xthl', 'xchg', 'pchl', 'sphl',
    'rstv', 'dsub', 'arhl', 'rdel', 'shlx', 'lhlx']
jumps = ['jmp', 'jx5', 'jnx5', 'jnz', 'jz', 'jnc', 'jc', 'jpo', 'jpe', 'jp', 'jm']
calls = ['call', 'cnz', 'cz', 'cnc', 'cc', 'cpo', 'cpe', 'cp', 'cm']
rets = ['ret', 'rnz', 'rz', 'rnc', 'rc', 'rpo', 'rpe', 'rp', 'rm']
registers = ['a', 'b', 'c', 'd', 'e', 'h', 'l', 'm']
reg_pairs = ['b', 'd', 'h', 'sp']
arith_reg_ops = ['add', 'adc', 'sub', 'sbb', 'inr', 'dcr', 'ana', 'ora', 'xra', 'cmp']
arith_imm_ops = ['adi', 'aci', 'sui', 'sbi', 'ani', 'ori', 'xri', 'cpi']
arith_rp_ops = ['inx', 'dcx', 'dad']
mem_rp_ops = ['ldax', 'stax']
mem_ops = ['lda', 'sta', 'lhld', 'shld']
stack_ops = ['push', 'pop']
rp_stack = ['b', 'd', 'h', 'psw']
io_ops = ['in', 'out']

print('\tCPU 8085UNDOC')
print('\tORG 0h')

print('flow:')
for op in jumps:
    print(f'\t{op} 1337h')

for op in calls:
    print(f'\t{op} 1337h')

for op in rets:
    print(f'\t{op}')

print(f'\trst 3h')
print(f'\tpchl')

print('lxi:')
for rp in reg_pairs:
    print(f'\tlxi {rp}, 1137h')

print('mvi:')
for r in registers:
    print(f'\tmvi {r}, 0aah')

print('reg_arith:')
for op in arith_reg_ops:
    for r in registers:
        print(f'\t{op} {r}')

print('imm_arith:')
for op in arith_imm_ops:
    print(f'\t{op} 0aah')

print('io:')
print('\tin 0aah')
print('\tout 0aah')

print('stack:')
for op in stack_ops:
    for rp in rp_stack:
        print(f'\t{op} {rp}')

print('mem:')
for op in mem_ops:
    print(f'\t{op} 1337h')

print('mem_rp:')
for op in mem_rp_ops:
    for rp in ['b', 'd']: # note: LDAX/STAX H are just MOV A,M and MOV M,A
        print(f'\t{op} {rp}')

print('arith_rp:')
for op in arith_rp_ops:
    for rp in reg_pairs[:-1]:
        print(f'\t{op} {rp}')

print('singlets:')
for op in singlet_ops:
    print(f'\t{op}')
