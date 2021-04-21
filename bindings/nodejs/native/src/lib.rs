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

#[derive(Clone, PartialEq)]
pub struct JsAddr {
    space: String,
    offset: u64,
}
impl ToString for JsAddr {
    fn to_string(&self) -> String {
        format!("{}({})", self.space, self.offset)
    }
}
impl JsAddr {
    pub fn from_rust<'a, C: Context<'a>>(cx: &mut C, addr: &Address) -> JsResult<'a, JAddr> {

        let space = cx.string(addr.space.clone()).upcast::<JsValue>();
        let offset = cx.number(addr.offset as u32).upcast::<JsValue>();

        Ok(JAddr::new(cx, vec![space, offset])?)
    }
    pub fn from_js<'a, C: Context<'a>>(self, cx: &mut C) -> JsResult<'a, JAddr> {
        let space = cx.string(self.space.clone()).upcast::<JsValue>();
        let offset = cx.number(self.offset as u32).upcast::<JsValue>();

        Ok(JAddr::new(cx, vec![space, offset])?)
    }
}

#[derive(Clone)]
pub struct JsPcode {
    addr: JsAddr,
    opcode: String,
    vars: Vec<JsVarnodeData>,
    out_var: Option<JsVarnodeData>,
}
impl ToString for JsPcode {
    fn to_string(&self) -> String {
        let mut vars_str = String::new();
        for v in self.vars.iter() {
            vars_str.push_str(",");
            vars_str.push_str(&v.to_string());
        }
        if let Some(out_var) = &self.out_var {
            format!(
                "Pcode@{}({}, [{}], {})",
                self.addr.to_string(),
                self.opcode,
                vars_str,
                out_var.to_string()
            )
        } else {
            format!(
                "Pcode@{}({}, [{}])",
                self.addr.to_string(),
                self.opcode,
                vars_str
            )
        }
    }
}
impl JsPcode {
    pub fn from_rust<'a, C: Context<'a>>(
        cx: &mut C,
        pcode: &PcodeInstruction,
    ) -> JsResult<'a, JPcode> {
        let _addr = JsAddr::from_rust(cx, &pcode.addr)?.upcast();
        let _opcode = cx
            .string(pcode.opcode.to_string())
            .downcast::<JsValue>()
            .unwrap();

        let vars: Vec<Handle<JVarnodeData>> = pcode
            .vars
            .iter()
            .map(|v| JsVarnodeData::from_rust(cx, v).unwrap())
            .collect();
        let js_vars: Handle<JsArray> = JsArray::new(cx, vars.len() as u32);
        for (i, var) in vars.iter().enumerate() {
            let js_var = var.clone();
            js_vars.set(cx, i as u32, js_var)?;
        }

        let option_out_var = pcode
            .out_var
            .as_ref()
            .map(|v| JsVarnodeData::from_rust(cx, &v).unwrap().clone());
        let js_out_var = JsArray::new(cx, option_out_var.iter().len() as u32);
        if let Some(out_var) = &option_out_var {
            let var = out_var.clone();
            js_out_var.set(cx, 0, var)?;
        } else {
            let null = cx.null();
            js_out_var.set(cx, 0, null)?;
        }
        let js_vars = js_vars.as_value(cx);
        let _js_out_var = js_out_var.as_value(cx);

        Ok(JPcode::new(cx, vec![_addr, _opcode, js_vars, _js_out_var])?)
    }
    pub fn from_js<'a, C: Context<'a>>(self, cx: &mut C) -> JsResult<'a, JPcode> {
        let _addr = self.addr.from_js(cx)?.upcast();
        let _opcode = cx
            .string(self.opcode.to_string())
            .downcast::<JsValue>()
            .unwrap();
        let js_vars: Handle<JsArray> = JsArray::new(cx, self.vars.len() as u32);
        for (i, var) in self.vars.iter().enumerate() {
            let js_var = var.clone().from_js(cx)?;
            js_vars.set(cx, i as u32, js_var)?;
        }

        let js_out_var = JsArray::new(cx, self.out_var.iter().len() as u32);
        if let Some(out_var) = &self.out_var {
            let o_var = out_var.clone().from_js(cx)?;
            js_out_var.set(cx, 0, o_var)?;
        } else {
            let null = cx.null();
            js_out_var.set(cx, 0, null)?;
        }

        let js_vars = js_vars.as_value(cx);
        let _js_out_var = js_out_var.as_value(cx);

        Ok(JPcode::new(cx, vec![_addr, _opcode, js_vars, _js_out_var])?)
    }
}

