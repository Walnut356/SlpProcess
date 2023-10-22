import slp_process as slp
import polars as pl
import time
import slippistats as ss

now = time.time()
game = slp.parse(R"G:/temp")
fin = time.time()
print(fin - now)
print(len(game))

game = game[0]
x = game.players[0]
x = x.frames
now = time.time()
x = x.pre
fin = time.time()
print(fin - now)
print(x)
