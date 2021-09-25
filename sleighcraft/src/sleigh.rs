//
//  Copyright 2021 StarCrossTech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use crate::error::{Error, Result};
use cxx::{CxxString, UniquePtr};
use once_cell::sync::Lazy;
use sleighcraft_util_macro::def_sla_load_preset;
use std::any::Any;
use std::collections::{BTreeSet, HashMap};

#[cxx::bridge]
pub mod ffi {

    enum SpaceType {
        Constant = 0,
        Processor = 1,
        SpaceBase = 2,
        Internal = 3,
        Fspec = 4,
        Iop = 5,
        Join = 6,
    }

    #[derive(Debug)]
    pub enum PcodeOpCode {
        COPY = 1,
        ///< Copy one operand to another
        LOAD = 2,
        ///< Load from a pointer into a specified address space
        STORE = 3,
        ///< Store at a pointer into a specified address space
        BRANCH = 4,
        ///< Always branch
        CBRANCH = 5,
        ///< Conditional branch
        BRANCHIND = 6,
        ///< Indirect branch (jumptable)
        CALL = 7,
        ///< Call to an absolute address
        CALLIND = 8,
        ///< Call through an indirect address
        CALLOTHER = 9,
        ///< User-defined operation
        RETURN = 10,
        ///< Return from subroutine
        // Integer/bit operations
        INT_EQUAL = 11,
        ///< Integer comparison, equality (==)
        INT_NOTEQUAL = 12,
        ///< Integer comparison, in-equality (!=)
        INT_SLESS = 13,
        ///< Integer comparison, signed less-than (<)
        INT_SLESSEQUAL = 14,
        ///< Integer comparison, signed less-than-or-equal (<=)
        INT_LESS = 15,
        ///< Integer comparison, unsigned less-than (<)
        // This also indicates a borrow on unsigned substraction
        INT_LESSEQUAL = 16,
        ///< Integer comparison, unsigned less-than-or-equal (<=)
        INT_ZEXT = 17,
        ///< Zero extension
        INT_SEXT = 18,
        ///< Sign extension
        INT_ADD = 19,
        ///< Addition, signed or unsigned (+)
        INT_SUB = 20,
        ///< Subtraction, signed or unsigned (-)
        INT_CARRY = 21,
        ///< Test for unsigned carry
        INT_SCARRY = 22,
        ///< Test for signed carry
        INT_SBORROW = 23,
        ///< Test for signed borrow
        INT_2COMP = 24,
        ///< Twos complement
        INT_NEGATE = 25,
        ///< Logical/bitwise negation (~)
        INT_XOR = 26,
        ///< Logical/bitwise exclusive-or (^)
        INT_AND = 27,
        ///< Logical/bitwise and (&)
        INT_OR = 28,
        ///< Logical/bitwise or (|)
        INT_LEFT = 29,
        ///< Left shift (<<)
        INT_RIGHT = 30,
        ///< Right shift, logical (>>)
        INT_SRIGHT = 31,
        ///< Right shift, arithmetic (>>)
        INT_MULT = 32,
        ///< Integer multiplication, signed and unsigned (*)
        INT_DIV = 33,
        ///< Integer division, unsigned (/)
        INT_SDIV = 34,
        ///< Integer division, signed (/)
        INT_REM = 35,
        ///< Remainder/modulo, unsigned (%)
        INT_SREM = 36,
        ///< Remainder/modulo, signed (%)
        BOOL_NEGATE = 37,
        ///< Boolean negate (!)
        BOOL_XOR = 38,
        ///< Boolean exclusive-or (^^)
        BOOL_AND = 39,
        ///< Boolean and (&&)
        BOOL_OR = 40,
        ///< Boolean or (||)
        // Floating point operations
        FLOAT_EQUAL = 41,
        ///< Floating-point comparison, equality (==)
        FLOAT_NOTEQUAL = 42,
        ///< Floating-point comparison, in-equality (!=)
        FLOAT_LESS = 43,
        ///< Floating-point comparison, less-than (<)
        FLOAT_LESSEQUAL = 44,
        ///< Floating-point comparison, less-than-or-equal (<=)
        // Slot 45 is currently unused
        FLOAT_NAN = 46,
        ///< Not-a-number test (NaN)
        FLOAT_ADD = 47,
        ///< Floating-point addition (+)
        FLOAT_DIV = 48,
        ///< Floating-point division (/)
        FLOAT_MULT = 49,
        ///< Floating-point multiplication (*)
        FLOAT_SUB = 50,
        ///< Floating-point subtraction (-)
        FLOAT_NEG = 51,
        ///< Floating-point negation (-)
        FLOAT_ABS = 52,
        ///< Floating-point absolute value (abs)
        FLOAT_SQRT = 53,
        ///< Floating-point square root (sqrt)
        FLOAT_INT2FLOAT = 54,
        ///< Convert an integer to a floating-point
        FLOAT_FLOAT2FLOAT = 55,
        ///< Convert between different floating-point sizes
        FLOAT_TRUNC = 56,
        ///< Round towards zero
        FLOAT_CEIL = 57,
        ///< Round towards +infinity
        FLOAT_FLOOR = 58,
        ///< Round towards -infinity
        FLOAT_ROUND = 59,
        ///< Round towards nearest
        // Internal opcodes for simplification. Not
        // typically generated in a direct translation.

