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
