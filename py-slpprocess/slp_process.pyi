from enum import Enum
from polars import DataFrame

class Frames:
    pre: DataFrame
    post: DataFrame

def parse(path: str) -> list[Game]: ...

class GameStart:
    random_seed: int
    is_teams: bool
    stage: int
    timer: int
    damage_ratio: float
    is_pal: bool | None
    is_frozen_stadium: bool | None
    is_netplay: bool | None
    match_id: str | None
    match_type: int | None
    game_number: int | None
    tiebreak_number: int | None

class Game:
    start: GameStart
    def get_port_frames(self, port: int) -> Frames: ...

class PreFrame(Enum):
    FRAME_NUMBER = 0
    RANDOM_SEED = 1
    ACTION_STATE = 2
    POSITION_X = 3
    POSITION_Y = 4
    FACING = 5
    JOYSTICK_X = 6
    JOYSTICK_Y = 7
    CSTICK_X = 8
    CSTICK_Y = 9
    TRIGGER = 10
    LOGICAL_BUTTONS = 11
    PHYSICAL_BUTTONS = 12
    PHYSICAL_L = 13
    PHYSICAL_R = 14
    PERCENT = 15
    def __str__(self) -> str: ...