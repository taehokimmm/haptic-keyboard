# haptic-keyboard

This is part of a Individual Research project at HCIL, which implements a haptic output to a palmrest from keyboard input.

## Project Structure

- LLHF: LLHF(Low Latency Haptic Feedback) implements haptic output with a CLI interface. The hapic output only generates when you *type directly* into the CLI.

- LLHF-global: LLHF-global catches global keyboard input and sends hapic output. The haptic output generates when you *type anywhere* on your computer.

Both projects outputs haptic feedback as a audio signal. The haptic apparatus used in this research converts audio inputs to haptic feedback.

## How to run

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

You can install Rust by running the following command:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Haptic Palmrest Apparatus

- Phoneme Files
You need a folder named `phoneme-whc-modified` in each of the project directories. The folder should contain the audio files used for haptic output. The mentioned files are not included in this repository.

### Compile and Run

To compile and run the project, run the following command:

- LLHF

```shell
cd llhf
cargo run
```

- LLHF-global

```shell
cd llhf-global
cargo run
```

### Terminating the program

- LLHF: Press `Esc` key twice to terminate the program.
- LLHF-global: Press `Ctrl + C` to terminate the program.