        // Data-flow operations
        MULTIEQUAL = 60,
        ///< Phi-node operator
        INDIRECT = 61,
        ///< Copy with an indirect effect
        PIECE = 62,
        ///< Concatenate
        SUBPIECE = 63,
        ///< Truncate
        CAST = 64,
        ///< Cast from one data-type to another
        PTRADD = 65,
        ///< Index into an array ([])
        PTRSUB = 66,
        ///< Drill down to a sub-field  (->)
        SEGMENTOP = 67,
        ///< Look-up a \e segmented address
        CPOOLREF = 68,
        ///< Recover a value from the \e constant \e pool
        NEW = 69,
        ///< Allocate a new object (new)
        INSERT = 70,
        ///< Insert a bit-range
        EXTRACT = 71,
        ///< Extract a bit-range
        POPCOUNT = 72,
        ///< Count the 1-bits
        MAX = 73,
    }

    extern "Rust" {
        type RustAssemblyEmit;
        fn dump(
            self: &mut RustAssemblyEmit,
            address: &AddressProxy,
            mnem: &CxxString,
            body: &CxxString,
        );
        type RustPcodeEmit;
        fn dump(
            self: &mut RustPcodeEmit,
            address: &AddressProxy,
            opcode: PcodeOpCode,
            outvar: Pin<&mut VarnodeDataProxy>,
            vars: &CxxVector<VarnodeDataProxy>,
        );
        type RustLoadImage;
        fn load_fill(self: &mut RustLoadImage, ptr: &mut [u8], addr: &AddressProxy);
        //fn get_arch_type(self: &RustLoadImage) -> String;
        fn adjust_vma(self: &mut RustLoadImage, adjust: isize);
        fn buf_size(self: &mut RustLoadImage) -> usize;

        type Instruction;
        fn set_addr(self: &mut Instruction, space: String, offset: u64);
        fn set_mnemonic(self: &mut Instruction, mnem: String);
        fn set_body(self: &mut Instruction, body: String);

    }

