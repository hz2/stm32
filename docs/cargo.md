# Cargo

## `readobj`

display the [ELF] file headers of the binary

```bash
cargo readobj --bin stm32 -- --file-headers
```

## `size`

display size of the linker sections in the binary

```bash
cargo size --bin stm32 --release -- -A
```

- `.text` contains the program instructions
- `.rodata` contains constant values like strings
- `.data` contains statically allocated variables whose initial values are not zero
- `.bss` also contains statically allocated variables whose initial values are zero
- `.vector_table` is a non-standard section that we use to store the vector (interrupt) table
- `.ARM.attributes` and the `.debug_*` sections contain metadata and will not be loaded onto the target when flashing the binary.

## `objdump`

disassemble the binary

```bash
cargo objdump --bin stm32 --release -- --disassemble --no-show-raw-insn --print-imm-hex
```
