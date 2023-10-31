# dev-shell a rust cli for interacting with ChatGPT

[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)

## Description

This is a ChatGPT CLI application written in Rust. The application allows users to interact with the ChatGPT API for AI text generation within their terminal.

If you do a quick search of the internet or GitHub for projects that are using ChatGPT in some way you will find quite a few. Many use interpreted languages and need your system to be setup to use the toolchain and dependencies. dev-shell is written in rust. A key goal of the project is to make it super easy to install. Typiclly download a single binary and you're done. (except fo the small dependency on an OpenAI application key)

As a cli the application can accept input from other tools.

```bash
git log HEAD~2 | dev-shell Summarize as a release note
```

### Project build and release status

![ci](https://github.com/grahambrooks/dev-shell/actions/workflows/ci.yaml/badge.svg) ![release](https://github.com/grahambrooks/dev-shell/actions/workflows/build.yaml/badge.svg) ![security audit](https://github.com/grahambrooks/dev-shell/actions/workflows/security-audit.yaml/badge.svg)

## Installing

### Prerequisites

- Rust stable https://www.rust-lang.org/tools/install or via rustup https://rustup.rs

You will also need a ChatGPT API key. You can get one from https://openai.com and then set it as an environment variable.

```bash
export OPENAI_API_KEY=<your key>
```

### Install from crates.io

```bash
cargo install dev-shell
````

## Building

### Prerequisites

- Rust stable https://www.rust-lang.org/tools/install

Clone the repository and run the tests

```bash
cargo test
```

Build the application

```bash
cargo build --release
```

add the binary `target/release/dev-shell` to your path or copy to a directory that is already on your path.

### Building the web assets

The web application css uses tailwindcss and is built/rebuilt using npx:

```bash
npx tailwindcss -i ./static/input.css -o ./static/site.css --watch
```

## Use-cases

### Running as a local web appliction

```bash
dev-shell --server
```
This starts the app as a local web server available on http://localhost:3000


### Summarize for a git commit 

The following summarizes changes and commits those changes.

```bash
git diff | dev-shell Summarize changes as a git commit message. | git commit -a -F -
```

Which is a little long-winded, so you can create an alias in your shell.

```bash
alias dscommit="git diff | dev-shell Summarize changes as a git commit message. | git commit -a -F -"
```
## Repository maintenance

Currently, repository maintenance is manual and run semi regularly.

### Updating the rust toolchain

```bash
rustup update
```

### Updating dependencies

The following command will update the Cargo.lock file with the latest versions of dependencies.

```bash
cargo update
```

## Inspiration

This project was inspired by the follow and the desire to learn Rust.

https://github.com/TheR1D/shell_gpt