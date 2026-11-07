# Terminal Pet 🐊🐀

A small virtual pet that lives in your terminal, written in Rust.

I'm building this project to learn Rust, so it's still in its early stages and will grow over time.

## Features

- Choose your pet's species: **Crocodile** or **Rat**
- Give your pet a name
- Your pet is printed to the terminal

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

## Getting started

Clone the repository and run the program with Cargo:

```sh
git clone https://github.com/venla-v/TerminalPet.git
cd TerminalPet
cargo run
```

## Usage

When the program starts, it asks you two questions:

1. **Species** – type `C` for Crocodile or `R` for Rat (uppercase). If you type anything else, it asks again.
2. **Name** – type any name you like.

Example session:

```text
Choose which type of pet you want. Type C for Crocodile or R for Rat: C
Set a name for the pet: Snappy
(Snappy, Crocodile)
```

## Project structure

```text
.
├── Cargo.toml   # Project metadata and dependencies
└── src
    └── main.rs  # The whole program (for now)
```

## Roadmap

Ideas for what comes next:

- [ ] Pet stats (e.g. hunger, happiness, energy)
- [ ] Actions such as feeding and playing with the pet
- [ ] A game loop so the pet lives on between commands
- [ ] Saving and loading the pet
- [ ] More species

## License

No license has been chosen yet.