    unsafe extern "C++" {
        include!("sleighcraft/src/cpp/bridge/disasm.h");
        include!("sleighcraft/src/cpp/bridge/proxies.h");
        type OpBehaviorProxy;
        type CoverProxy;
        type TypeOpProxy;
        fn get_name(self: &TypeOpProxy) -> &CxxString;
        fn get_opcode(self: &TypeOpProxy) -> PcodeOpCode;
        fn get_flags(self: &TypeOpProxy) -> u32;
        fn get_behavior(self: &TypeOpProxy) -> UniquePtr<OpBehaviorProxy>;
        fn evaluate_unary(self: &TypeOpProxy, sizeout: i32, sizein: i32, in1: usize) -> usize;
        fn evaluate_binary(
            self: &TypeOpProxy,
            sizeout: i32,
            sizein: i32,
            in1: usize,
            in2: usize,
        ) -> usize;
        fn recover_input_binary(
            self: &TypeOpProxy,
            slot: i32,
            sizeout: i32,
            sout: usize,
            sizein: i32,
            sin: usize,
        ) -> usize;
        fn recover_input_unary(self: &TypeOpProxy, sizeout: i32, out: usize, sizein: i32) -> usize;
        fn is_commutative(self: &TypeOpProxy) -> bool;
        fn inherits_sign(self: &TypeOpProxy) -> bool;

        type OpCodeProxy;

        fn count(self: &OpCodeProxy) -> i32;
        fn get_out(self: &OpCodeProxy) -> UniquePtr<VarnodeProxy>;
        fn get_in(self: &OpCodeProxy, slot: i32) -> UniquePtr<VarnodeProxy>;
        fn get_time(self: &OpCodeProxy) -> u32;
        fn get_eval_type(self: &OpCodeProxy) -> u32;
        fn get_halt_type(self: &OpCodeProxy) -> u32;
        fn is_dead(self: &OpCodeProxy) -> bool;
        fn is_assignment(self: &OpCodeProxy) -> bool;
        fn is_call(self: &OpCodeProxy) -> bool;
        fn is_call_without_spec(self: &OpCodeProxy) -> bool;
        fn is_marker(self: &OpCodeProxy) -> bool;
        fn is_indirect_creation(self: &OpCodeProxy) -> bool;
        fn is_indirect_store(self: &OpCodeProxy) -> bool;
        fn not_printed(self: &OpCodeProxy) -> bool;
        fn is_bool_output(self: &OpCodeProxy) -> bool;
        fn is_branch(self: &OpCodeProxy) -> bool;
        fn is_call_or_branch(self: &OpCodeProxy) -> bool;
        fn is_flow_break(self: &OpCodeProxy) -> bool;
        fn is_boolean_flip(self: &OpCodeProxy) -> bool;
        fn is_fallthru_true(self: &OpCodeProxy) -> bool;
        fn is_code_ref(self: &OpCodeProxy) -> bool;
        fn is_instruction_start(self: &OpCodeProxy) -> bool;
        fn is_block_start(self: &OpCodeProxy) -> bool;
        fn is_modified(self: &OpCodeProxy) -> bool;
        fn is_mark(self: &OpCodeProxy) -> bool;
        fn set_mark(self: &OpCodeProxy);
        fn is_warning(self: &OpCodeProxy) -> bool;
        fn clear_mark(self: &OpCodeProxy);
        fn is_indirect_source(self: &OpCodeProxy) -> bool;
        fn set_indirect_source(self: &OpCodeProxy);
        fn clear_indirect_source(self: &OpCodeProxy);
        fn is_ptr_flow(self: &OpCodeProxy) -> bool;
        fn set_ptr_flow(self: &OpCodeProxy);
        fn is_splitting(self: &OpCodeProxy) -> bool;
        fn does_special_propagation(self: &OpCodeProxy) -> bool;
        fn does_special_printing(self: &OpCodeProxy) -> bool;
        fn is_incidental_copy(self: &OpCodeProxy) -> bool;
        fn is_calculated_bool(self: &OpCodeProxy) -> bool;
        fn is_cpool_transformed(self: &OpCodeProxy) -> bool;
        fn uses_spacebase_ptr(self: &OpCodeProxy) -> bool;
        fn get_cse_hash(self: &OpCodeProxy) -> u32;
        fn get_opcode(self: &OpCodeProxy) -> UniquePtr<TypeOpProxy>;
        fn get_code(self: &OpCodeProxy) -> PcodeOpCode;
        fn is_commutative(self: &OpCodeProxy) -> bool;
        fn next_op(self: &OpCodeProxy) -> UniquePtr<CoverProxy>;
        fn previous_op(self: &OpCodeProxy) -> UniquePtr<CoverProxy>;
        fn get_start_op(self: &OpCodeProxy) -> UniquePtr<CoverProxy>;

        type VariableProxy;
        type VarnodeProxy;
        fn set_high(self: &VarnodeProxy, tv: &VariableProxy, mg: i16);
        fn get_addr(self: &VarnodeProxy) -> UniquePtr<AddressProxy>;
        fn get_space(self: &VarnodeProxy) -> UniquePtr<AddrSpaceProxy>;
        fn get_offset(self: &VarnodeProxy) -> usize;
        fn get_size(self: &VarnodeProxy) -> i32;
        fn get_merge_group(self: &VarnodeProxy) -> i16;
        fn get_def(self: &VarnodeProxy) -> UniquePtr<CoverProxy>;
        fn get_high(self: &VarnodeProxy) -> UniquePtr<VariableProxy>;
        fn equals(self: &VarnodeProxy, op2: &VarnodeProxy) -> bool;
        fn not_equal(self: &VarnodeProxy, op2: &VarnodeProxy) -> bool;
        fn less_than(self: &VarnodeProxy, op2: &VarnodeProxy) -> bool;

        type VarnodeDataProxy;

        fn get_addr(self: &VarnodeDataProxy) -> UniquePtr<AddressProxy>;
        fn is_contains(self: &VarnodeDataProxy, op2: &VarnodeDataProxy) -> bool;
        fn get_offset(self: &VarnodeDataProxy) -> usize;
        fn get_size(self: &VarnodeDataProxy) -> u32;
        fn get_space(self: &VarnodeDataProxy) -> UniquePtr<AddrSpaceProxy>;
        fn not_null(self: &VarnodeDataProxy) -> bool;

        type AddrSpaceProxy;
        fn get_name(self: &AddrSpaceProxy) -> &CxxString;
        fn get_type(self: &AddrSpaceProxy) -> SpaceType;
        fn get_delay(self: &AddrSpaceProxy) -> i32;
        fn get_deadcode_delay(self: &AddrSpaceProxy) -> i32;
        fn get_index(self: &AddrSpaceProxy) -> i32;
        fn get_wordsize(self: &AddrSpaceProxy) -> u32;
        fn get_addrsize(self: &AddrSpaceProxy) -> u32;
        fn get_highest(self: &AddrSpaceProxy) -> usize;
        fn get_pointer_lower_bound(self: &AddrSpaceProxy) -> usize;
        fn get_pointer_upper_bound(self: &AddrSpaceProxy) -> usize;
        fn get_minimum_ptr_size(self: &AddrSpaceProxy) -> i32;
        fn wrap_offset(self: &AddrSpaceProxy, off: usize) -> usize;
        fn get_shortcut(self: &AddrSpaceProxy) -> i8;
        fn is_heritaged(self: &AddrSpaceProxy) -> bool;
        fn does_deadcode(self: &AddrSpaceProxy) -> bool;
        fn has_physical(self: &AddrSpaceProxy) -> bool;
        fn is_big_endian(self: &AddrSpaceProxy) -> bool;
        fn is_reverse_justified(self: &AddrSpaceProxy) -> bool;
        fn is_overlay(self: &AddrSpaceProxy) -> bool;
        fn is_overlay_base(self: &AddrSpaceProxy) -> bool;
        fn is_other_space(self: &AddrSpaceProxy) -> bool;
        fn is_truncated(self: &AddrSpaceProxy) -> bool;
        fn has_near_pointers(self: &AddrSpaceProxy) -> bool;
        // fn print_offset(self: &AddrSpaceProxy ,s: buffer, offset: usize);
        fn num_spacebase(self: &AddrSpaceProxy) -> i32;
        // fn get_spacebase(self: &AddrSpaceProxy, i: i32) -> UniquePtr<VarnodeDataProxy>;
        // fn get_spacebase_full(self: &AddrSpaceProxy, i: i32) -> UniquePtr<VarnodeDataProxy>;

        type AddressProxy;
        fn is_invalid(self: &AddressProxy) -> bool;
        fn get_addr_size(self: &AddressProxy) -> i32;
        fn is_big_endian(self: &AddressProxy) -> bool;
        fn get_space(self: &AddressProxy) -> UniquePtr<AddrSpaceProxy>;
        fn get_offset(self: &AddressProxy) -> usize;
        fn to_physical(self: Pin<&mut AddressProxy>);
        fn get_shortcut(self: &AddressProxy) -> i8;
        fn equals(self: &AddressProxy, op2: &AddressProxy) -> bool;
        fn not_equal(self: &AddressProxy, op2: &AddressProxy) -> bool;
        fn less_than(self: &AddressProxy, op2: &AddressProxy) -> bool;
        fn less_equal(self: &AddressProxy, op2: &AddressProxy) -> bool;
        fn add(self: &AddressProxy, off: i32) -> UniquePtr<AddressProxy>;
        fn sub(self: &AddressProxy, off: i32) -> UniquePtr<AddressProxy>;
        fn contained_by(self: &AddressProxy, size: i32, op2: &AddressProxy, size2: i32) -> bool;
        fn justified_contain(
            self: &AddressProxy,
            size: i32,
            op2: &AddressProxy,
            size2: i32,
            forceleft: bool,
        ) -> i32;
        fn overlap(self: &AddressProxy, skip: i32, op: &AddressProxy, size: i32) -> i32;
        fn is_contiguous(self: &AddressProxy, size: i32, loaddr: &AddressProxy, losz: i32) -> bool;
        fn is_constant(self: &AddressProxy) -> bool;
        fn renormalize(self: Pin<&mut AddressProxy>, size: i32);
        fn is_join(self: &AddressProxy) -> bool;
        fn addr_get_space_from_const(addr: &AddressProxy) -> UniquePtr<AddrSpaceProxy>;

        type RustLoadImageProxy;

        fn from_rust(load_iamge: Box<RustLoadImage>) -> UniquePtr<RustLoadImageProxy>;

        // type InstructionProxy;
        //
        // fn get_space(self: &InstructionProxy) -> &CxxString;
        // fn get_offset(self: &InstructionProxy) -> u64;
        // fn get_mnemonic(self: &InstructionProxy) -> &CxxString;
        // fn get_body(self: &InstructionProxy) -> &CxxString;

        type SleighProxy;
        fn set_spec(self: Pin<&mut SleighProxy>, spec_content: &str, mode: i32);
        fn new_sleigh_proxy(ld: Box<RustLoadImage>) -> UniquePtr<SleighProxy>;
        fn decode_asm_at(self: Pin<&mut SleighProxy>, start: u64) -> Result<i32>;
        fn decode_pcode_at(self: Pin<&mut SleighProxy>, start: u64) -> Result<()>;
        fn set_loader(self: Pin<&mut SleighProxy>, load: Box<RustLoadImage>);
        fn set_asm_emit(self: Pin<&mut SleighProxy>, asm_emit: Box<RustAssemblyEmit>);
        fn set_pcode_emit(self: Pin<&mut SleighProxy>, pcode_emit: Box<RustPcodeEmit>);
        fn get_loader_mut(self: Pin<&mut SleighProxy>) -> &mut Box<RustLoadImage>;
        fn get_asm_emit_mut(self: Pin<&mut SleighProxy>) -> &mut Box<RustAssemblyEmit>;
        fn get_pcode_emit_mut(self: Pin<&mut SleighProxy>) -> &mut Box<RustPcodeEmit>;

        fn get_loader(self: &SleighProxy) -> &Box<RustLoadImage>;
        fn get_asm_emit(self: &SleighProxy) -> &Box<RustAssemblyEmit>;
        fn get_pcode_emit(self: &SleighProxy) -> &Box<RustPcodeEmit>;
    }
}

