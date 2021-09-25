use sleighcraft::SleighSetting;
use sleighcraft::prelude::*;
use sleighcraft::Mode;

#[test]
fn test_custom_spec() {
    let compiled = include_str!("test.sla");

    let buf = [0x20, 0x30];

    let loader = PlainLoadImage::from_buf(&buf, 0);

    let setting = SleighSetting::with_spec(compiled, Box::new(loader));
    let mut sleigh = Sleigh::with_setting(setting).unwrap();

    sleigh.decode(0, None).unwrap();

    let asm = sleigh
        .asm_emit()
        .as_any()
        .downcast_ref::<CollectingAssemblyEmit>()
        .unwrap();

    let pcode = sleigh
        .pcode_emit()
        .as_any()
        .downcast_ref::<CollectingPcodeEmit>()
        .unwrap();

    assert_eq!(asm.asms.len(), 1);
    assert!(pcode.pcode_asms.len() > 0);
}

#[test]
fn test_x86() {
    let buf = [0x90, 0x32, 0x31];
    let loader = Box::new(PlainLoadImage::from_buf(&buf, 0));

    let mut setting = SleighSetting::with_arch("x86", loader).unwrap();
    setting.mode(Mode::MODE32);
    let mut sleigh = Sleigh::with_setting(setting).unwrap();

    sleigh.decode(0, None).unwrap();

    let asm = sleigh
        .asm_emit()
        .as_any()
        .downcast_ref::<CollectingAssemblyEmit>()
        .unwrap();

    let pcode = sleigh
        .pcode_emit()
        .as_any()
        .downcast_ref::<CollectingPcodeEmit>()
        .unwrap();

    assert!(asm.asms.len() == 2);
    assert!(pcode.pcode_asms.len() > 0);
}

#[test]
fn test_x86_case_ignoring() {
    let buf = [0x90, 0x32, 0x31];
    let loader = Box::new(PlainLoadImage::from_buf(&buf, 0));

    let mut setting = SleighSetting::with_arch("X86", loader).unwrap();
    setting.mode(Mode::MODE32);
    let mut sleigh = Sleigh::with_setting(setting).unwrap();

    sleigh.decode(0, None).unwrap();

    let asm = sleigh
        .asm_emit()
        .as_any()
        .downcast_ref::<CollectingAssemblyEmit>()
        .unwrap();

    let pcode = sleigh
        .pcode_emit()
        .as_any()
        .downcast_ref::<CollectingPcodeEmit>()
        .unwrap();

    assert!(asm.asms.len() == 2);
    assert!(pcode.pcode_asms.len() > 0);
}


#[test]
fn test_x86_64_bit() {
    let buf = [0x90, 0x58];
    let loader = Box::new(PlainLoadImage::from_buf(&buf, 0));
    let mut setting = SleighSetting::with_arch("X86-64", loader).unwrap();
    setting.mode(Mode::MODE64);
    let mut sleigh = Sleigh::with_setting(setting).unwrap();

    sleigh.decode(0, None).unwrap();

    let asm = sleigh
        .asm_emit()
        .as_any()
        .downcast_ref::<CollectingAssemblyEmit>()
        .unwrap();

    let pcode = sleigh
        .pcode_emit()
        .as_any()
        .downcast_ref::<CollectingPcodeEmit>()
        .unwrap();

    assert!(asm.asms.len() == 2);
    assert!(pcode.pcode_asms.len() > 0);
}


#[test]
fn test_mips32le_bit() {
    /*
    0x0 j 8
    0x4 add $1, $2, $3
    0x8 ori $1, $2, 0x64
    */
    let buf = [2, 0, 0, 8, 32, 8, 67, 0, 100, 0, 65, 52];
    let loader = Box::new(PlainLoadImage::from_buf(&buf, 0));
    let mut sleigh = Sleigh::with_arch("mips32le", loader).unwrap();

    sleigh.decode(0, None).unwrap();

    let asm = sleigh
        .asm_emit()
        .as_any()
        .downcast_ref::<CollectingAssemblyEmit>()
        .unwrap();

    let pcode = sleigh
        .pcode_emit()
        .as_any()
        .downcast_ref::<CollectingPcodeEmit>()
        .unwrap();

    assert!(asm.asms.len() > 0);
    assert!(pcode.pcode_asms.len() > 0);
}