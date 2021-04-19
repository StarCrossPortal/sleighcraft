mod funcs;
mod util;

use std::os::raw::{c_char, c_int};

use rusqlite::{
    ffi,
};
use rusqlite::{to_sqlite_error, Connection, Result};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn sqlite3_extension_init(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    // SQLITE_EXTENSION_INIT2 equivalent
    unsafe {
        ffi::sqlite3_api = p_api;
    }
    let res = querycraft_init(db);
    if let Err(err) = res {
        return unsafe { to_sqlite_error(&err, pz_err_msg) };
    }

    ffi::SQLITE_OK
}

fn querycraft_init(db: *mut ffi::sqlite3) -> Result<()> {
    let conn = unsafe { Connection::from_handle(db)? };
    funcs::load::load_func_init(&conn)?;
    funcs::disasm::disasm_func_init(&conn)?;
    Ok(())
}