use ffi::*;
use num_enum::TryFromPrimitive;
use std::pin::Pin;

impl ToString for PcodeOpCode {
    fn to_string(&self) -> String {
        match *self {
            PcodeOpCode::COPY => String::from("COPY"),
            PcodeOpCode::LOAD => String::from("LOAD"),
            PcodeOpCode::STORE => String::from("STORE"),
            PcodeOpCode::BRANCH => String::from("BRANCH"),
            PcodeOpCode::CBRANCH => String::from("CBRANCH"),
            PcodeOpCode::BRANCHIND => String::from("BRANCHIND"),
            PcodeOpCode::CALL => String::from("CALL"),
            PcodeOpCode::CALLIND => String::from("CALLIND"),
            PcodeOpCode::CALLOTHER => String::from("CALLOTHER"),
            PcodeOpCode::RETURN => String::from("RETURN"),
            PcodeOpCode::INT_EQUAL => String::from("INT_EQUAL"),
            PcodeOpCode::INT_NOTEQUAL => String::from("INT_NOTEQUAL"),
            PcodeOpCode::INT_SLESS => String::from("INT_SLESS"),
            PcodeOpCode::INT_SLESSEQUAL => String::from("INT_SLESSEQUAL"),
            PcodeOpCode::INT_LESS => String::from("INT_LESS"),
            PcodeOpCode::INT_LESSEQUAL => String::from("INT_LESSEQUAL"),
            PcodeOpCode::INT_ZEXT => String::from("INT_ZEXT"),
            PcodeOpCode::INT_SEXT => String::from("INT_SEXT"),
            PcodeOpCode::INT_ADD => String::from("INT_ADD"),
            PcodeOpCode::INT_SUB => String::from("INT_SUB"),
            PcodeOpCode::INT_CARRY => String::from("INT_CARRY"),
            PcodeOpCode::INT_SCARRY => String::from("INT_SCARRY"),
            PcodeOpCode::INT_SBORROW => String::from("INT_SBORROW"),
            PcodeOpCode::INT_2COMP => String::from("INT_2COMP"),
            PcodeOpCode::INT_NEGATE => String::from("INT_NEGATE"),
            PcodeOpCode::INT_XOR => String::from("INT_XOR"),
            PcodeOpCode::INT_AND => String::from("INT_AND"),
            PcodeOpCode::INT_OR => String::from("INT_OR"),
            PcodeOpCode::INT_LEFT => String::from("INT_LEFT"),
            PcodeOpCode::INT_RIGHT => String::from("INT_RIGHT"),
            PcodeOpCode::INT_SRIGHT => String::from("INT_SRIGHT"),
            PcodeOpCode::INT_MULT => String::from("INT_MULT"),
            PcodeOpCode::INT_DIV => String::from("INT_DIV"),
            PcodeOpCode::INT_SDIV => String::from("INT_SDIV"),
            PcodeOpCode::INT_REM => String::from("INT_REM"),
            PcodeOpCode::INT_SREM => String::from("INT_SREM"),
            PcodeOpCode::BOOL_NEGATE => String::from("BOOL_NEGATE"),
            PcodeOpCode::BOOL_XOR => String::from("BOOL_XOR"),
            PcodeOpCode::BOOL_AND => String::from("BOOL_AND"),
            PcodeOpCode::BOOL_OR => String::from("BOOL_OR"),
            PcodeOpCode::FLOAT_EQUAL => String::from("FLOAT_EQUAL"),
            PcodeOpCode::FLOAT_NOTEQUAL => String::from("FLOAT_NOTEQUAL"),
            PcodeOpCode::FLOAT_LESS => String::from("FLOAT_LESS"),
            PcodeOpCode::FLOAT_LESSEQUAL => String::from("FLOAT_LESSEQUAL"),
            PcodeOpCode::FLOAT_NAN => String::from("FLOAT_NAN"),
            PcodeOpCode::FLOAT_ADD => String::from("FLOAT_ADD"),
            PcodeOpCode::FLOAT_DIV => String::from("FLOAT_DIV"),
            PcodeOpCode::FLOAT_MULT => String::from("FLOAT_MULT"),
            PcodeOpCode::FLOAT_SUB => String::from("FLOAT_SUB"),
            PcodeOpCode::FLOAT_NEG => String::from("FLOAT_NEG"),
            PcodeOpCode::FLOAT_ABS => String::from("FLOAT_ABS"),
            PcodeOpCode::FLOAT_SQRT => String::from("FLOAT_SQRT"),
            PcodeOpCode::FLOAT_INT2FLOAT => String::from("FLOAT_INT2FLOAT"),
            PcodeOpCode::FLOAT_FLOAT2FLOAT => String::from("FLOAT_FLOAT2FLOAT"),
            PcodeOpCode::FLOAT_TRUNC => String::from("FLOAT_TRUNC"),
            PcodeOpCode::FLOAT_CEIL => String::from("FLOAT_CEIL"),
            PcodeOpCode::FLOAT_FLOOR => String::from("FLOAT_FLOOR"),
            PcodeOpCode::FLOAT_ROUND => String::from("FLOAT_ROUND"),
            PcodeOpCode::MULTIEQUAL => String::from("MULTIEQUAL"),
            PcodeOpCode::INDIRECT => String::from("INDIRECT"),
            PcodeOpCode::PIECE => String::from("PIECE"),
            PcodeOpCode::SUBPIECE => String::from("SUBPIECE"),
            PcodeOpCode::CAST => String::from("CAST"),
            PcodeOpCode::PTRADD => String::from("PTRADD"),
            PcodeOpCode::PTRSUB => String::from("PTRSUB"),
            PcodeOpCode::SEGMENTOP => String::from("SEGMENTOP"),
            PcodeOpCode::CPOOLREF => String::from("CPOOLREF"),
            PcodeOpCode::NEW => String::from("NEW"),
            PcodeOpCode::INSERT => String::from("INSERT"),
            PcodeOpCode::EXTRACT => String::from("EXTRACT"),
            PcodeOpCode::POPCOUNT => String::from("POPCOUNT"),
            PcodeOpCode::MAX => String::from("MAX"),
            _ => unreachable!(),
        }
    }
}

