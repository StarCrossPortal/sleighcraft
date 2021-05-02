from bincraft import Sleigh

code = [0x55, 0x48, 0x89, 0xE5, 0xBF, 0xC4, 0x05, 0x40]

sleigh = Sleigh("x86", code)

for asm in sleigh.disasm(0x123120):
    addr = asm.addr()
    mnem = asm.mnemonic()
    body = asm.body()

    print(f'Addr: {addr}\t  mnemonic: {mnem}\t body: {body}')
    print(asm)

    pcodes = asm.pcodes()
    for pcode in pcodes:
        opcode = pcode.opcode()
        vars = pcode.vars()

        print(f'opcode: {opcode}\t vars: {vars}\t')
        print(pcode)
    print()