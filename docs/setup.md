# Setup

```bash
cargo update
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools
cargo install cargo-generate

brew install arm-none-eabi-gdb
brew install openocd
brew install qemu
```

connect the mini-usb cable to the 'USB ST-LINK'

```bash
openocd -f interface/stlink.cfg -f target/stm32f3x.cfg
```

which should output something like this:

```bash
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : clock speed 1000 kHz
Info : STLINK V2J37M26 (API v2) VID:PID 0483:374B
Info : Target voltage: 2.885647
Info : [stm32f3x.cpu] Cortex-M4 r0p1 processor detected
Info : [stm32f3x.cpu] target has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f3x.cpu on 3333
Info : Listening on port 3333 for gdb connections
```

```bash
rustup target add thumbv7m-none-eabi
cargo build --target thumbv7m-none-eabi
```
