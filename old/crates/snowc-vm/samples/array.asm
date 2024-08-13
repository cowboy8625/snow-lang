.entry main
; .data
; string: .ascii "EQ\n"
.text
main:
;   ; alocate 10 bytes to heap
;   load %0 10
;   aloc %0
;   ; set index 0 to 'a'
;   load %0 0
;   load %1 96
;   setm %0 %1
;   loadm %0 %2
;   eq %1 %2
;   jeq L1
;   jmp exit
; L1:
;   prts string
;   ; set index 1 to 'b'
;   load %0 1
;   load %1 97
;   setm %0 %1
;   loadm %0 %2
;   eq %1 %2
;   jeq L2
;   jmp exit
; L2:
;   prts string
;
; exit:
;   hlt
  load %0 10  ; array size
  load %3 0  ; array loc ptr
  load %1 0   ; array idx
  aloc %0

  load %2 96  ; item
  setm %1 %2  ; set item to array
  inc %1
  call print_array
  hlt

print_array:
  load %31 0
  add %3 %31 %5  ; %5 start of array
  add %0 %3 %4   ; %4 end of array
  dec %4
print_array_loop:
  loadm %5 %6
  prti %6
  inc %5
  prti %5
  eq %5 %4
  jne print_array_loop
  ret


