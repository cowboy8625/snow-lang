#!/home/cowboy/.cargo/bin/swrt
.entry main
.data
fiz: .ascii "fiz"
buz: .ascii "buz"
newline: .ascii "\n"
.text
print_fiz:
  prts fiz
  prts newline
  jmp loop
print_buz:
  prts buz
  prts newline
  jmp loop
print_fizbuz:
  prts fiz
  prts buz
  prts newline
  jmp loop
main:
;; reg 0 is count
;; reg 1 is max count
  load %1 30
;; reg 2 mod value
  load %3 3
  load %5 5
  jmp loop_start
loop:
  inc %0
loop_start:
;; check if fizbuz
  mod %0 %3 %2
  mod %0 %5 %4
  eq %2 %6
  jne check_if_fiz
  eq %4 %6
  jeq print_fizbuz
check_if_fiz:
  mod %0 %3 %2
  eq %2 %6
  jeq print_fiz
;; check if buz
  mod %0 %5 %2
  eq %2 %6
  jeq print_buz
;; loop condition
  geq %1 %0
  prti %0
  jeq loop
  hlt


