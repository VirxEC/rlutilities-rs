__doc__: str

class Ball:
    pass

try:
    from rlbot.utils.structures.game_data_struct import (FieldInfoPacket,
                                                         GameTickPacket)
except ImportError:
    pass

class Game:
    def __new__() -> Game: ...
    def set_mode(mode: str): ...
    def read_field_info(self, field_info: FieldInfoPacket): ...
    def read_packet(self, packet: GameTickPacket): ...