#[derive(TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum Mode {
    // Default Address size is 16-bit
    MODE16 = 0,
    // Address size is 32-bit
    MODE32 = 1,
    // Address size is 32-bit
    MODE64 = 2,
}

impl Default for Mode {
    fn default() -> Self {
        Self::MODE16
    }
}

pub trait AssemblyEmit {
    fn dump(&mut self, addr: &AddressProxy, mnem: &str, body: &str);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct RustAssemblyEmit {
    internal: Box<dyn AssemblyEmit>,
}

impl RustAssemblyEmit {
    pub fn dump(&mut self, address: &AddressProxy, mnem: &CxxString, body: &CxxString) {
        let mnem = mnem.to_str().unwrap();
        let body = body.to_str().unwrap();

        self.internal.dump(address, mnem, body);
    }

    pub fn from_internal(internal: Box<dyn AssemblyEmit>) -> Self {
        Self { internal }
    }

    pub fn internal_ref(&self) -> &Box<dyn AssemblyEmit> {
        &self.internal
    }

    pub fn internal_mut(&mut self) -> &mut Box<dyn AssemblyEmit> {
        &mut self.internal
    }
}

#[derive(Debug, Default)]
pub struct CollectingAssemblyEmit {
    pub asms: Vec<Instruction>,
}

impl AssemblyEmit for CollectingAssemblyEmit {
    fn dump(&mut self, addr: &AddressProxy, mnem: &str, body: &str) {
        let space = addr.get_space().get_name().to_str().unwrap().to_string();
        let offset = addr.get_offset() as u64;
        let asm = Instruction {
            addr: Address { space, offset },
            mnemonic: mnem.to_string(),
            body: body.to_string(),
        };
        self.asms.push(asm)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


impl CollectingAssemblyEmit {
    pub fn correspond<'a>(&'a mut self, pcodes: &'a CollectingPcodeEmit) -> CorrespondedCollectedAsm<'a> {
        let mut idx_map = HashMap::new();
        for asm_idx in 0..self.asms.len() {
            let asm = &self.asms[asm_idx];
            let mut pcode_indices = BTreeSet::new();
            for i in 0..pcodes.pcode_asms.len() {
                let addr = &pcodes.pcode_asms[i].addr;
                if pcodes.pcode_asms[i].addr == asm.addr {
                    pcode_indices.insert(PcodeAddrIndex {
                        pcode_vec_idx: i,
                        offset: addr.offset,
                        seq: pcodes.pcode_asms[i].seq
                    });
                }
            }

            idx_map.insert(asm_idx, pcode_indices);
        }

        CorrespondedCollectedAsm {
            asm_emit: self,
            pcode_emit: pcodes,
            idx_map
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct PcodeAddrIndex {
    /// index into pcode table
    pcode_vec_idx: usize,
    /// addr sequence, to properly order the pcodes
    offset: u64,
    seq: u64
}

impl PartialOrd for PcodeAddrIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.offset == other.offset {
            self.seq.partial_cmp(&other.seq)
        } else {
            self.offset.partial_cmp(&other.offset)
        }
    }
}

impl Ord for PcodeAddrIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone)]
pub struct CorrespondedCollectedAsm<'a> {
    asm_emit: &'a CollectingAssemblyEmit,
    pcode_emit: &'a CollectingPcodeEmit,
    /// asm => range of inst in pcode table.
    idx_map: HashMap<usize, BTreeSet<PcodeAddrIndex>>,
}

impl<'a> CorrespondedCollectedAsm<'a> {
    pub fn asms(&self) -> &Vec<Instruction> {
        &self.asm_emit.asms
    }

