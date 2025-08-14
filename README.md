# hacky

hacky is an assembler for the hack computer, as described in the nand2tetris course. it translates hack assembly language (.asm files) into binary machine code (.hack files) that can be executed on the hack hardware platform. the implementation conforms to the hack language specification outlined in [project 4 of nand2tetris](https://www.nand2tetris.org/project04).

## features
- supports all hack assembly commands:
  - `@value` for a-instructions (addressing values or variables).
  - [`dest=comp;jump`](src/parser.rs ) for c-instructions (computation and control flow).
  - `(label)` for labels used in jump instructions.
- handles predefined symbols (`R0`-`R15`, `SCREEN`, `KBD`, etc.).
- supports user-defined symbols and variables, starting at memory address 16.
- removes comments and whitespace during parsing.
- validates commands for correctness, including:

## usage
1. compile the project using cargo:
   ```bash
   cargo build --release
   ```

2. run the assembler:
   ```bash
   ./target/release/hacky <SRC> --out <OUT>
   ```
   - `<SRC>`: path to the input `.asm` file.
   - `<OUT>`: path to the output `.hack` file.

## example
given the following input file `Test.asm`:
```asm
@2
D=A
@3
D=D+A
@0
M=D
```

the assembler will produce the following output in `Test.hack`:
```hack
0000000000000010
1110110000010000
0000000000000011
1110000010010000
0000000000000000
1110001100001000
```

## testing
basic unit tests are included to verify the correctness of the parser
```bash
cargo test
```

## hack language specification
this assembler adheres to the hack language specification as described in [project 4 of nand2tetris](https://www.nand2tetris.org/project04). it supports:
- a-instructions for direct addressing and symbolic variables.
- c-instructions for computation and control flow.
- labels for jump destinations.
- predefined symbols and user-defined variables.

## limitations
- does not optimise the generated machine code.
