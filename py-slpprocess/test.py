import slp_process as slp
import polars as pl
import time
import slippistats as ss

now = time.time()
game = slp.parse(R"./Game_20230526T020459.slp")
fin = time.time()
print(fin - now)
print(len(game))

frames = game[0].start

now = time.time()
print(frames)
fin = time.time()
print(fin - now)