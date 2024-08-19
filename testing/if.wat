(module
  (import "core" "write" (func $write (param i32) (result i32)))
  (memory 1)
  (export "memory" (memory 0))
  (data (i32.const 16) "If\n" "Else\n")
  (func $main
	;; string if ptr
	(i32.store (i32.const 0) (i32.const 16))
	;; string if length
	(i32.store (i32.const 4) (i32.const 3))
	;; string else ptr
	(i32.store (i32.const 8) (i32.const 19))
	;; string else length
	(i32.store (i32.const 12) (i32.const 5))

	(i32.const 1) (i32.const 2) (i32.ne)
	(if
	  (then
	    (i32.const 0)
	    (call $write)
	  drop )
	  (else
	    (i32.const 8)
	    (call $write)
	    drop
	))
  )
  (start $main)
)
