import polars

def parse(path: str) -> list[Game]:
    ...

class Game:
    ...

    def get_port_frames(port: int) -> polars.DataFrame:
        ...