import sys

from .rlutilities import *

__doc__ = rlutilities.__doc__
if hasattr(rlutilities, "__all__"):
    __all__ = rlutilities.__all__

sys.modules["rlutilities.simulation"] = simulation
