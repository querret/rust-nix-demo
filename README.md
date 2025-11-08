# Rust + Nix Demo

Weekend project learning Nix flakes by building and deploying a Rust web service. Built to understand reproducible builds and Nix/Rust interaction.

## Live Demo

http://54.90.165.134:3000

Built on openSUSE Tumbleweed, deployed to AWS EC2 Ubuntu. 

## Quick Start

**Local dev:**
```bash
nix develop
cargo run
# visit http://localhost:3000
```

**Build with Nix:**
```bash
nix build
./result/bin/rust-nix-demo
```

## Tech Stack

- Rust 1.91 with Axum web framework
- Nix flakes for reproducible builds
- Deployed to AWS EC2 t3.micro (Ubuntu 24.04)

## What I Actually Learned

### The Basics
Flakes are Nix's build manager. The `flake.nix` defines inputs (where stuff comes from) and outputs (what you get). `flake.lock` pins everything to exact versions. Package management like cargo but for the entire build environment.

### Git Integration
Nix only builds what's in Git. Tried to build with uncommitted files and got errors immediately. Makes sense for reproducibility - can't build what isn't tracked. Forced me to keep commits clean.

### Development Workflow
`nix develop` drops you into a shell with the exact toolchain versions specified in the flake. Exit the shell, environment's gone. Enter again, it's identical. Similar to venv in Python. No more "works on my machine" arguments with collaborators.

`nix build` produces a `result/` symlink to the actual binary in `/nix/store/`. The store path is content-addressed - same inputs always produce the same output path.

### Deployment Was Surprisingly Clean
Built the binary on my openSUSE laptop. Used `nix copy --to ssh://...` to push it to the Ubuntu EC2 instance. Ran it. Worked immediately.

The binary works because Nix handles all runtime dependencies.

### Initial Blockers

**Experimental features:** Nix requires `nix-command` and `flakes` enabled in config. Not enabled by default yet. Added one line to `/etc/nix/nix.conf`.

**Trust signatures:** The remote VM didn't trust binaries from my laptop's Nix store initially. Fixed by adding my user to `trusted-users` in the remote Nix config.

**Build times on t3.micro:** Limited resources means buildind on t3.micro impoassible. Build on local machine, copied to vm.

**Dirty Git tree warnings:** Nix warns when building from uncommitted changes. Doesn't fail, just a reminder the build isn't 100% reproducible.

### Build System Impact

**Reproducibility** Same inputs = same outputs, every time. Can bisect builds to find exactly when something broke. Can build locally and deploy remotely with confidence.

**No dependency drift.** All developers get the same toolchain. New dev runs `nix develop`, they're productive immediately. No further setup docs.

**Aggresive Caching** Nix cache means most builds just download prebuilt binaries. Only recompile what actually changed. Makes CI fast.

**Clear separation.** Dev environment in `devShells`, production binary in `packages`. No confusion about what's for development vs what ships.

## Deployment Process

**Building on t3.micro doesn't work** - 1GB RAM isn't enough for Rust compilation. The instance either hangs or gets killed by OOM. Solution: build locally, copy the result.
```bash
# Build on local machine
nix build

# Copy built binary to remote
nix copy --to ssh://ubuntu@54.90.165.134 .

# On remote - binary is already in Nix store, build and run
cd ~/apps/rust-nix-demo
nix build  # creates result symlink to copied binary
./result/bin/rust-nix-demo
```

**Details:**
- Local machine does the heavy lifting (compilation)
- Remote machine receives the final binary
- Nix ensures the binary works on both systems (reproducibility)
- t3.micro's limited resources don't matter as it's serving HTTP

### Running as a System Service

Set up systemd to keep the service running persistently:

**Create service file:**
```bash
sudo nano /etc/systemd/system/rust-nix-demo.service
```

**Configuration:**
```ini
[Unit]
Description=Rust Nix Demo Web Service
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu/apps/rust-nix-demo
ExecStart=/home/ubuntu/apps/rust-nix-demo/result/bin/rust-nix-demo
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Enable and start:**
```bash
sudo systemctl daemon-reload
sudo systemctl enable rust-nix-demo
sudo systemctl start rust-nix-demo
```

**Verify:**
```bash
sudo systemctl status rust-nix-demo
sudo journalctl -u rust-nix-demo -f  # view logs
```

Service survives SSH disconnects and server reboots. Currently running at http://54.90.165.134:3000

## Future Improvements

- Set up proper DNS (planning nixdemo.snacky.dev)
- Add more endpoints to demonstrate routing
- Explore cross-compilation to other architectures

## Source

https://codeberg.org/querret/rust-nix-demo

Built to learn Nix. Mirrors to GitHub for visibility.