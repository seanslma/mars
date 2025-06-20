# fix `AttributeError: module 'mars' has no attribute 'opt'`

You're using the [**Polars-style layout**](https://github.com/pola-rs/polars/blob/main/py-polars/src/lib.rs) where:

* The Python package is called `mars`
* The Rust-compiled extension is named `mars.mars` (i.e., `lib.name = "mars"` inside `mars_py/Cargo.toml`)
* `mars/__init__.py` loads the compiled native module with `from .mars import *`


## ✅ What's Wrong

Even though you’ve done this:

```python
from .mars import *
```

Python **still doesn’t see `mars.opt`** because:

1. You're importing everything from `mars.mars` into `mars`’s `__init__.py`, **but not setting `mars.opt` manually**.
2. Nested modules like `mars.opt.grb` only exist **inside the compiled extension**, and `from .mars import *` does **not copy over nested submodules** to the `mars` Python package’s `__init__.py`.

---

## ✅ The Fix

You need to **forward `opt` from the native extension manually** in `mars/__init__.py`.

### 🔧 `mars/__init__.py`:

```python
from .mars import *  # loads the native Rust module
from . import mars as rust_mod

# Manually attach nested modules to make them importable
opt = getattr(rust_mod, "opt", None)
if opt is not None:
    globals()["opt"] = opt
    import sys
    sys.modules["mars.opt"] = opt
    grb = getattr(opt, "grb", None)
    if grb is not None:
        sys.modules["mars.opt.grb"] = grb

# preserve docs & all
__doc__ = rust_mod.__doc__
if hasattr(rust_mod, "__all__"):
    __all__ = rust_mod.__all__
```

---

## 🔁 Reinstall After Editing

```bash
# In mars_py/
maturin develop --release
```

---

## 🧼 Summary

* You're correctly building with PyO3.
* The issue is Python’s import system doesn't auto-expose nested modules from compiled extensions.
* You must manually wire up `mars.opt` and `mars.opt.grb` inside `__init__.py`.

Let me know if you'd like a full `tree` or a zip scaffold of this layout!


## Avoid adding submodules manually
Actually if we constructs the full module hierarchy directly, there is no need to add the submodules to the python sys.
