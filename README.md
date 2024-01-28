# SolarReader

Lorem ipsum....

# Instructions

## Dev Environment

**Pre-Requisites:**
- Ubuntu
- rustup

**Dependencies:**
Run the `./depfile` from the repository root to install all dependencies and setting up the rp2040js emulator.

**Emulator:**
Invoking `cargo r` will compile the code for target `thumbv6m-none-eabi` (ARM M0, M0+, M1), create an ihex, and execute it on the rs2040js emulator using node.
