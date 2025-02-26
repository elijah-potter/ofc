# Ollama Function Caller (`ofc`)

The Ollama function caller, otherwise known as `ofc`, is a command-line tool for prompting Ollama models locally on your system.
There are other programs out there that do similar things, but they either don't support streaming or don't give me access to important settings, like context length or temperature.

In order to use `ofc`, you need to have Ollama on your system. You may install it from the [Ollama website](https://ollama.com/).

## Installation

`ofc` is installable from either `crates.io` or this repository.

```bash
cargo install ofc --locked

# Or...
cargo install --git https://github.com/elijah-potter/ofc --locked
```

## Usage

It's pretty simple. Just call `ofc` with the user prompt of your desire.

```bash
ofc "What is the meaning of life?"
```

You may can change the model from the default (phi4) and control context size and temperature.

```bash
ofc --context 8192 --temperature 0.3 --model tinyllama "What is the best pizza?"
```

## Previous Art

`ofc` was inspired by [`ooo`](https://github.com/Npahlfer/ooo).
