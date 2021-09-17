use sleighcraft::prelude::*;
use sleighcraft::Mode::{MODE32, MODE64};

// #[test]
// fn test_custom_spec() {
//     // let compiled = include_str!("../test/test.sla");
//     // let mut sleigh = Sleigh::from_spec(compiled).unwrap();
//     // let buf = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
//     // sleigh.decode(0, &buf, 1);
//     // println!("{:?}", sleigh.pcode_emit)
// }

#[test]
fn test_x86() {
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
}

#[test]
fn test_x86_case_ignoring() {
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
}

#[test]
fn test_x86_32_bit() {
    let mut sleigh_builder = SleighBuilder::default();
    let spec = arch("x86").unwrap();
    let buf = [0x90, 0x32, 0x31];
    let mut loader = PlainLoadImage::from_buf(&buf, 0);
    sleigh_builder.loader(&mut loader);
    sleigh_builder.spec(spec);
    sleigh_builder.mode(MODE32);
    let mut asm_emit = CollectingAssemblyEmit::default();
    let mut pcode_emit = CollectingPcodeEmit::default();
    sleigh_builder.asm_emit(&mut asm_emit);
    sleigh_builder.pcode_emit(&mut pcode_emit);
    let mut sleigh = sleigh_builder.try_build().unwrap();

    sleigh.decode(0).unwrap();

    println!("{:?}", asm_emit.asms);
    println!("{:?}", pcode_emit.pcode_asms);
}

#[test]
fn test_x86_64_bit() {
    let mut sleigh_builder = SleighBuilder::default();
    let spec = arch("x86-64").unwrap();
    let buf = [72, 49, 192];
    let mut loader = PlainLoadImage::from_buf(&buf, 0);
    sleigh_builder.loader(&mut loader);
    sleigh_builder.spec(spec);
    sleigh_builder.mode(MODE64);
    let mut asm_emit = CollectingAssemblyEmit::default();
    let mut pcode_emit = CollectingPcodeEmit::default();
    sleigh_builder.asm_emit(&mut asm_emit);
    sleigh_builder.pcode_emit(&mut pcode_emit);
    let mut sleigh = sleigh_builder.try_build().unwrap();

    sleigh.decode(0).unwrap();

    println!("{:?}", asm_emit.asms);
    println!("{:?}", pcode_emit.pcode_asms);
}

#[test]
fn test_mips32le_bit() {
    let mut sleigh_builder = SleighBuilder::default();
    let spec = arch("mips32le").unwrap();
    /*
    0x0 j 8
    0x4 add $1, $2, $3
    0x8 ori $1, $2, 0x64
    */
    let buf = [2, 0, 0, 8, 32, 8, 67, 0, 100, 0, 65, 52];
    let mut loader = PlainLoadImage::from_buf(&buf, 0);
    sleigh_builder.loader(&mut loader);
    sleigh_builder.spec(spec);
    let mut asm_emit = CollectingAssemblyEmit::default();
    let mut pcode_emit = CollectingPcodeEmit::default();
    sleigh_builder.asm_emit(&mut asm_emit);
    sleigh_builder.pcode_emit(&mut pcode_emit);
    let mut sleigh = sleigh_builder.try_build().unwrap();

    sleigh.decode(0).unwrap();

    for asm in asm_emit.asms.iter() {
        println!("{}:\t{}\t{}", asm.addr.offset, asm.mnemonic, asm.body);
    }

    println!();

    for pcode in pcode_emit.pcode_asms.iter() {
        println!("address: {:?}", pcode.addr);
        println!("opcode: {:?}", pcode.opcode);
        println!("vars: {:?}", pcode.vars);
        println!("out_var: {:?}", pcode.out_var);
        println!();
    }
}
