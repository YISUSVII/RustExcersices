# RustExercises — Zero → Ninja

A practical, progressive Rust course repository designed to take you from **absolute beginner** to **advanced/professional** level through hands-on exercises.

This repo is focused on **learning Rust by building** (not interview drills or DevOps-heavy content).

## Prerequisites

- A computer with macOS, Linux, or Windows
- Basic command line familiarity
- Motivation to practice consistently

## Installation

1. Install Rust with `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Verify tools:
   ```bash
   rustc --version
   cargo --version
   ```
3. Clone this repository and open it locally.

## Rust Zero → Ninja

```text
ZERO
↓
Rust syntax and fundamentals

BEGINNER
↓
Ownership, structs, enums, collections

ADVANCED
↓
Traits, lifetimes, iterators, testing

NINJA
↓
Concurrency, async, macros, unsafe Rust and real projects
```

## Learning roadmap

| Level | Topic                       | Difficulty   | Status |
| ----- | --------------------------- | ------------ | ------ |
| 0     | Setup                       | Beginner     | ⬜      |
| 1     | Basics                      | Beginner     | ⬜      |
| 2     | Control Flow                | Beginner     | ⬜      |
| 3     | Functions                   | Beginner     | ⬜      |
| 4     | Ownership & Borrowing       | Intermediate | ⬜      |
| 5     | Structs & Enums             | Intermediate | ⬜      |
| 6     | Pattern Matching            | Intermediate | ⬜      |
| 7     | Collections & Strings       | Intermediate | ⬜      |
| 8     | Error Handling              | Intermediate | ⬜      |
| 9     | Generics, Traits, Lifetimes | Advanced     | ⬜      |
| 10    | Iterators & Closures        | Advanced     | ⬜      |
| 11    | Modules & Crates            | Advanced     | ⬜      |
| 12    | Testing                     | Advanced     | ⬜      |
| 13    | Smart Pointers              | Advanced     | ⬜      |
| 14    | Concurrency                 | Advanced     | ⬜      |
| 15    | Async Rust                  | Advanced     | ⬜      |
| 16    | Macros                      | Advanced     | ⬜      |
| 17    | Advanced Rust               | Ninja        | ⬜      |
| 18    | Projects                    | Ninja        | ⬜      |

## Repository structure

```text
RustExercises/
├── README.md
├── 00-setup/README.md
├── 01-basics/README.md
├── 01-basics/exercises/
├── 02-control-flow/README.md
├── 03-functions/README.md
├── 04-ownership-borrowing/README.md
├── ... (continue through 18-projects, each with README.md)
├── solutions/
└── resources/
```

> Note: each populated learning level includes an `exercises/` directory, and `solutions/` mirrors solved files by level.

## Recommended learning order

1. `00-setup` → install tools and first project.
2. `01-basics` to `04-ownership-borrowing` → fundamentals and ownership mastery.
3. `05-structs-enums` to `10-iterators-closures` → idiomatic Rust building blocks.
4. `11-modules-crates` to `17-advanced-rust` → professional-level concepts.
5. `18-projects` → capstone implementations.

## How to run exercises

- Read the module `README.md` first.
- Start from the provided starter file in `exercises/`.
- Compile frequently:
  ```bash
  rustc your_file.rs
  ```
  or with Cargo when a project skeleton is provided.
- Embrace compiler errors and fix them incrementally.

## How solutions work

- Exercise prompts are in module folders.
- Reference solutions are in `solutions/` with matching paths/names.
- Try solving first, then compare with the solution.

## Progress checklist

- [ ] Complete Level 0 setup and Hello World
- [ ] Complete Levels 1–4 (fundamentals + ownership)
- [ ] Complete Levels 5–10 (core intermediate concepts)
- [ ] Complete Levels 11–17 (advanced and ninja topics)
- [ ] Complete at least 5 projects from `18-projects`

## Rust style in this repository

Prefer the Rust way:
- `Result<T, E>` and `Option<T>` for error/absence handling
- borrowing and ownership-aware APIs
- iterators and pattern matching where they improve clarity
- standard library first; third-party crates only when educationally useful
