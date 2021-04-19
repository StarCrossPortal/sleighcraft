//! qc_disasm: disassembly the bytes into a table describing with pcodes and asms.
//! - arg0: disasm type, "bytes" to disasm bytes directly, "loaded" to disasm on loaded res table
//! - arg1: if "bytes", the bytes to disasm. if "loaded", the table name to disasm
//! - arg2: arch name
//! - arg3: output asm table name
//! - arg4: output pcode table name
use std::fmt::{self, Display};

use rusqlite::{Connection, Error, Result, functions::{ConnectionRef, Context, FunctionFlags}, types::{ToSqlOutput, Value}, params};
use crate::util::valid_table_name;
use sleighcraft::prelude::*;


#[derive(Debug)]
enum DisasmError {
    UnknownDisasmType(String),
    InvalidTableName(String),
    DecodeError(sleighcraft::error::Error),
}

impl Display for DisasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisasmError::UnknownDisasmType(s) => {
                write!(f, "unknown disasm type {}", s)
            }
            DisasmError::InvalidTableName(s) => {
                write!(f, "invalid table name {}", s)
            },
            DisasmError::DecodeError(e) => {
                write!(f, "sleigh decode error {}", e)
            }
        }
    }
}

impl std::error::Error for DisasmError {}
impl From<sleighcraft::error::Error> for DisasmError {
    fn from(e: sleighcraft::error::Error) -> Self {
        Self::DecodeError(e)
    }
}
impl From<DisasmError> for Error {
    fn from(e: DisasmError) -> Self {
        Error::UserFunctionError(Box::new(e))
    }
}

fn prepare_output_tables(db: &ConnectionRef, asm_table: &str, pcode_table: &str) -> Result<()> {
    // TODO: support more vars in pcodes
    let sql = format!(r#"
        CREATE TABLE {}(
            space STRING,
            offset STRING,
            mnemonic STRING,
            body STRING
        );
        CREATE TABLE {}(
            space STRING,
            offset STRING,
            op STRING,
            opr1_space STRING,
            opr1_offset STRING,
            opr1_size INT,
            opr2_space STRING,
            opr2_offset STRING,
            opr2_size INT,
            out_space STRING,
            out_offset STRING,
            out_size INT,
            comment STRING
        );
    "#, asm_table, pcode_table);

    db.execute_batch(&sql)
}

fn do_disasm_bytes<'a>(db: ConnectionRef, ctx: &Context) -> Result<ToSqlOutput<'a>> {
    let buf: Vec<u8> = ctx.get(1)?;
    let arch_name: String = ctx.get(2)?;
    let output_asm_name: String = ctx.get(3)?;
    let output_pcode_name: String = ctx.get(4)?;

    if !valid_table_name(&output_asm_name) {
        return Err(DisasmError::InvalidTableName(output_asm_name).into());
    }
    
    if !valid_table_name(&output_pcode_name) {
        return Err(DisasmError::InvalidTableName(output_pcode_name).into());
    }


    prepare_output_tables(&db, &output_asm_name, &output_pcode_name)?;

    let mut sleigh_builder = SleighBuilder::default();
    let spec = arch(&arch_name).unwrap();
    let mut loader = PlainLoadImage::from_buf(&buf, 0);
    sleigh_builder.loader(&mut loader);
    sleigh_builder.spec(spec);
    let mut asm_emit = CollectingAssemblyEmit::default();
    let mut pcode_emit = CollectingPcodeEmit::default();
    sleigh_builder.asm_emit(&mut asm_emit);
    sleigh_builder.pcode_emit(&mut pcode_emit);
    let mut sleigh = sleigh_builder.try_build().map_err(DisasmError::from)?;

    sleigh.decode(0).map_err(DisasmError::from)?;
    
    let sql = format!("INSERT INTO {} values (?, ?, ?, ?)", output_asm_name);

    for asm in asm_emit.asms.iter() {
        let space = asm.addr.space.to_string();
        let offset = asm.addr.offset;
        let mnemonic = asm.mnemonic.to_string();
        let body = asm.body.to_string();

        db.execute(&sql, params![space, offset, mnemonic, body])?;
    }

    for pcode_ins in pcode_emit.pcode_asms.iter() {
        let space = pcode_ins.addr.space.to_string();
        let offset = pcode_ins.addr.offset;
        let op = pcode_ins.opcode.to_string();
        let vars = &pcode_ins.vars;
        let (var1_space, var1_off, var1_size) = if vars.len() >= 1 {
            (Some(vars[0].space.to_string()), Some(vars[0].offset), Some(vars[0].size))
        } else {
            (None, None, None)
        };
        let (var2_space, var2_off, var2_size) = if vars.len() >= 2 {
            (Some(vars[1].space.to_string()), Some(vars[1].offset), Some(vars[1].size))
        } else {
            (None, None, None)
        };

        // temp solution so that we don't lose the information of vars
        let comments = if vars.len() >= 3 {
            Some(format!("rest_vars: {:?}", &vars[3..]))
        } else {
            None
        };

        let (out_space, out_off, out_size) = match &pcode_ins.out_var {
            Some(var) => (Some(var.space.to_string()), Some(var.offset), Some(var.size)),
            None => (None, None, None)
        };

        let sql = format!("INSERT INTO {} values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", output_pcode_name);
        db.execute(&sql, params![
            space,
            offset,
            op,
            var1_space,
            var1_off,
            var1_size,
            var2_space,
            var2_off,
            var2_size,
            out_space,
            out_off,
            out_size,
            comments
        ])?;
    }

    Ok(ToSqlOutput::Owned(1.into()))
}

fn do_disasm_loaded<'a>(db: ConnectionRef, ctx: &Context) -> Result<ToSqlOutput<'a>> {
    unimplemented!("do_disasm_loaded")
}

fn do_disasm<'a>(db: ConnectionRef, ctx: &Context) -> Result<ToSqlOutput<'a>> {
    let disasm_type: String = ctx.get(0)?;
    match disasm_type.as_str() {
        "bytes" => do_disasm_bytes(db, ctx),
        "loaded" => do_disasm_loaded(db, ctx),
        _ => Err(DisasmError::UnknownDisasmType(disasm_type).into())
    }
}

pub(crate) fn disasm_func_init(db: &Connection) -> Result<()> {
    db.create_scalar_function(
        "qc_disasm",
        5,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let conn = unsafe { ctx.get_connection()? };
            do_disasm(conn, ctx)
        }
    )
}