#[derive(Clone)]
pub struct JsVarnodeData {
    space: String,
    size: u32,
    offset: usize,
}

impl ToString for JsVarnodeData {
    fn to_string(&self) -> String {
        format!("varnode@{}({}):{}", self.space, self.size, self.offset)
    }
}

impl JsVarnodeData {
    pub fn from_rust<'a, C: Context<'a>>(
        cx: &mut C,
        varnode: &PcodeVarnodeData,
    ) -> JsResult<'a, JVarnodeData> {
        let space = cx
            .string(varnode.space.to_string())
            .downcast::<JsValue>()
            .unwrap();
        let size = cx.number(varnode.size).downcast::<JsValue>().unwrap();
        let offset = cx
            .number(varnode.offset as u32)
            .downcast::<JsValue>()
            .unwrap();

        Ok(JVarnodeData::new(cx, vec![space, size, offset])?)
    }

    pub fn from_js<'a, C: Context<'a>>(self, cx: &mut C) -> JsResult<'a, JVarnodeData> {
        let space = cx
            .string(self.space.to_string())
            .downcast::<JsValue>()
            .unwrap();
        let size = cx.number(self.size).downcast::<JsValue>().unwrap();
        let offset = cx.number(self.offset as u32).downcast::<JsValue>().unwrap();

        Ok(JVarnodeData::new(cx, vec![space, size, offset])?)
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
    pub fn from_rust_no_pcodes<'a, C: Context<'a>>(
        cx: &mut C,
        inst: &Instruction,
    ) -> JsResult<'a, JInstruction> {
        let addr = JsAddr::from_rust(cx, &inst.addr)?
            .downcast::<JsValue>()
            .unwrap();
        let mnemonic = cx
            .string(inst.mnemonic.to_string())
            .downcast::<JsValue>()
            .unwrap();
        let body = cx
            .string(inst.body.to_string())
            .downcast::<JsValue>()
            .unwrap();
        let pcodes = JsArray::new(cx, 0).as_value(cx);
        Ok(JInstruction::new(cx, vec![addr, mnemonic, body, pcodes])?)
    }
}

