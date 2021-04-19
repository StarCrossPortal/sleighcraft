# BinCraft - Binary Analysis Craft

BinCraft is a future binary analysis toolkit.

Features:

- Layered Architecture: composed by multiple libraries that can be used seperatedly.
- Written in Rust: high performance, safe interface, no VM.
- Python API: easy scripting. In the future, C API will also be provided, allows to bind to more programming languages.
- Extensible: with [sleigh DSL](https://ghidra.re/courses/languages/html/sleigh.html), new architecture is easy to add.
- (Currently In Development) **SQL based binary analysis**

BinCraft is seperated into multiple parts, while currently only the first one, `sleighcraft` is working.

NOTE:

this project is still in early stage.
Large scale API modifications, bugs are expected.
Documentations are yet to be complete.
Star us, we will try our best to make it complete and better. 🥺 Please, do it.


## SleighCraft

`SleighCraft` is a decoder (or, linear disassembler) based on ghidra's decompiler implementation. Sleighcraft can be used in Rust or Python, with both high-level and low-level API.

In general, `sleighcraft` is just like [capstone](https://www.capstone-engine.org/) but with IR and more archs.

Features:

- Rust based API and Python scripting API.
- Decoding with IR as the semantic meaning.
- Archs: **110** architectures.

️️✔️: provided

❌: not provided

🚧: in construction

🤔: not sure, maybe not

Comparison with capstone:

|Feature|SleighCraft| Capstone Engine |
|-------|----------|----------|
|disassemble| ✔️ |  ✔️ | 
|IR|✔️️|❌|
|C API|🚧|✔️|
|custom architecture|️✔️|❌|

Architectures comparision with capstone (according to [capstone arch list](https://www.capstone-engine.org/arch.html)):

|Arch Names|SleighCraft| Capstone Engine |
|----------|-----------|-----------------|
|6502|✔️|🤔|
|6805|✔️|🤔|
|8051|✔️|🤔|
|8048|✔️|🤔|
|8085|✔️|🤔|
|68000|✔️|🤔|
|aarch64(armv8)|✔️|️️✔️|
|arm|✔️|️️✔️|
|cp1600|✔️|🤔|
|cr16|✔️|🤔|
|avr8|✔️|️️🤔|
|dalvik|✔️|🤔|
|jvm|✔️|🤔|
|mips|✔️|️️✔️|
|powerpc|✔️|️️✔️|
|sparc|✔️|️️✔️|
|tricore|✔️|🤔|
|riscv|✔️|🤔|
|z80|✔️|🤔|
|System Z|❌|✔️|
|xCore|❌|✔️|

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

Rust (kinda low level):

```Rust
// Overall procedure:
// 1. get the spec, this is where we know how to decode anything
// 2. get a loader, this is where we fill the input bytes to the engine.
// A predefined loader is provided: `PlainLoadImage`, which sets
// the things to decode by using a single buf.
// 3. set the AssemblyEmit and PcodeEmit instance, these are two
// traits that defines the callback at the decode time.
// 4. do the decode
use sleighcraft::*;
let mut sleigh_builder = SleighBuilder::default();
let spec = arch("x86").unwrap();
let buf = [0x90, 0x32, 0x31];
let mut loader = PlainLoadImage::from_buf(&buf, 0);
sleigh_builder.loader(&mut loader);
sleigh_builder.spec(spec);
let mut asm_emit = CollectingAssemblyEmit::default();
let mut pcode_emit = CollectingPcodeEmit::default();
sleigh_builder.asm_emit(&mut asm_emit);
sleigh_builder.pcode_emit(&mut pcode_emit);
let mut sleigh = sleigh_builder.try_build().unwrap();

sleigh.decode(0).unwrap();

println!("{:?}", asm_emit.asms);
println!("{:?}", pcode_emit.pcode_asms);
```

A more detailed documentation of Rust API is still under development.

## QueryCraft (In-Development)

QueryCraft is a SQL based binary analysis, its goal is to allow analyzer write SQL to fetch information (both raw and analyzed) from binary.

This is a currently in development future.

Demo only support for disassembly bytes into table is available. One can do this using the demo:

```sqlite
sqlite> select qc_disasm("bytes", X'319090', "x86", "qc_out_asm", "qc_out_pcode");
1
sqlite> select * from qc_out_asm;
ram|0|XOR|word ptr [BX + SI + 0x90],DX
sqlite> select * from qc_out_pcode;
ram|0|INT_ADD|register|12|2|register|24|2|unique|4736|2|
ram|0|INT_ADD|unique|4736|2|const|144|2|unique|4992|2|
ram|0|CALLOTHER|const|0|4|register|262|2|unique|14336|4|rest_vars: []
ram|0|COPY|const|0|1||||register|512|1|
ram|0|COPY|const|0|1||||register|523|1|
ram|0|LOAD|const|94479501128016|8|unique|14336|4|unique|30848|2|
ram|0|INT_XOR|unique|30848|2|register|8|2|unique|30848|2|
ram|0|STORE|const|94479501128016|8|unique|14336|4||||rest_vars: []
ram|0|LOAD|const|94479501128016|8|unique|14336|4|unique|30848|2|
ram|0|INT_SLESS|unique|30848|2|const|0|2|register|519|1|
ram|0|LOAD|const|94479501128016|8|unique|14336|4|unique|30848|2|
ram|0|INT_EQUAL|unique|30848|2|const|0|2|register|518|1|
ram|0|LOAD|const|94479501128016|8|unique|14336|4|unique|30848|2|
ram|0|INT_AND|unique|30848|2|const|255|2|unique|55552|2|
ram|0|POPCOUNT|unique|55552|2||||unique|55680|1|
ram|0|INT_AND|unique|55680|1|const|1|1|unique|55808|1|
ram|0|INT_EQUAL|unique|55808|1|const|0|1|register|514|1|
```

## In the Future

Currently we are in the early stage of the project.
But we have already planned several goals in the future:

- [x] decoder (linear disassembler) with IR (based on ghidra)
- [ ] encoder (single instruction assemble) (based on ghidra)
- [ ] universal binary analysis algorithms (CFG generation, data flow information)
- [ ] C API/More language bindings
- [ ] PCode emulator
- [ ] Analysis Framework
- [ ] symbolic execution
- [ ] customizable (with DSL, like sleigh to decoder) loader

## About Us

This is a project started by [StarCrossTech](https://www.starcross.tech/#/) ret2lab.

Any contribution through pull request is welcome. ✌️