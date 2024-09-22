<p align="center">
  <h1 align="center">
    nexus2fa
  </h1>

  <p align="center">
    <a href="https://img.shields.io/badge/version-0.1.0dev-green" target="_blank">
      <img alt="Version Badge" src="https://img.shields.io/badge/version-0.0.1-green">
    </a>
    <a href="https://crates.io/crates/chaintools" target="_blank">
      <img alt="Crates.io Version" src="https://img.shields.io/crates/v/nexsu2fa">
    </a>
    <a href="https://github.com/alejandrogzi/chaintools" target="_blank">
      <img alt="GitHub License" src="https://img.shields.io/github/license/alejandrogzi/nexsu2fa?color=blue">
    </a>
    <a href="https://crates.io/crates/chaintools" target="_blank">
      <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/nexsu2fa">
    </a>
  </p>

  <p align="center">
    decided to make this because the other tools could not convert my .nex to .fa
  </p>
</p>


## Usage
### Binary
``` rust
Usage: nexus2fa -n/--nexus <NEXUS> -f/--fasta <FASTA>

Arguments:
    -n, --nexus <NEXUS>: NEXUS file
    -f, --fasta <FASTA>: FASTA file

Options:
    --help: print help
    --version: print version
```
#### crate: [https://crates.io/crates/nexus2fa](https://crates.io/crates/nexus2fa)

## Build
to build nexus2fa from this repo, do:

1. get rust
2. run `git clone https://github.com/alejandrogzi/nexus2fa.git && cd nexus2fa`
3. run `cargo run --release -- -n <NEXUS> -f <FASTA>`
