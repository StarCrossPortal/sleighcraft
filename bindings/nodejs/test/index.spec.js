const Sleigh = require('..');

const sleigh = new Sleigh("x86",[0x90,90]);

const asms = sleigh.disasm();

asms.forEach(asm => {
    let addr = asm.addr();
    let mnemonic = asm.mnemonic();
    let body = asm.body();
    console.log(`addr: ${addr}\t mnemonic: ${mnemonic}\t body: ${body}`);
    let pcodes = asm.pcodes();
    pcodes.forEach(pcode => {
        opcode = pcode.opcode();
        vars = pcode.vars();
        console.log(`opcode: ${opcode}\t vars: ${vars}`);
    });
});