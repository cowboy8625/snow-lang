// prettier-ignore
const code = [
  // Magic number
  0x00, 0x61, 0x73, 0x6d,// 0asm
  // Version
  0x01, 0x00, 0x00, 0x00,
  // type section
  1, 5, 1, 0x60, 0, 1, 0x7f,
  // function section
  3, 2, 1, 0,
  // export section
  7, 7, 1, 3, 97, 98, 99, 0, 0,
  // code
  10, 7 + 2, 1, 7, 0, 0x41, 5, 0x41, 5, 0x6C, 0x0B,
];

// // prettier-ignore
// const code = [
//   0x00, 0x61, 0x73, 0x6d, // 00 | WASM magic number (0x00, 0x61, 0x73, 0x6d) - identifies the file as a WASM module
//   0x01, 0x00, 0x00, 0x00, // 04 | WASM version number (1.0)
//   0x01, 0x09, 0x02, 0x60, // 08 | Type section: 1 entry, type index 0x60
//   0x01, 0x7f, 0x01, 0x7f, // 0c | Function type: (param i32) -> (result i32)
//   0x60, 0x00, 0x00, 0x02, // 10 | Function type: no params, returns (result i32)
//   0x0e, 0x01, 0x04, 0x63, // 14 | Import section: 1 import, import index 0x63
//   0x6f, 0x72, 0x65, 0x05, // 18 | Import name: "core"
//   0x77, 0x72, 0x69, 0x74, // 1c | Import function name: "write"
//   0x65, 0x00, 0x00, 0x03, // 20 | Import: function index 0, no local variables
//   0x02, 0x01, 0x01, 0x05, // 24 | Function section: 1 function, function index 5
//   0x03, 0x01, 0x00, 0x01, // 28 | Memory section: 1 memory, 1 page
//   0x07, 0x0a, 0x01, 0x06, // 2c | Export section: 1 export, export index 6
//   0x6d, 0x65, 0x6d, 0x6f, // 30 | Export name: "memory"
//   0x72, 0x79, 0x02, 0x00, // 34 | Exported memory index 0
//   0x08, 0x01, 0x01, 0x0a, // 38 | Start section: 1 function, start function index 10
//   0x17, 0x01, 0x15, 0x00, // 3c | Code section: 1 function, length 21 bytes
//   0x41, 0x00, 0x41, 0x08, // 40 | i32.const 0 (push 0), i32.const 8 (push 8)
//   0x36, 0x02, 0x00, 0x41, // 44 | i32.store offset=2 (store 8 at memory[0])
//   0x04, 0x41, 0x0e, 0x36, // 48 | i32.const 4, i32.const 14, i32.store offset=2 (store 14 at memory[4])
//   0x02, 0x00, 0x41, 0x00, // 4c | i32.const 1 (stdout), i32.const 0 (iov ptr)
//   0x10, 0x00, 0x1a, 0x0b, // 50 | i32.const 20 (bytes written), call core.write (function index 0)
//   0x0b, 0x14, 0x01, 0x00, // 54 | drop (result), end function body
//   0x41, 0x08, 0x0b, 0x0e, // 58 | Data section: 1 data segment, length 14 bytes
//   0x48, 0x65, 0x6c, 0x6c, // 5c | "Hello"
//   0x6f, 0x2c, 0x20, 0x57, // 60 | ", World"
//   0x6f, 0x72, 0x6c, 0x64, // 64 | "orld"
//   0x21, 0x0a,             // 68 | "!\n"
// ]

const core = {
  core: {
    write: (ptr) => {
      console.log(ptr);
    },
  },
};

const arr = Uint8Array.from(code);
const mod = await WebAssembly.compile(arr);
const i = await WebAssembly.instantiate(mod, core);
console.log(i.add(1, 2));
