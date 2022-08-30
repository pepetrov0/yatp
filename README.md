# YATP (Yet Another Texture Packer)

A small and simple CLI application to pack multiple textures/sprites into a texture atlas/sprite sheet.

## Installation

### Through Cargo

```sh
cargo install yatp-cli
```

## Usage

```sh
yatp-cli [OPTIONS] <INPUTS...>
```

Options:

- `-o` - Name of the output files, default: `atlas`;
- `-d` - Dictionary format, if not provided - dictionary won't be serialized;
- `-i` - Image format of the atlas, default: `png`;
- `-h` - Height of the texture atlas, default: `1024`;
- `-w` - Weight of the texture atlas, default: `1024`;
- `-g` - Gap between packed textures, default: `0`;
- `--help` - Prints help;
- `-V` - Prints version;
