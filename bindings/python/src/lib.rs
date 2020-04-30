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
use pyo3::class::basic::PyObjectProtocol;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyLong, PyUnicode};
use sleighcraft::error::Error;
use sleighcraft::{
    arch, CollectingAssemblyEmit, CollectingPcodeEmit, PlainLoadImage, SleighBuilder,
};
use sleighcraft::{Address, Instruction, PcodeInstruction, PcodeVarnodeData};

#[pyclass]
#[derive(Clone, PartialEq, Eq)]
pub struct PyAddr {
    space: String,
    offset: u64,
}

impl ToString for PyAddr {
    fn to_string(&self) -> String {
        format!("{}({})", self.space, self.offset)
    }
}

#[pyproto]
impl PyObjectProtocol for PyAddr {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

impl PyAddr {
    pub fn from_rust(addr: &Address) -> Self {
        Self {
            space: addr.space.to_string(),
            offset: addr.offset,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyPcode {
    addr: PyAddr,
    opcode: String,
    vars: Vec<PyVarnodeData>,
    out_var: Option<PyVarnodeData>,
}

impl ToString for PyPcode {
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

#[pyproto]
impl PyObjectProtocol for PyPcode {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

#[pymethods]
impl PyPcode {
    pub fn addr(&self) -> PyResult<PyAddr> {
        Ok(self.addr.clone())
    }

    pub fn opcode(&self) -> PyResult<String> {
        Ok(self.opcode.clone())
    }

    pub fn vars(&self) -> PyResult<Vec<PyVarnodeData>> {
        Ok(self.vars.clone())
    }

    pub fn out_var(&self) -> PyResult<Option<PyVarnodeData>> {
        Ok(self.out_var.clone())
    }
}

impl PyPcode {
    fn from_rust(pcode: &PcodeInstruction) -> Self {
        let addr = PyAddr::from_rust(&pcode.addr);
        let opcode = pcode.opcode.to_string();
        let vars = pcode
            .vars
            .iter()
            .map(|v| PyVarnodeData::from_rust(v))
            .collect();
        let out_var = pcode.out_var.as_ref().map(|v| PyVarnodeData::from_rust(&v));

        Self {
            addr,
            opcode,
            vars,
            out_var,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyVarnodeData {
    space: String,
    size: u32,
    offset: usize,
}

impl ToString for PyVarnodeData {
    fn to_string(&self) -> String {
        format!("varnode@{}({}):{}", self.space, self.size, self.offset)
    }
}

#[pyproto]
impl PyObjectProtocol for PyVarnodeData {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

impl PyVarnodeData {
    pub fn from_rust(varnode: &PcodeVarnodeData) -> Self {
        Self {
            space: varnode.space.to_string(),
            size: varnode.size,
            offset: varnode.offset,
        }
    }
}

#[pyclass]
pub struct PyInstruction {
    addr: PyAddr,
    mnemonic: String,
    body: String,
    pcodes: Vec<PyPcode>,
}

impl ToString for PyInstruction {
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

#[pyproto]
impl PyObjectProtocol for PyInstruction {
    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

#[pymethods]
impl PyInstruction {
    pub fn addr(&mut self) -> PyResult<PyAddr> {
        Ok(self.addr.clone())
    }

    pub fn mnemonic(&mut self) -> PyResult<String> {
        Ok(self.mnemonic.clone())
    }

    pub fn body(&mut self) -> PyResult<String> {
        Ok(self.body.clone())
    }

    pub fn pcodes(&mut self) -> PyResult<Vec<PyPcode>> {
        Ok(self.pcodes.clone())
    }
}

impl PyInstruction {
    fn from_rust_no_pcodes(inst: &Instruction) -> Self {
        let addr = PyAddr::from_rust(&inst.addr);
        let mnemonic = inst.mnemonic.to_string();
        let body = inst.body.to_string();
        let pcodes = vec![];

        Self {
            addr,
            mnemonic,
            body,
            pcodes,
        }
    }
}

// This class use init Sleigh
#[pyclass]
pub struct Sleigh {
    spec: Option<String>,
    code: Option<Vec<u8>>,
}

#[pymethods]
impl Sleigh {
    #[new]
    pub fn new(spec: &PyUnicode, code: &PyList) -> Self {
        let sp: &str = spec.extract().unwrap();
        let spec = arch(sp);
        let codes: Vec<u8> = code.extract().unwrap();
        let spec = Option::from(spec.unwrap().to_string());
        let code = Option::from(codes);
        Sleigh { spec, code }
    }

    #[text_signature = "($self, start, cnt)"]
    pub fn disasm(&mut self, start: &PyLong) -> Result<Vec<Py<PyInstruction>>, Error> {
        let start: u64 = start.extract().unwrap();

        let mut sleigh_builder = SleighBuilder::default();
        let mut loader = PlainLoadImage::from_buf(self.code.as_ref().unwrap().as_ref(), 0);
        sleigh_builder.loader(&mut loader);
        sleigh_builder.spec(self.spec.as_ref().unwrap().as_str());

        let mut asm_emit = CollectingAssemblyEmit::default();
        let mut pcode_emit = CollectingPcodeEmit::default();

        sleigh_builder.asm_emit(&mut asm_emit);
        sleigh_builder.pcode_emit(&mut pcode_emit);

        let mut sleigh = sleigh_builder.try_build().unwrap();
        sleigh.decode(start)?;
        let mut pcodes = Vec::new();
        for pcode_asm in pcode_emit.pcode_asms {
            pcodes.push(PyPcode::from_rust(&pcode_asm));
        }

        let mut py_insts = Vec::new();
        // pcodes and insts are collected seperately, we need to manually associate them
        for asm in asm_emit.asms {
            py_insts.push(PyInstruction::from_rust_no_pcodes(&asm));
        }

        // now instructions have no pcodes associated, associate them!
        for py_inst in py_insts.iter_mut() {
            for pcode in pcodes.iter() {
                if pcode.addr == py_inst.addr {
                    py_inst.pcodes.push(pcode.clone());
                }
            }
        }

        let gil = Python::acquire_gil();
        let py = gil.python();

        let mut insts = vec![];
        for inst in py_insts.into_iter() {
            insts.push(Py::new(py, inst)?);
        }

        Ok(insts)
    }
}

#[pymodule]
fn bincraft(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Sleigh>()
}
