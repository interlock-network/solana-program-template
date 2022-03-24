# Solana Program Template

This README outlines the canonical (best practice) structure of a Solana program written in Rust.

This repository contains a template Solana program in Rust.

> _Language note: a smart contract is called a **program** in Solana._

### This template compiles, and can perform, but instructionOne needs right accounts created first to work right

Compiling goes like so.

From the ```template``` directory, run:
```
cargo build-bpf
```

### To deploy this program to a Solana test-validator, after building run the following 

From this repository's root, run:
```
solana program deploy template/target/deploy/template.so
```

### The template has a structure like this:

```
.
├── entrypoint
│   ├── entrypoint.rs
│   └── mod.rs
├── error
│   ├── error.rs
│   └── mod.rs
├── instruction
│   ├── data.rs
│   ├── mod.rs
│   └── unpack.rs
├── lib.rs
├── processor
│   ├── instructionOne.rs
│   ├── instructionThree.rs
│   ├── instructionTwo.rs
│   ├── mod.rs
│   └── run.rs
├── state
│   ├── FIRST.rs
│   ├── SECOND.rs
│   └── mod.rs
└── utils
    ├── mod.rs
    └── utils.rs
```


## Solana program anatomy:

A Solana program has five general components:

1. entrypoint
2. error
3. instruction
4. processor
5. state

In a simple program, we can assign a single .rs file to each of these components.

For larger projects however, it helps to make these into directory modules.

It is also convenient to dedicate a module to utilities: constants and functions.

For small programs, these can be jammed into a single file, though that's gross.

For not-huge programs, each directory can just be a .rs file.

### 1. entrypoint

An excerpt from Solana docs:

>Solana on-chain programs are compiled via the LLVM compiler infrastructure to an Executable and Linkable Format (ELF) containing a variation of the Berkeley Packet Filter (BPF) bytecode.
>
>Because Solana uses the LLVM compiler infrastructure, a program may be written in any programming language that can target the LLVM's BPF backend. Solana currently supports writing programs in Rust and C/C++.
>
>BPF provides an efficient instruction set that can be executed in an interpreted virtual machine or as efficient just-in-time compiled native instructions.

A program is deployed and executed by a **loader** that serializes the program input data/parameters, and calls the program's entrypoint.

The ```entrypoint!``` macro within entrypoint.rs takes in this serialized data and deserializes the main parameters: ```program_id```, ```accounts```, and ```instruction_data```. It is up to the program to deserialize the ```instruction_data```.

Because the entrypoint.rs code is so small, I'll just include it here:

```rs
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::run_process(program_id, accounts, instruction_data)
}
```

### 2. error

This is self explanatory. We define additional errors that are specific to our particular program.

### 3. instruction

The instruction module contains everything we need to process our ```instruction_data```. This includes determining the called instruction, and unpacking additional parameters passed along with the instruction specification.

### 4. processor

The processor module contains everything we need to execute an instruction and do the stuff it specifies.

### 5. state

The state module defines the state variables for any accounts accessed by the Solana program. There are also specifications for how to pack and unpack these account states.

.

.

.

```
TODO:
. expound on 2-5
x explain how to compile
x explain how to deploy
. comment template better
```
