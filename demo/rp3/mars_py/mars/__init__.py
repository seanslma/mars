# from .mars import *  # loads the native Rust module

from .mars import sim

from .sim import *

# from . import mars as rust_mod

# # Manually attach nested modules to make them importable
# opt = getattr(rust_mod, "opt", None)
# if opt is not None:
#     globals()["opt"] = opt
#     import sys

#     sys.modules["mars.opt"] = opt
#     grb = getattr(opt, "grb", None)
#     if grb is not None:
#         sys.modules["mars.opt.grb"] = grb

# preserve docs & all
__doc__ = mars.__doc__
if hasattr(mars, '__all__'):
    __all__ = mars.__all__
