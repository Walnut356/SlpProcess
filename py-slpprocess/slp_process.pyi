from enum import Enum
from polars import DataFrame

class Frames:
    pre: DataFrame
    post: DataFrame

class Player:
    frames: Frames
    nana_frames: Frames | None
    @property
    def character(self) -> int: ...
    @property
    def costume(self) -> int: ...
    @property
    def port(self) -> int: ...
    @property
    def connect_code(self) -> str | None: ...
    @property
    def display_name(self) -> str | None: ...
    @property
    def is_winner(self) -> bool | None: ...

    def __init__(self):
        self.__character: int
        self.__costume: int
        self.__port: int
        self.__connect_code: str | None
        self.__display_name: str | None
        self.__is_winner: bool | None


def parse(path: str) -> list[Game]: ...

class Game:

    @property
    def players(self) -> list[Player]: ...
    @property
    def random_seed(self) -> int: ...
    @property
    def is_teams(self) -> bool: ...
    @property
    def stage(self) -> int: ...
    @property
    def timer(self) -> int: ...
    @property
    def damage_ratio(self) -> float: ...
    @property
    def is_pal(self) -> bool | None: ...
    @property
    def is_frozen_stadium(self) -> bool | None: ...
    @property
    def is_netplay(self) -> bool | None: ...
    @property
    def match_id(self) -> str | None: ...
    @property
    def match_type(self) -> int | None: ...
    @property
    def game_number(self) -> int | None: ...
    @property
    def tiebreak_number(self) -> int | None: ...
    @property
    def end_method(self) -> int | None: ...
    @property
    def lras_initiator(self) -> int | None: ...
    @property
    def placements(self) -> list[int]: ...

    def __init__(self, path: str):
        self.__players: list[Player]
        self.__random_seed: int
        self.__is_teams: bool
        self.__stage: int
        self.__timer: int
        self.__damage_ratio: float
        self.__is_pal: bool | None
        self.__is_frozen_stadium: bool | None
        self.__is_netplay: bool | None
        self.__match_id: str | None
        self.__match_type: int | None
        self.__game_number: int | None
        self.__tiebreak_number: int | None
        self.__end_method: int | None
        self.__lras_initiator: int | None
        self.__placements: list[int]
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