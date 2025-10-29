# TOON Python Bindings

Python bindings for TOON (Token-Oriented Object Notation).

## Build and Install

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Or build a wheel
maturin build --release
pip install target/wheels/*.whl
```

## Usage

```python
import json
import toon

data = {"name": "Ada", "active": True}
result = toon.encode(json.dumps(data))
print(result)
```

## Publish

```bash
maturin build --release
pip install twine
twine upload target/wheels/*
```
