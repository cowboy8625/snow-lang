mod module;
mod type_section;
mod ui;
mod utils;
mod wasm_walker;

use anyhow::Result;
use module::Module;

fn main() -> Result<()> {
    let filename = "./../test.wasm";
    let bytes = std::fs::read(filename)?;
    let module = Module::new(bytes)?;

    ui::App::new(module).run()?;
    Ok(())
}
