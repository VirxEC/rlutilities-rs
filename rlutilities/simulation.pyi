from typing import Optional, Tuple

try:
    from rlbot.utils.structures.game_data_struct import (BallInfo,
                                                         FieldInfoPacket,
                                                         GameTickPacket)
except ImportError:
    pass

_Shape = Tuple[int, ...]
__doc__: str

class Ball:
    def __init__(packet_ball: Optional[BallInfo]) -> Ball: ...
    def step(self, dt: float): ...

class Game:
    def __init__() -> Game: ...
    def set_mode(mode: str): ...
    def read_field_info(self, field_info: FieldInfoPacket): ...
    def read_packet(self, packet: GameTickPacket): ...
