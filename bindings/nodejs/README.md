# Nodejs Binding

## Dev

The code are splited into:
- `lib/index.js`:  the `nodejs` entry point for sleighcraft
- `native/build.rs`:  the `neon` build setting
- `native/src/lib.rs` sleigh with nodejs api
- `package.json`: the `npm` build config

## Install

```
# linux-x86
npm install bincraft

# or 
git clone https://github.com/StarCrossPortal/bincraft
cd bindings/nodejs/
npm install -g neon-cli
neon build
```
Due to compilation reasons, the bincraft obtained by the current npm installation method will be of the `linux-x86` architecture.

If other architectures need to be installed via npm
First step:
``` 
    npm i bincraft
```
The second step:
Download the package of the corresponding architecture through [bincraft-releases](https://github.com/StarCrossPortal/bincraft/releases)

The third step:
```asm
cp download/corresponding/arch/native/index.node your/node_modules/path/bincraft/native/index.node
```
Done

## Usage

```
const Sleigh = require('bincraft');
//or const Sleigh = require('.');

// init the sleigh engine Sleigh(arch, code) like python
const sleigh = new Sleigh("x86",[0x90,90]);

// disasm(start_addr) 
// - start: Default is 0
const asms = sleigh.disasm();

asms.forEach(asm => {
    let addr = asm.addr();
    let mnemonic = asm.mnemonic();
    let body = asm.body();
    // dump instruction
    console.log(`addr: ${addr}\t mnemonic: ${mnemonic}\t body: ${body}`);
    
    // And we have IR！
    let pcodes = asm.pcodes();
    pcodes.forEach(pcode => {
        opcode = pcode.opcode();
        vars = pcode.vars();
        
        console.log(`opcode: ${opcode}\t vars: ${vars}`);
    });
});

```
