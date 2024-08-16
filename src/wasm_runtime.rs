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
    linker.func_wrap(
        "core",
        "write",
        |mut ctx: Caller<'_, WasiCtx>, iovs_ptr: i32| {
            let Some(exported_memory) = ctx.get_export("memory") else {
                return 1;
            };

            let Some(memory) = exported_memory.into_memory() else {
                return 1;
            };

            let offset_ptr = iovs_ptr as usize;
            let mut ptr_str_buf = [0u8; 4];
            memory.read(&ctx, offset_ptr, &mut ptr_str_buf).unwrap();
            let mut ptr_len_buf = [0u8; 4];
            memory.read(&ctx, offset_ptr + 4, &mut ptr_len_buf).unwrap();
            let len = u32::from_le_bytes(ptr_len_buf) as usize;
            let ptr = u32::from_le_bytes(ptr_str_buf) as usize;
            let mut string = vec![0u8; len];
            memory.read(&ctx, ptr, &mut string).unwrap();

            print!("{}", String::from_utf8(string.clone()).unwrap());
            std::io::stdout().flush().unwrap();

            return 0;
        },
    )?;

    linker.instantiate(&mut store, &module)?;
    Ok(())
}
