# Rust + Nix Demo

A simple Rust/Axum web service deployed using Nix flakes, demonstrating reproducible builds and deployment.

## What This Demonstrates

- Nix flake configuration for Rust projects
- Reproducible builds across local and remote systems
- Deployment to AWS EC2 using Nix

## Quick Start

### Local Development
```bash
# Enter development environment
nix develop

# Run the service
cargo run
```

### Building with Nix
```bash
# Build reproducible binary
nix build

# Run the built binary
./result/bin/rust-nix-demo
```

## Deployment

Deployed to AWS EC2 t3.micro (Ubuntu 24.04) using Nix for reproducible deployment. First build takes ~15 min on t2.micro due to initial binary downloads and compiling Rust toolchain. Subsequent builds are faster due to Nix caching.

## Learning Benchmarks


