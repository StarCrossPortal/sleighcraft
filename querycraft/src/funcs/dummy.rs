use rusqlite::{
    Connection,
    Result,
    functions::FunctionFlags,
    types::{ToSqlOutput, Value},
};

pub(crate) fn dummy_func_init(db: &Connection) -> Result<()> {
    db.create_scalar_function(
        "dummy_test_function",
        0,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |_ctx| {
            Ok(ToSqlOutput::Owned(Value::Text(
                "dummy_test_function!".to_string()
            )))
        }
    )
}