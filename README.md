# (Blazingly Fast) Bun Version Manager 

Bun Version Manager build using Rust that allows you to quickly install and use different versions of Bun via the command line. This is just a side project I did in order to learn Rust, so is not usable at all.

# Installation
Step 1:
```bash
git clone git@github.com:chrisdadev13/bvm.git && cd bvm 
```  

Step 2:
```bash
cargo install && cargo build
cp ./target/debug/bvm /usr/local/bin
```


The goal is to eventually migrate the project from Rust to Zig!

```bash
bvm --help
```

## Usage

To install an specific version of Bun:
```bash
bvm install v1.0.21
```

To list available versions of Bun, you can use:
```bash
bvm ls-remote
```

To list installed versions of Bun, use: 
```bash
bvm ls
```

And finally to switch from a version to another use:
```bash
bvm use v1.0.20
```

Uninstall command is not ready yet :3 Feel free to contribute
