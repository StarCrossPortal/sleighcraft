//! Function that loads binary into table, currently only ELF supported.
use std::fmt::{self, Display};

use rand::distributions::Alphanumeric;
use rusqlite::{
    Connection,
    Result,
    functions::{FunctionFlags},
    types::{ToSqlOutput, Value},
    Error
};
use rand::{thread_rng, Rng};
use anyhow::anyhow;

#[derive(Debug)]
enum LoadError {
    InvalidTableName
}

impl Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTableName => {
                write!(f, "invalid output table name")
            }
        }
    }
}

impl std::error::Error for LoadError {}

pub(crate) fn load_func_init(db: &Connection) -> Result<()> {

    db.create_scalar_function(
        "qc_load",
        2, 
        FunctionFlags::SQLITE_DETERMINISTIC,
    |ctx| {
        let conn = unsafe {
            ctx.get_connection()?
        };

        let bin_path: String = ctx.get(0)?;
        let rand_id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let out_table_name = ctx.get(1).unwrap_or(format!("qc_loaded_{}", rand_id));
        if !out_table_name.chars().all(char::is_alphabetic) {
            return Err(Error::UserFunctionError(Box::new(LoadError::InvalidTableName)));
        }

        let sql = format!(r#"CREATE TABLE {}(
            name STRING,
            bytes BLOB,
        );
        "#, out_table_name);

        conn.execute_batch(&sql)?;

        // TODO: load the binary by inserting..

        Ok(ToSqlOutput::Owned(Value::Text(out_table_name)))
    })
}