impl ToString for JsInstruction {
    fn to_string(&self) -> String {
        let mut pcode_str = String::new();
        for pcode in self.pcodes.iter() {
            pcode_str.push_str(&pcode.to_string());
            pcode_str.push_str(", ");
        }

        format!(
            "Inst@{} {} {} pcodes=[{}]",
            self.addr.to_string(),
            self.mnemonic,
            self.body,
            pcode_str
        )
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
    pub class JAddr for JsAddr {
        init(mut cx) {
            let js_space: Handle<JsString> = cx.argument::<JsString>(0)?;
            let js_offset: Handle<JsNumber> = cx.argument::<JsNumber>(1)?;

            let space = js_space.value();
            let offset = js_offset.value() as u64;
            let js_addr = JsAddr{offset, space};
            Ok(js_addr)
        }
        method space(mut cx) {
            let this = cx.this();
            let space = {
                let guard = cx.lock();
                let addr = this.borrow(&guard);
                addr.space.clone()
            };

            Ok(cx.string(space).upcast())
        }

        method offset(mut cx) {
            let this = cx.this();
            let offset = {
                let guard = cx.lock();
                let addr = this.borrow(&guard);
                addr.offset.clone()
            };

            Ok(cx.number(offset as u32).upcast())
        }

        method toString(mut cx) {
            let this = cx.this();
            let js_addr = {
                let guard = cx.lock();
                let addr = this.borrow(&guard);
                addr.to_string()
            };

            Ok(cx.string(&js_addr).upcast())

        }

    }

    pub class JVarnodeData for JsVarnodeData {
        init(mut cx) {
            let js_space: Handle<JsString> = cx.argument::<JsString>(0)?;
            let js_size: Handle<JsNumber> = cx.argument::<JsNumber>(1)?;
            let js_offset: Handle<JsNumber> = cx.argument::<JsNumber>(2)?;

            let var =JsVarnodeData{space: js_space.value(), size: js_size.value() as u32, offset: js_offset.value() as usize};
            Ok(var)
        }

        method space(mut cx) {
            let this = cx.this();
            let space = {
                let guard = cx.lock();
                let varnode = this.borrow(&guard);
                varnode.space.clone()
            };

            Ok(cx.string(space).upcast())
        }
        method offset(mut cx) {
            let this = cx.this();
            let offset = {
                let guard = cx.lock();
                let varnode = this.borrow(&guard);
                varnode.offset.clone()
            };

            Ok(cx.number(offset as u32).upcast())
        }
        method size(mut cx) {
            let this = cx.this();
            let size = {
                let guard = cx.lock();
                let varnode = this.borrow(&guard);
                varnode.size.clone()
            };

            Ok(cx.number(size).upcast())
        }
        method toString(mut cx) {
            let this = cx.this();

            let js_varnode = {
                let guard = cx.lock();
                let varnode = this.borrow(&guard);
                varnode.to_string()
            };

            Ok(cx.string(&js_varnode).upcast())

        }

    }
    pub class JPcode for JsPcode {
        init(mut cx) {
            let js_addr = cx.argument::<JsValue>(0)?.downcast::<JAddr>().unwrap();
            let js_opcode: Handle<JsString> = cx.argument::<JsString>(1)?;
            let js_vars: Vec<Handle<JsValue>> = cx.argument::<JsArray>(2)?.to_vec(&mut cx)?;
            let _js_out_vars: Vec<Handle<JsValue>> =cx.argument::<JsArray>(3)?.to_vec(&mut cx)?.to_vec();

            let guard = cx.lock();
            let addr = js_addr.borrow(&guard).clone();
            let opcode = js_opcode.value();
            let mut vars = Vec::new();
            for varnode in js_vars {
                let js_var = varnode.downcast::<JVarnodeData>().unwrap().borrow(&guard).clone();
                vars.push(js_var)
            }

            let _out_vars = _js_out_vars[0];
            let defalut_var = PcodeVarnodeData{offset:0,size:0,space:"None".to_string()};
            let default = JsVarnodeData::from_rust(&mut cx, &defalut_var)?;
            let js_out_var = {let var= _out_vars.downcast::<JVarnodeData>().unwrap_or(default); let x = var.borrow(&cx.lock()).clone(); x};
            let mut out_var: Option<JsVarnodeData> = Option::None;
            if js_out_var.space != "None" {
                out_var = Option::from(js_out_var);
            }

            let js_pcode = JsPcode{addr,opcode,vars, out_var};
            Ok(js_pcode)
        }

        method addr(mut cx) {
            let this = cx.this();

            let js_addr= {
                let guard = cx.lock();
                let pcode = this.borrow(&guard);
                pcode.addr.clone()
            };

            Ok(js_addr.from_js(&mut cx)?.upcast())

        }
        method opcode(mut cx) {
            let this = cx.this();

            let js_opcode= {
                let guard = cx.lock();
                let pcode = this.borrow(&guard);
                pcode.opcode.clone()
            };

            Ok(cx.string(js_opcode).upcast())

        }
        method vars(mut cx) {
            let this = cx.this();

            let js_vars= {
                let guard = cx.lock();
                let pcode = this.borrow(&guard);
                pcode.vars.clone()
            };

            let vars = JsArray::new(&mut cx, js_vars.len() as u32);
            for (i,varnode) in js_vars.iter().enumerate() {
                let var = varnode.clone().from_js(&mut cx)?;
                vars.set(&mut cx, i as u32, var)?;
            }
            Ok(vars.upcast())

        }

        method toString(mut cx) {
            let this = cx.this();

            let js_instruction= {
                let guard = cx.lock();
                let pcode = this.borrow(&guard);
                pcode.to_string()
            };

            Ok(cx.string(&js_instruction).upcast())

        }

    }

    pub class JInstruction for JsInstruction {
        init(mut cx) {
            let js_addr = cx.argument::<JAddr>(0)?;
            let js_mnemonic: Handle<JsString> = cx.argument::<JsString>(1)?;
            let js_body: Handle<JsString> = cx.argument::<JsString>(2)?;
            let js_pcodes = cx.argument::<JsArray>(3)?.to_vec(&mut cx)?;

            let guard = cx.lock();
            let addr = {let addr = js_addr.borrow(&guard).clone(); addr};
            let mnemonic = js_mnemonic.value();
            let body = js_body.value();

            let mut pcodes = Vec::new();
            for js_pcode in js_pcodes {
                let pcode = js_pcode.downcast::<JPcode>().unwrap().borrow(&guard).clone();
                pcodes.push(pcode)
            }


            Ok(JsInstruction{addr,mnemonic,body,pcodes})
        }

        method toString(mut cx) {
            let this = cx.this();

            let js_instruction= {
                let guard = cx.lock();
                let instruction = this.borrow(&guard);
                instruction.to_string()
            };

            Ok(cx.string(&js_instruction).upcast())

        }
        method addr(mut cx) {
            let this = cx.this();

            let js_addr= {
                let guard = cx.lock();
                let instruction = this.borrow(&guard).clone();
                instruction.addr.clone()
            }.from_js(&mut cx)?;

            Ok(js_addr.upcast())

        }

        method mnemonic(mut cx) {
            let this = cx.this();

            let js_mnemonic= {
                let guard = cx.lock();
                let instruction = this.borrow(&guard);
                instruction.mnemonic.clone()
            };

            Ok(cx.string(&js_mnemonic).upcast())

        }

        method body(mut cx) {
            let this = cx.this();

            let js_body= {
                let guard = cx.lock();
                let instruction = this.borrow(&guard);
                instruction.body.clone()
            };

            Ok(cx.string(&js_body).upcast())

        }

        method pcodes(mut cx) {
            let this = cx.this();

            let js_pcodes= {
                let guard = cx.lock();
                let instruction = this.borrow(&guard);
                instruction.pcodes.clone()
            };
            let pcodes = JsArray::new(&mut cx, js_pcodes.len() as u32);
            for (i,pcode) in js_pcodes.iter().enumerate() {
                let pcode = pcode.clone().from_js(&mut cx)?;
                pcodes.set(&mut cx, i as u32, pcode)?;
            }
            Ok(pcodes.upcast())
        }

    }

    pub class JsSleigh for Sleigh {
        init(mut cx) {

            let js_spec: Handle<JsString> = cx.argument::<JsString>(0)?;
            let js_code: Handle<JsArray> = cx.argument::<JsArray>(1)?;
            let sleigh = Sleigh::from_rust(cx, js_spec, js_code)?;
            Ok(sleigh)
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

                let js_pcode = JsPcode::from_rust(&mut cx, &pcode_asm)?;
                pcodes.push(js_pcode)
            }

            let mut handle_insts = Vec::new();

            for asm in asm_emit.asms {
                let js_instruction = JsInstruction::from_rust_no_pcodes(&mut cx,&asm)?;
                handle_insts.push(js_instruction)
            }
            let guard = cx.lock();
            for inst in handle_insts.iter_mut() {
                for pcode in pcodes.iter() {
                    let js_pcode = pcode.borrow(&guard);
                    let mut rust_inst = inst.borrow_mut(&guard);
                    if js_pcode.addr == rust_inst.addr {
                        rust_inst.pcodes.push(js_pcode.clone())
                    }
                }
            }

            for (index, js_inst) in handle_insts.iter().enumerate() {
                let inst = js_inst.clone();
                js_insts.set(&mut cx, index as u32, inst)?;
            }

            Ok(js_insts.as_value(&mut cx))
        }
    }
}
register_module!(mut cx, {
    cx.export_class::<JAddr>("Address")?;
    cx.export_class::<JPcode>("Pcode")?;
    cx.export_class::<JVarnodeData>("PcodeVarnodeData")?;
    cx.export_class::<JInstruction>("Instruction")?;
    cx.export_class::<JsSleigh>("Sleigh")
});
