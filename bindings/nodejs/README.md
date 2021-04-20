# Nodejs Binding

## Dev

The code are splited into:
- `lib/index.js`:  the `nodejs` entry point for sleighcraft
- `native/build.rs`:  the `neon` build setting
- `native/src/lib.rs` sleigh with nodejs api
- `package.json`: the `npm` build config

## Install

```
# this method hat not yet been implemented
npm install bincraft

# or 
npm install -g neon-cli
neon build

```

## Usage

```
const Sleigh = require('.');

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

```
