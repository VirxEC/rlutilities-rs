import sys
from pathlib import Path

from .rlutilities import *

__doc__ = rlutilities.__doc__
if hasattr(rlutilities, "__all__"):
    __all__ = rlutilities.__all__

sys.modules["rlutilities.simulation"] = simulation
sys.modules["rlutilities.linear_algebra"] = linear_algebra

asset_dir = Path(__file__).parent / "assets"
initialize(asset_dir.as_posix() + "/")
