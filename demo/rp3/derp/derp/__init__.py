from .ders import ders  # loads the native Rust module

# from .sim import util

# from . import ders as rust_mod

# # Manually attach nested modules to make them importable
# opt = getattr(rust_mod, "opt", None)
# if opt is not None:
#     globals()["opt"] = opt
#     import sys

#     sys.modules["ders.opt"] = opt
#     grb = getattr(opt, "grb", None)
#     if grb is not None:
#         sys.modules["ders.opt.grb"] = grb

# preserve docs & all
__doc__ = ders.__doc__
if hasattr(ders, '__all__'):
    __all__ = ders.__all__
