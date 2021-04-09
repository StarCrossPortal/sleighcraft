# Bincraft - Binary Analysis Craft

Bincraft is a (soon-to-be) binary analysis toolkit that serves like [capstone](https://www.capstone-engine.org/) or [angr](https://github.com/angr/angr) based on [ghidra](https://ghidra-sre.org/).

Our goal is to provide a binary analysis toolkit that is:

- extensible
- library-like (composible)

So that people could freely select what they want and build the things they want.

## Plan

We planed big, but we needs to start small.

Our plan includes:

- [x] decoder (linear disassembler) with IR (based on ghidra)
- [ ] encoder (single instruction assemble) (based on ghidra)
- [ ] universal binary analysis algorithms (high-level enough to be used anywhere)
- [ ] DECOMPILER! yay!

Our plan is to split them up into several crates, use the beloved Rust to write as much as we can. So, as for now, we only have `slgiehcraft` that could do disassemble.

## Sleighcraft

Sleighcraft is a decoder (or, linear disassembler) based on ghidra's decompiler implementation. Sleighcraft can be used in Rust or Python, with both high-level and low-level API.

### How to install

#### Rust

Use cargo:

```toml
sleighcraft = { git = "https://github.com/ret2lab/bincraft" }
```

The repo is a bit large to submit on crates-io (because of predefined sla files), but save you the complex of compiling sleigh files yourself.

#### Python

```bash
# quick install it with pip
$ pip3 install bincraft

# or download binaries than choose the corresponding architecture
$ pip3 install bincraft-0.1.0-cp39-cp39-Arch.whl

# or manual, to do this, you need to have rust compiler installed and maturin
# better with rustup.
$ pip3 install maturin
$ maturin build
$ pip3 install bincraft-0.1.0-cp39-cp39-Arch.whl
```

### How to Use

One could refer to doc.rs to see how Rust binding can be used.

Python binding:

```python
from bincraft import Sleigh

code = [0x90, 0x31, 0x32] # code to disassemble

# init the sleigh engine Sleigh(arch, code)
sleigh = Sleigh("x86", code)

# now we are prepared to disassemble!
# disasm(start_addr)
for asm in sleigh.disasm(0):
    addr = asm.addr()
    mnem = asm.mnemonic()
    body = asm.body()

    # quite like capstone, right?
    print(f'Addr: {addr}\t  mnemonic: {mnem}\t body: {body}')

    # but! we also have the IR!
    pcodes = asm.pcodes()
    for pcode in pcodes:
        opcode = pcode.opcode()
        vars = pcode.vars()
        print(f'opcode: {opcode}\t vars: {vars}\t')
    print()
```

TODO: add Rust example