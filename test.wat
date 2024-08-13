(module
  (import "core" "write"
    (func $fd_write (param i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))
  (data (i32.const 8) "Hello, World!\n")
  (func $main
    (i32.store (i32.const 0) (i32.const 8)) ;; ptr to the string
    (i32.store (i32.const 4) (i32.const 14)) ;; length of the string
    (call $fd_write
      (i32.const 0) ;; ptr to the iov structure
    )
    drop ;; drop the result of the write (i32)
  )
  (start $main)
)
