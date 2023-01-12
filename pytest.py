from random import uniform, randint

from rlbot.utils.structures.game_data_struct import (BoostPad, BoxShape,
                                                     FieldInfoPacket,
                                                     GameTickPacket, GoalInfo,
                                                     Physics, PlayerInfo,
                                                     Rotator, ScoreInfo,
                                                     Vector3)

from rlutilities.simulation import Ball, Game
from rlutilities.linear_algebra import vec3

def get_field_info() -> FieldInfoPacket:
    packet = FieldInfoPacket()

    packet.num_goals = 2
    packet.goals[0] = GoalInfo(0, Vector3(0, 5120, 300), Vector3(0, -1, 0), 300, 100)
    packet.goals[1] = GoalInfo(1, Vector3(0, -5120, 300), Vector3(0, 1, 0), 300, 100)

    packet.num_boosts = 15
    for i in range(packet.num_boosts):
        packet.boost_pads[i] = BoostPad(Vector3(15, 3, 0.1), True)

    return packet

def get_random_packet():
    packet = GameTickPacket()

    packet.game_ball.physics.location.x = uniform(-4000, 4000)
    packet.game_ball.physics.location.y = uniform(-5020, 5020)
    packet.game_ball.physics.location.z = uniform(100, 1944)

    packet.game_ball.physics.velocity.x = uniform(-2000, 2000)
    packet.game_ball.physics.velocity.y = uniform(-2000, 2000)
    packet.game_ball.physics.velocity.z = uniform(-2000, 2000)

    packet.game_ball.physics.angular_velocity.x = uniform(-1, 1)
    packet.game_ball.physics.angular_velocity.y = uniform(-1, 1)
    packet.game_ball.physics.angular_velocity.z = uniform(-1, 1)

    packet.game_ball.collision_shape.type = 1
    packet.game_ball.collision_shape.sphere.diameter = 182.5
    packet.game_info.world_gravity_z = -650.
    packet.game_info.seconds_elapsed = 0.469

    packet.num_cars = 4
    for i in range(packet.num_cars):
        packet.game_cars[i] = PlayerInfo(
            physics=Physics(
                location=Vector3(uniform(-4000, 4000), uniform(-5020, 5020), uniform(100, 1944)),
                rotation=Rotator(0, 1.1, 0),
                velocity=Vector3(uniform(-2000, 2000), uniform(-2000, 2000), uniform(-2000, 2000)),
                angular_velocity=Vector3(uniform(-1, 1), uniform(-1, 1), uniform(-1, 1)),
            ),
            score_info=ScoreInfo(0, 0, 0, 0, 0, 0, 0),
            is_demolished=False,
            has_wheel_contact=False,
            is_super_sonic=False,
            is_bot=True,
            team=0,
            name="DownToEarth",
            jumped=False,
            double_jumped=False,
            boost=50,
            hitbox=BoxShape(
                length=118,
                width=84.2,
                height=36.2,
            ),
            hitbox_offset=Vector3(13.9, 0, 20.8),
            spawn_id=randint(0, 2**30),
        )

    return packet

Game.set_mode("soccar")
game = Game()
game.read_field_info(get_field_info())
game_tick_packet = get_random_packet()
game.read_packet(game_tick_packet)

random_vec3 = vec3(randint(-4000, 4000), randint(-5020, 5020), randint(100, 1944))
vec3_copy = vec3(random_vec3)
vec3_copy.z = 0
print(random_vec3)
print(vec3_copy)
assert random_vec3.z != vec3_copy.z

random_ball = Ball()
packet_ball = Ball(game_tick_packet.game_ball)
packet_ball.step(1/120)
