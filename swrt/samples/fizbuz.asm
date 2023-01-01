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
reg_0_is_count:
reg_1_is_max_count:
  load %1 100
reg_2_mod_value:
  load %3 3
  load %5 5
  jmp loop_start
loop:
  inc %0
loop_start:
check_if_fizbuz:
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
check_if_buz:
  mod %0 %5 %2
  eq %2 %6
  jeq print_buz
loop_condition:
  geq %1 %0
  prti %0
  jeq loop
  hlt


