# Technus Timecode

A WIP timecode monitor that will support LTC, MTC and ArtNet as well as being cross-platform for Windows, MacOS and Linux

## Dev envirionment setup

You can run this project just like any other Rust project with

```
cargo run
```

### Dependancies

[egui has some depndancies](https://github.com/emilk/egui?tab=readme-ov-file#demo), here's how to install them:

Debian:

```
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

Fedora Rawhide:

```
dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

For NixOS, there is a `flake.nix` file included, so you just need to run

```
use flake
```
