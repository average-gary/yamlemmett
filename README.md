# yamlemmett

A simple command-line tool to convert YAML to JSON. Written in Rust.

## Installation

```bash
cargo install --path .
```

## Usage

The tool can read YAML from a file or stdin and output JSON to a file or stdout.

### Examples

Convert a YAML file to JSON and print to stdout:
```bash
yamlemmett -i input.yaml
```

Convert YAML from stdin to JSON and save to a file:
```bash
cat input.yaml | yamlemmett -o output.json
```

Convert a YAML file to JSON and save to a file:
```bash
yamlemmett -i input.yaml -o output.json
```

## Building from source

```bash
git clone https://github.com/average-gary/yamlemmett.git
cd yamlemmett
cargo build --release
```

## License

MIT 