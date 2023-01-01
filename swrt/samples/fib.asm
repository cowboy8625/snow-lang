#!/home/cowboy/.cargo/bin/swrt
.entry main
.data
.text
main:
a:
    load %0 1
b:
    load %1 1
    load %2 46
loop:
    push %1
    add %0 %1 %1 
    pop %0
    inc %3
    eq %3 %2
    jne loop
    prti %0
    hlt

