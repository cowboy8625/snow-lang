#!/home/cowboy/.cargo/bin/swrt
.entry main
.data
data: .ascii "hello world!\n"
.text
main:
      load %1 10
loop:
      prts data
      inc %0
      neq %0 %1
      jeq loop
      hlt
