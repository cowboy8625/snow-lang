.entry main
.data
string: .ascii "Calling a function\n"
.text
main:
  load %31 123
  load %30 321
  prti %31
  call func_add
  prti %31
  hlt

func_add:
  prts string
  add %31 %30 %31
  ret
