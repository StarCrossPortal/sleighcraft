//! Function that loads binary into table, currently only ELF supported.
use std::convert::TryInto;
use std::fmt::{self, Display};
use std::num::TryFromIntError;

use goblin::{elf, Object};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rusqlite::{
    functions::{ConnectionRef, Context, FunctionFlags},
    params,
    types::{ToSqlOutput, Value},
    Connection, Error, Result,
};

#[derive(Debug)]
enum LoadError {
    InvalidTableName,
    IoError(std::io::Error),
    BinFormatError(goblin::error::Error),
    InvalidSegmentInfo(String),
    UnsupportedFormat,
    InvalidNumber(TryFromIntError),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTableName => {
                write!(f, "invalid output table name")
            }
            Self::IoError(e) => {
                write!(f, "io error {}", e)
            }
            Self::BinFormatError(e) => {
                write!(f, "binary format error {}", e)
            }
            Self::InvalidSegmentInfo(s) => {
                write!(f, "invalid segment {}", s)
            }
            Self::UnsupportedFormat => {
                write!(f, "unsupported format")
            }
            Self::InvalidNumber(e) => {
                write!(f, "invalid number {}", e)
            }
        }
    }
}
impl LoadError {
    fn invalid_seg(s: String) -> Self {
        Self::InvalidSegmentInfo(s)
    }
}

impl From<TryFromIntError> for LoadError {
    fn from(e: TryFromIntError) -> Self {
        Self::InvalidNumber(e)
    }
}
impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
impl From<goblin::error::Error> for LoadError {
    fn from(e: goblin::error::Error) -> Self {
        Self::BinFormatError(e)
    }
}
impl From<LoadError> for Error {
    fn from(e: LoadError) -> Self {
        Self::UserFunctionError(Box::new(e))
    }
}

impl std::error::Error for LoadError {}

fn insert_elf_data(db: ConnectionRef, bin: &[u8], elf: &elf::Elf, table_name: &str) -> Result<()> {
    // TODO: use real segname
    let mut idx = 0;
    for ph in elf.program_headers.iter() {
        if ph.p_type == elf::program_header::PT_LOAD {
            let size: usize = ph.p_filesz.try_into().map_err(LoadError::from)?;
            let vaddr = ph.p_vaddr;
            let offset: usize = ph.p_offset.try_into().map_err(LoadError::from)?;

            if offset + size >= bin.len() {
                return Err(LoadError::invalid_seg(format!(
                    "size {} too big",
                    ph.p_filesz
                )))?;
            }

            let bytes = &bin[offset..offset + size];

            let sql = format!(
                "INSERT INTO {}(ADDR, NAME, BYTES) VALUES (?, ?, ?);",
                table_name
            );
            let seg_name = format!("segment_{}", idx);
            idx += 1;

            db.execute(&sql, params![vaddr, seg_name, bytes])?;
        }
    }

    Ok(())
}

fn insert_segments(db: ConnectionRef, path: &str, table_name: &str) -> Result<()> {
    let bin_bytes = std::fs::read(path).map_err(LoadError::from)?;
    match Object::parse(&bin_bytes).map_err(LoadError::from)? {
        Object::Elf(elf) => insert_elf_data(db, &bin_bytes, &elf, table_name),
        _ => Err(Error::UserFunctionError(Box::new(
            LoadError::UnsupportedFormat,
        ))),
    }
}

fn do_load<'a>(conn: ConnectionRef, ctx: &Context) -> Result<ToSqlOutput<'a>> {
    let bin_path: String = ctx.get(0)?;
    let rand_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let out_table_name: String = if ctx.len() > 1 {
        ctx.get(1)?
    } else {
        format!("qc_load_res_{}", rand_id)
    };
    if !out_table_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(Error::UserFunctionError(Box::new(
            LoadError::InvalidTableName,
        )));
    }

    let sql = format!(
        r#"CREATE TABLE {}(
            addr STRING,
            name STRING,
            bytes BLOB
        );
        "#,
        out_table_name
    );

    conn.execute_batch(&sql)?;
    insert_segments(conn, &bin_path, &out_table_name)?;

    // TODO: load the binary by inserting..

    Ok(ToSqlOutput::Owned(Value::Text(out_table_name)))
}

pub(crate) fn load_func_init(db: &Connection) -> Result<()> {
    db.create_scalar_function("qc_load", 2, FunctionFlags::SQLITE_DETERMINISTIC, |ctx| {
        let conn = unsafe { ctx.get_connection()? };
        do_load(conn, ctx)
    })?;

    db.create_scalar_function("qc_load", 1, FunctionFlags::SQLITE_DETERMINISTIC, |ctx| {
        let conn = unsafe { ctx.get_connection()? };
        do_load(conn, ctx)
    })?;

    Ok(())
}
