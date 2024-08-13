# SWRT (Snow Run Time)

#### Header
| offset | size | purpose                                       |
|:-------|:----:|:---------------------------------------------:|
|  0x00  |  4   | 0x7F followed by NOW(45 4c 46) in ASCII;      |
|        |      | these four bytes constitute the magic number. |
|  0x04  |  4   |   How big the .data section is                |
|  0x08  |  4   |   Entry point into .text section              |
|  0x0C  |  52  |   Not used section                              |

#### Insturctions Supported

|instruction| arg1 | arg2 | arg3 |
|:----------|:----:|:----:|:----:|
|    load   | reg  |     imm     |
|    push   | reg  |     N/A     |
|    pop    | reg  |     N/A     |
|    aloc   | reg  |     N/A     |
|    add    | reg  | reg  | reg  |
|    sub    | reg  | reg  | reg  |
|    div    | reg  | reg  | reg  |
|    mul    | reg  | reg  | reg  |
|    eq     | reg  | reg  | N/A  |
|    neq    | reg  | reg  | N/A  |
|    gt     | reg  | reg  | N/A  |
|    geq    | reg  | reg  | N/A  |
|    lt     | reg  | reg  | N/A  |
|    leq    | reg  | reg  | N/A  |
|    setm   | reg  | reg  | N/A  |
|    inc    | reg  |     N/A     |
|    dec    | reg  |     N/A     |
|    prti   | reg  |     N/A     |
|    jmp    |      label name    |
|    jeq    |      label name    |
|    jne    |      label name    |
|    prts   |      label name    |
|    hlt    |        N/A         |
|    nop    |        N/A         |
|    ige    |        N/A         |
