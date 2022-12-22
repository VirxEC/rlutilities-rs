__doc__: str

class Ball:
    pass

try:
    from rlbot.utils.structures.game_data_struct import FieldInfoPacket
except ImportError:
    pass

class Game:
    def __new__() -> Game: ...
    def read_field_info(self, field_info: FieldInfoPacket): ...
    def set_mode(mode: str): ...