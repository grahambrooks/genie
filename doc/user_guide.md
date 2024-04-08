# User Guide

Genie is a general tool that can be used in a variety of use cases. These are some ideas on how to use the tool.


## Command line queries

Genie can be used to answer questions from the command line. For example:

```bash
genie --model 'ollama::mistral' How far is the earth from the sun`
```


> The average distance from the Earth to the Sun is approximately 93 million miles (150 million kilometers). This value is called an astronomical unit (AU), and it's commonly used as a standard unit of measurement for distances within our solar system. So, you can use "echo" or "print" command in your terminal to display this information:


## Generating code

```bash
genie --model ollama::mistral --code Rust function to generate a fibonacci series
```

**Output**

Here's a simple Rust function that generates a Fibonacci series up to the given number:

```rust
fn fibonacci(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        vec.push(a);
        let c = a + b;
        a = b;
        b = c;
    }

    vec
}
```

## Image generation 

TBD

### Summarize changes for a git commit

The following summarizes changes and commits those changes.

```bash
git diff | genie Summarize changes as a git commit message. | git commit -a -F -
```

OR

```bash
git diff | genie --model ollama::mistral  Summarize changes as a git commit message. | git commit -a -F -
```

Which is a little long-winded, so you can create an alias in your shell.

```bash
alias gcommit="git diff | genie --model ollama::mistral Summarize changes as a git commit message. | git commit -a -F -"
```

