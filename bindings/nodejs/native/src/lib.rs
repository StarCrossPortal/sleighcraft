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
use neon::{prelude::*, result::Throw};
use sleighcraft::{
    arch, CollectingAssemblyEmit, CollectingPcodeEmit, PlainLoadImage, SleighBuilder,
};
use sleighcraft::{Address, Instruction, PcodeInstruction, PcodeVarnodeData};

#[derive(Clone, PartialEq, Eq)]
pub struct JsAddr {
    space: String,
    offset: u64,
}

impl JsAddr {
    pub fn from_rust(addr: &Address) -> Result<JsAddr, Throw> {
        Ok(Self {
            space: addr.space.to_string(),
            offset: addr.offset,
        })
    }

    pub fn addr_to_object<'a, C: Context<'a>>(
        cx: &mut C,
        rust_addr: &JsAddr,
    ) -> JsResult<'a, JsObject> {
        let js_addr = JsObject::new(cx);

        let js_space = cx.string(&rust_addr.space);
        let js_offset = cx.number(rust_addr.offset as f64);

        js_addr.set(cx, "offset", js_offset)?;
        js_addr.set(cx, "space", js_space)?;

        Ok(js_addr)
    }

}

#[derive(Clone)]
pub struct JsPcode {
    addr: JsAddr,
    opcode: String,
    vars: Vec<JsVarnodeData>,
    out_var: Option<JsVarnodeData>,
}

impl JsPcode {
    pub fn from_rust(pcode: &PcodeInstruction) -> Result<JsPcode, Throw> {
        let addr = JsAddr::from_rust(&pcode.addr)?;
        let opcode = pcode.opcode.to_string();
        let vars = pcode
            .vars
            .iter()
            .map(|v| JsVarnodeData::from_rust(v).unwrap())
            .collect();
        let out_var = pcode.out_var.as_ref().map(|v| JsVarnodeData::from_rust(&v).unwrap());

        Ok(Self {
            addr,
            opcode,
            vars,
            out_var,
        })
    }
    pub fn pcode_to_object<'a, C: Context<'a>>(cx: &mut C, pcode: &JsPcode) -> JsResult<'a, JsObject> {
        let js_pcode = JsObject::new(cx);

        let js_addr = JsAddr::addr_to_object(cx, &pcode.addr)?;
        let js_opcode = cx.string(&pcode.opcode);

        let js_vars = JsArray::new(cx, pcode.vars.len() as u32);
        for (index,obj) in pcode.vars.iter().enumerate(){
            let varnode = JsVarnodeData::varnode_data_to_object(cx, obj)?;
            js_vars.set(cx,index as u32, varnode)?;
        }

        js_pcode.set(cx, "addr", js_addr)?;
        js_pcode.set(cx, "opcode", js_opcode)?;
        js_pcode.set(cx, "vars", js_vars)?;
        if let Some(out_var) = &pcode.out_var { 
            let js_out_var = JsVarnodeData::varnode_data_to_object(cx, out_var)?;
            js_pcode.set(cx, "out_var", js_out_var)?;
        }
        
        Ok(js_pcode)
    }
}

#[derive(Clone)]
pub struct JsVarnodeData {
    space: String,
    size: u32,
    offset: usize,
}

impl JsVarnodeData {
    pub fn from_rust(varnode: &PcodeVarnodeData) -> Result<JsVarnodeData, Throw> {
        Ok(Self {
            space: varnode.space.to_string(),
            size: varnode.size,
            offset: varnode.offset,
        })
    }
    pub fn varnode_data_to_object<'a, C: Context<'a>>(cx: &mut C, varnode: &JsVarnodeData) -> JsResult<'a, JsObject> {
        let js_varnode_data = JsObject::new(cx);

        let js_space = cx.string(&varnode.space);
        let js_size = cx.number(varnode.size as f64);
        let js_offset = cx.number(varnode.offset as f64);

        js_varnode_data.set(cx, "space", js_space)?;
        js_varnode_data.set(cx, "size", js_size)?;
        js_varnode_data.set(cx, "offset", js_offset)?;

        Ok(js_varnode_data)
    }
}

#[derive(Clone)]
pub struct JsInstruction {
    addr: JsAddr,
    mnemonic: String,
    body: String,
    pcodes: Vec<JsPcode>,
}

impl JsInstruction {
    pub fn from_rust_no_pcodes(inst: &Instruction) -> Result<JsInstruction, Throw> {
        let addr = JsAddr::from_rust(&inst.addr)?;
        let mnemonic = inst.mnemonic.to_string();
        let body = inst.body.to_string();
        let pcodes = vec![];

        Ok(JsInstruction {
            addr,
            mnemonic,
            body,
            pcodes,
        })
    }

    pub fn instruction_to_object<'a, C: Context<'a>>(
        cx: &mut C,
        inst: &JsInstruction,
    ) -> JsResult<'a, JsObject> {
        let js_instruction = JsObject::new(cx);