    pub fn pcodes(&self) -> &Vec<PcodeInstruction> {
        &self.pcode_emit.pcode_asms
    }

    pub fn pcode_indices_of_asm(&self, asm_idx: usize) -> BTreeSet<usize> {
        self.idx_map[&asm_idx].iter().map(|i| i.pcode_vec_idx).collect()
    }
}


#[derive(Debug)]
pub struct PcodeVarnodeData {
    pub space: String,
    pub offset: usize,
    pub size: u32,
}

impl PcodeVarnodeData {
    pub fn from_proxy(proxy: &VarnodeDataProxy) -> Self {
        let space = String::from(proxy.get_space().get_name().to_str().unwrap());
        let offset = proxy.get_offset();
        let size = proxy.get_size();

        Self {
            space,
            offset,
            size,
        }
    }
}

#[derive(Debug)]
pub struct PcodeInstruction {
    pub addr: Address,
    pub seq: u64,
    pub opcode: PcodeOpCode,
    pub vars: Vec<PcodeVarnodeData>,
    pub out_var: Option<PcodeVarnodeData>,
}

pub trait PcodeEmit {
    /// Callback that will be called when disassembling, emitting the pcode
    /// - address: the address of the machine instruction
    /// - opcode: the opcode of the particular pcode instruction
    /// - outvar: a data about the output varnode
    /// - vars: an array of VarnodeData for each input varnode
    fn dump(
        &mut self,
        address: &AddressProxy,
        opcode: PcodeOpCode,
        outvar: Option<&VarnodeDataProxy>,
        vars: &[&VarnodeDataProxy],
    );
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Default)]
pub struct CollectingPcodeEmit {
    pub pcode_asms: Vec<PcodeInstruction>,
    seq: u64,
    last_offset: u64,
}

