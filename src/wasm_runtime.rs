use anyhow::Result;
use std::io::Write;
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

pub fn run(wasm_bytes: &[u8]) -> Result<()> {
    let engine = Engine::default();
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);

    let module = Module::new(&engine, wasm_bytes)?;

    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    linker.func_wrap("core", "add", |a: i32, b: i32| -> i32 { a + b })?;
    linker.func_wrap(
        "core",
        "write",
        |mut ctx: Caller<'_, WasiCtx>, ptr: i32, len: i32| {
            let Some(exported_memory) = ctx.get_export("memory") else {
                return 1;
            };

            let Some(memory) = exported_memory.into_memory() else {
                println!("Memory export not found");
                return 1;
            };

            // let offset_ptr = iovs_ptr as usize;
            // let mut ptr_str_buf = [0u8; 4];
            // memory.read(&ctx, offset_ptr, &mut ptr_str_buf).unwrap();
            // let mut ptr_len_buf = [0u8; 4];
            // memory.read(&ctx, offset_ptr + 4, &mut ptr_len_buf).unwrap();
            // let len = u32::from_le_bytes(ptr_len_buf) as usize;
            // let ptr = u32::from_le_bytes(ptr_str_buf) as usize;
            let len = len as usize;
            let ptr = ptr as usize;
            let mut string = vec![0u8; len];
            memory.read(&ctx, ptr, &mut string).unwrap();

            print!("{}", String::from_utf8(string.clone()).unwrap());
            std::io::stdout().flush().unwrap();

            return 0;
        },
    )?;

    linker.instantiate(&mut store, &module)?;
    // linker.get(&mut store, "main")?.call(&mut store, ())?;
    Ok(())
}
