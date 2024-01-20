# SlpProcess

Mostly complete Rust backend for slippi parsing, meant to succeed my python [SlippiStats](https://github.com/Walnut356/SlippiStats) library. Focuses specifically on stats and easy transfer to python.

## slp_parse

Main parser, contains everything necessary to turn a file path into an in-memory object containing all the file's information in a columnar format.

## ssbm_utils

Crate containing all of the various enums you'd expect from a parser, as well as functions to calculate various pieces of ingame behaviour (e.g. knockback trajectories, hitstun/hitlag)

## py_slp_parse

1:1 python mapping for slp_parse

## slp_db

WIP long-term storage library to easily table-ify replays and/or their resultant stats and store them via duckdb




Limitiations:

* No live parsing
* 1v1 only
* Enum-ing all the values is not done by the backend
* Some fields and replay features that aren't directly relevant to stats are missing
