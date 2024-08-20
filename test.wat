(module
  ;; Import an external function to handle writing to the console.
  ;; Assume the imported function is `write` which takes two parameters:
  ;;   1. An address in memory (i32)
  ;;   2. The length of the string (i32)
  (import "core" "write" (func $write (param i32 i32) (result i32)))

  (memory 1)
  (export "memory" (memory 0))
  ;; Define the string "Hello World\n" in memory.
  (data (i32.const 0) "Hello World\n")

  ;; Define the main function
  (func $main
    ;; Write the string by passing the address and length to the imported `write` function
    (call $write
      (i32.const 0)    ;; Address of the string in memory
      (i32.const 12)   ;; Length of the string "Hello World\n"
    )
    (drop)
  )

  (start $main)
)
