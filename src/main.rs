use std::io::Result as IOResult;
use api_snap::main as lib_main;

#[cfg(not(tarpaulin_include))]
fn main() -> IOResult<()> {
    lib_main()
}
