from rlbot.utils.structures.game_data_struct import (BoostPad, FieldInfoPacket,
                                                     GoalInfo, Vector3)

from rlutilities.simulation import Ball, Game


def get_field_info() -> FieldInfoPacket:
    packet = FieldInfoPacket()

    packet.num_goals = 2
    packet.goals[0] = GoalInfo(0, Vector3(0, 5120, 300), Vector3(0, -1, 0), 300, 100)
    packet.goals[1] = GoalInfo(1, Vector3(0, -5120, 300), Vector3(0, 1, 0), 300, 100)

    packet.num_boosts = 15
    for i in range(packet.num_boosts):
        packet.boost_pads[i] = BoostPad(Vector3(15, 3, 0.1), True)

    return packet

Game.set_mode("soccar")
game = Game()
game.read_field_info(get_field_info())