        let js_addr = JsAddr::addr_to_object(cx, &inst.addr)?;
        let js_mnemonic = cx.string(&inst.mnemonic);
        let js_body = cx.string(&inst.body);

        let js_pcodes = JsArray::new(cx, inst.pcodes.len() as u32);
        for (index, pcode) in inst.pcodes.iter().enumerate() {
            let js_pcode = JsPcode::pcode_to_object(cx, pcode)?;
            js_pcodes.set(cx, index as u32, js_pcode)?;
        }
        js_instruction.set(cx, "addr", js_addr)?;
        js_instruction.set(cx, "mnemonic", js_mnemonic)?;
        js_instruction.set(cx, "body", js_body)?;
        js_instruction.set(cx, "pcodes", js_pcodes)?;
        Ok(js_instruction)
    }
}
pub struct Sleigh {
    spec: Option<String>,
    code: Option<Vec<u8>>,
}

impl Sleigh {
    pub fn from_rust(
        mut cx: CallContext<JsUndefined>,
        spec: Handle<JsString>,
        js_code: Handle<JsArray>,
    ) -> Result<Sleigh, Throw> {
        let mut rust_code: Vec<u8> = Vec::new();
        let handle_code: Vec<Handle<JsValue>> = js_code.to_vec(&mut cx)?;

        for _code in handle_code {
            let code = _code.downcast::<JsNumber>().or_throw(&mut cx)?.value();
            rust_code.push(code as u8)
        }

        let spec = Option::from(arch(spec.value().as_str()).unwrap().to_string());
        let code = Option::from(rust_code);
        
        Ok(Self { spec, code })
    }
}

declare_types! {
    pub class JsSleigh for Sleigh {
        init(mut cx) {

            let js_spec: Handle<JsString> = cx.argument::<JsString>(0)?;
            let js_code: Handle<JsArray> = cx.argument::<JsArray>(1)?;

            let sleigh = Sleigh::from_rust(cx, js_spec, js_code)?;
            Ok(sleigh)
        }

        method spec(mut cx) {
            let this = cx.this();
            let spec = {
                let guard = cx.lock();
                let sleigh = this.borrow(&guard);
                sleigh.spec.clone()
            }.unwrap();

            Ok(cx.string(&spec).upcast())
        }

        method code(mut cx) {
            let this = cx.this();
            let code = {
                let guard = cx.lock();
                let sleigh = this.borrow(&guard);
                sleigh.code.clone()
            }.unwrap();

            let js_code = JsArray::new(&mut cx,code.len() as u32);
            for (index, obj) in code.iter().enumerate() {
                let js_obj  = cx.number(*obj as f64);
                js_code.set(&mut cx, index as u32, js_obj)?;
            }

            Ok(js_code.as_value(&mut cx))
        }

        method disasm(mut cx) {
            let _start = match cx.argument_opt(0) {
                Some(arg) => arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64,
                None => 0 as u64,
            };

            let mut sleigh_builder = SleighBuilder::default();

            let this = cx.this();
            let spec = {
                let guard = cx.lock();
                let sleigh = this.borrow(&guard);
                sleigh.spec.clone()
            }.unwrap();
            let code = {
                let guard = cx.lock();
                let sleigh = this.borrow(&guard);
                sleigh.code.clone()
            }.unwrap();

            let mut loader = PlainLoadImage::from_buf(code.as_ref(), _start);
            sleigh_builder.loader(&mut loader);
            sleigh_builder.spec(spec.as_ref());

            let mut asm_emit = CollectingAssemblyEmit::default();
            let mut pcode_emit = CollectingPcodeEmit::default();

            sleigh_builder.asm_emit(&mut asm_emit);
            sleigh_builder.pcode_emit(&mut pcode_emit);

            let mut sleigh = sleigh_builder.try_build().unwrap();

            match sleigh.decode(_start) {
                Ok(v) => v,
                Err(e) => std::panic::panic_any(e)
            }
            let js_insts = JsArray::new(&mut cx, asm_emit.asms.len() as u32);

            let mut pcodes = Vec::new();
            for pcode_asm in pcode_emit.pcode_asms {
                pcodes.push(JsPcode::from_rust(&pcode_asm)?)
            }

            let mut rust_insts: Vec<JsInstruction> = Vec::new();
            for asm in asm_emit.asms {
                let js_instruction = JsInstruction::from_rust_no_pcodes(&asm)?;
                rust_insts.push(js_instruction);
            }

            for rust_inst in rust_insts.iter_mut() {
                for pcode in pcodes.iter() {
                    if pcode.addr == rust_inst.addr {
                        &rust_inst.pcodes.push(pcode.clone());
                    }
                }
            }
            
            for (index, js_inst) in rust_insts.iter().enumerate() {
                let js_inst_object = JsInstruction::instruction_to_object(&mut cx, &js_inst)?;
                js_insts.set(&mut cx, index as u32, js_inst_object)?;
            }
            
            Ok(js_insts.as_value(&mut cx))
        }
    }
}
register_module!(mut cx, { cx.export_class::<JsSleigh>("Sleigh") });
