# Basic Options Pricer

This Rust package demonstrates a simple options pricer allowing use of both a
binomial tree and black-scholes methodology.

On top of this it provides python bindings for calling these methods from
python code.

## Python Integration

Currently as this is experimentation the Cargo.toml is setup to export the
library under the name "pricer". To do so you will need to:

1. Setup a virtual environment in the repo, run `python -m venv .env` in root
2. Install maturin for ease of use, `pip install maturin`
3. Run `maturin build` or `maturin develop` to access it locally in this
environment
4. See test scripts for usage
