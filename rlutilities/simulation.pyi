from typing import Optional, Tuple

from rlutilities.linear_algebra import vec3

try:
    from rlbot.utils.structures.game_data_struct import (FieldInfoPacket,
                                                         GameTickPacket)
except ImportError:
    pass

_Shape = Tuple[int, ...]
__doc__: str

class Ball:
    time: float
    position: vec3
    velocity: vec3
    angular_velocity: vec3

    def __init__(ball: Optional[Ball]) -> Ball: ...
    def __str__(self): ...
    def __repr__(self): ...
    def step(self, dt: float): ...

class Game:
    ball: Ball

    def __init__() -> Game: ...
    def set_mode(mode: str): ...
    def read_field_info(self, field_info: FieldInfoPacket): ...
    def read_packet(self, packet: GameTickPacket): ...

class Field: ...