impl PcodeEmit for CollectingPcodeEmit {
    fn dump(
        &mut self,
        addr: &AddressProxy,
        opcode: PcodeOpCode,
        outvar: Option<&VarnodeDataProxy>,
        vars: &[&VarnodeDataProxy],
    ) {
        let space = String::from(addr.get_space().get_name().to_str().unwrap());
        let offset = addr.get_offset() as u64;
        let mut pcode_vars = vec![];
        for v in vars.iter() {
            pcode_vars.push(PcodeVarnodeData::from_proxy(*v));
        }
        let out_var = if let Some(outvar) = outvar {
            Some(PcodeVarnodeData::from_proxy(outvar))
        } else {
            None
        };

        if self.last_offset != offset {
            self.seq = 0;
        }

        self.pcode_asms.push(PcodeInstruction {
            addr: Address { space, offset },
            opcode: opcode,
            vars: pcode_vars,
            seq: self.seq,
            out_var,
        });

        self.seq += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct RustPcodeEmit {
    pub internal: Box<dyn PcodeEmit>,
}

impl RustPcodeEmit {
    pub fn from_internal(internal: Box<dyn PcodeEmit>) -> Self {
        Self { internal }
    }

    pub fn internal_ref(&self) -> &Box<dyn PcodeEmit> {
        &self.internal
    }

    pub fn internal_mut(&mut self) -> &mut Box<dyn PcodeEmit> {
        &mut self.internal
    }

    pub fn dump(
        &mut self,
        address: &AddressProxy,
        opcode: PcodeOpCode,
        outvar: Pin<&mut VarnodeDataProxy>,
        vars: &cxx::CxxVector<VarnodeDataProxy>,
    ) {
        let outvar = if outvar.not_null() {
            Some(&*outvar)
        } else {
            None
        };

        let mut vars_vec = vec![];
        for i in 0..vars.len() {
            vars_vec.push(vars.get(i).unwrap());
        }
        self.internal
            .dump(address, opcode, outvar, vars_vec.as_slice());
    }
}

pub trait LoadImage {
    fn load_fill(&mut self, ptr: &mut [u8], addr: &AddressProxy);
    fn adjust_vma(&mut self, _adjust: isize) {}
    fn buf_size(&mut self) -> usize;
}

pub struct RustLoadImage {
    internal: Box<dyn LoadImage>,
}

impl RustLoadImage {
    pub fn from_internal(internal: Box<dyn LoadImage>) -> Self {
        Self { internal }
    }

    pub fn internal_ref(&self) -> &Box<dyn LoadImage> {
        &self.internal
    }

    pub fn internal_mut(&mut self) -> &mut Box<dyn LoadImage> {
        &mut self.internal
    }

    pub fn load_fill(&mut self, ptr: &mut [u8], addr: &AddressProxy) {
        self.internal.load_fill(ptr, addr)
    }

    pub fn adjust_vma(&mut self, adjust: isize) {
        self.internal.adjust_vma(adjust)
    }
    pub fn buf_size(&mut self) -> usize {
        self.internal.buf_size()
    }
}

#[derive(Debug, Default)]
pub struct PlainLoadImage {
    buf: Vec<u8>,
    start: u64,
}

impl LoadImage for PlainLoadImage {
    fn load_fill(&mut self, ptr: &mut [u8], addr: &AddressProxy) {
        let start_off = addr.get_offset() as u64;
        let size = ptr.len();
        let max = self.start + (self.buf.len() as u64 - 1);

        for i in 0..size {
            let cur_off = start_off + i as u64;
            if self.start <= cur_off && max >= cur_off {
                let offset = (cur_off - self.start) as usize;
                ptr[i] = self.buf[offset];
            } else {
                ptr[i] = 0;
            }
        }
    }
    fn buf_size(&mut self) -> usize {
        self.buf.len()
    }
}

impl PlainLoadImage {
    pub fn from_buf(buf: &[u8], start: u64) -> Self {
        let mut v = vec![];
        v.extend_from_slice(buf);
        Self { buf: v, start }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub space: String,
    pub offset: u64,
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.space == other.space {
            self.offset.partial_cmp(&other.offset)
        } else {
            None
        }
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("incorrect address: not comparable")
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub addr: Address,
    pub mnemonic: String,
    pub body: String,
}

impl Instruction {
    pub fn set_addr(&mut self, sp: String, of: u64) {
        self.addr = Address {
            space: sp,
            offset: of,
        }
    }
    pub fn set_mnemonic(&mut self, mnem: String) {
        self.mnemonic = mnem
    }
    pub fn set_body(&mut self, body: String) {
        self.body = body
    }
}

// relative to root?
def_sla_load_preset!("sleighcraft/sla/", fn load_preset() -> HashMap<&'static str, &'static str>);

const PRESET: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| load_preset());

pub struct SleighSetting {
    asm_emit: Box<dyn AssemblyEmit>,
    pcode_emit: Box<dyn PcodeEmit>,
    loader: Box<dyn LoadImage>,
    spec: String,
    mode: Mode,
}

impl SleighSetting {
    pub fn new(loader: Box<dyn LoadImage>) -> Self {
        let asm_emit = Box::new(CollectingAssemblyEmit::default());
        let pcode_emit = Box::new(CollectingPcodeEmit::default());
        let spec = arch_spec("x86-64")
            .expect("unable to find default arch: x86-64")
            .to_string();
        let mode = Mode::default();

        Self {
            asm_emit,
            pcode_emit,
            loader,
            spec,
            mode,
        }
    }

    pub fn with_spec(spec: &str, loader: Box<dyn LoadImage>) -> Self {
        let mut setting = SleighSetting::new(loader);
        setting.spec(spec);
        setting
    }

    pub fn with_arch(arch: &str, loader: Box<dyn LoadImage>) -> Result<Self> {
        let mut setting = SleighSetting::new(loader);
        setting.arch(arch)?;
        Ok(setting)
    }

    pub fn asm_emit(&mut self, emit: Box<dyn AssemblyEmit>) -> &mut Self {
        self.asm_emit = emit;
        self
    }

    pub fn pcode_emit(&mut self, emit: Box<dyn PcodeEmit>) -> &mut Self {
        self.pcode_emit = emit;
        self
    }

    pub fn spec(&mut self, spec_str: &str) -> &mut Self {
        self.spec = spec_str.to_string();
        self
    }

    pub fn arch(&mut self, arch_name: &str) -> Result<&mut Self> {
        self.spec = arch_spec(arch_name)?.to_string();
        Ok(self)
    }

    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.mode = mode;
        self
    }
}

pub struct Sleigh {
    sleigh_proxy: UniquePtr<ffi::SleighProxy>,
}

impl Sleigh {
    pub fn new(loader: Box<dyn LoadImage>) -> Result<Self> {
        let setting = SleighSetting::new(loader);
        Self::with_setting(setting)
    }

    pub fn with_setting(setting: SleighSetting) -> Result<Self> {
        let SleighSetting {
            asm_emit,
            pcode_emit,
            loader,
            spec,
            mode,
        } = setting;

        let asm_emit = Box::new(RustAssemblyEmit::from_internal(asm_emit));
        let pcode_emit = Box::new(RustPcodeEmit::from_internal(pcode_emit));
        let loader = Box::new(RustLoadImage::from_internal(loader));

        let mut sleigh_proxy = new_sleigh_proxy(loader);
        sleigh_proxy
            .as_mut()
            .unwrap()
            .set_spec(spec.as_str(), mode as i32);

        sleigh_proxy.as_mut().unwrap().set_asm_emit(asm_emit);

        sleigh_proxy.as_mut().unwrap().set_pcode_emit(pcode_emit);

        Ok(Self { sleigh_proxy })
    }

    pub fn with_arch(arch_name: &str, loader: Box<dyn LoadImage>) -> Result<Self> {
        let mut setting = SleighSetting::new(loader);
        setting.arch(arch_name)?;
        Self::with_setting(setting)
    }

    pub fn with_spec(spec: &str, loader: Box<dyn LoadImage>) -> Result<Self> {
        let mut setting = SleighSetting::new(loader);
        setting.spec(spec);
        Self::with_setting(setting)
    }

    pub fn decode_asm_at(&mut self, addr: u64) -> Result<i32> {
        self.sleigh_proxy
            .as_mut()
            .unwrap()
            .decode_asm_at(addr)
            .map_err(|e| Error::CppException(e))
    }

    pub fn decode_pcode_at(&mut self, addr: u64) -> Result<()> {
        self.sleigh_proxy
            .as_mut()
            .unwrap()
            .decode_pcode_at(addr)
            .map_err(|e| Error::CppException(e))
    }

    pub fn decode_at(&mut self, addr: u64) -> Result<i32> {
        let length = self.decode_asm_at(addr)?;
        self.decode_pcode_at(addr)?;
        Ok(length)
    }

    pub fn decode(&mut self, mut start: u64, inst_size: Option<u64>) -> Result<()> {
        let inst_size = if inst_size.is_none() {
            0
        } else {
            inst_size.unwrap()
        };
        let buf_size = self.load_image_mut().buf_size();
        let mut buf_used = 0;
        let mut total_inst = 0;

        while buf_used < buf_size {
            let length = self.decode_at(start)?;
            assert!(length >= 0);
            start += length as u64;
            buf_used += length as usize;
            total_inst += 1;
            if inst_size > 0 && total_inst >= inst_size {
                break;
            }
        }

        Ok(())
    }

    pub fn set_asm_emit(&mut self, asm_emit: Box<dyn AssemblyEmit>) {
        let asm_emit = Box::new(RustAssemblyEmit::from_internal(asm_emit));
        self.sleigh_proxy.as_mut().unwrap().set_asm_emit(asm_emit);
    }

    pub fn set_pcode_emit(&mut self, pcode_emit: Box<dyn PcodeEmit>) {
        let pcode_emit = Box::new(RustPcodeEmit::from_internal(pcode_emit));
        self.sleigh_proxy
            .as_mut()
            .unwrap()
            .set_pcode_emit(pcode_emit);
    }

    pub fn set_loader(&mut self, loader: Box<dyn LoadImage>) {
        let load_image = Box::new(RustLoadImage::from_internal(loader));
        self.sleigh_proxy.as_mut().unwrap().set_loader(load_image);
    }

    pub fn asm_emit(&self) -> &Box<dyn AssemblyEmit> {
        self.sleigh_proxy
            .as_ref()
            .unwrap()
            .get_asm_emit()
            .as_ref()
            .internal_ref()
    }

    pub fn asm_emit_mut(&mut self) -> &mut Box<dyn AssemblyEmit> {
        self.sleigh_proxy
            .as_mut()
            .unwrap()
            .get_asm_emit_mut()
            .as_mut()
            .internal_mut()
    }

    pub fn pcode_emit(&self) -> &Box<dyn PcodeEmit> {
        self.sleigh_proxy
            .as_ref()
            .unwrap()
            .get_pcode_emit()
            .as_ref()
            .internal_ref()
    }

    pub fn pcode_emit_mut(&mut self) -> &mut Box<dyn PcodeEmit> {
        self.sleigh_proxy
            .as_mut()
            .unwrap()
            .get_pcode_emit_mut()
            .as_mut()
            .internal_mut()
    }

    pub fn loader(&self) -> &Box<dyn LoadImage> {
        self.load_image()
    }

    pub fn loader_mut(&mut self) -> &mut Box<dyn LoadImage> {
        self.load_image_mut()
    }

    pub fn load_image(&self) -> &Box<dyn LoadImage> {
        self.sleigh_proxy
            .as_ref()
            .unwrap()
            .get_loader()
            .as_ref()
            .internal_ref()
    }

    pub fn load_image_mut(&mut self) -> &mut Box<dyn LoadImage> {
        self.sleigh_proxy
            .as_mut()
            .unwrap()
            .get_loader_mut()
            .as_mut()
            .internal_mut()
    }
}

pub fn arch_spec(name: &str) -> Result<&str> {
    let content = *PRESET
        .get(&name.to_lowercase().as_str())
        .ok_or(Error::ArchNotFound(name.to_string()))?;
    Ok(content)
}
