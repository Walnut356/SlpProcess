# SlpProcess

A collection of libraries for handling .slp replay files. For more detailed info, check out the [wiki](https://github.com/Walnut356/SlpProcess/wiki)

### slp_parse

Main parser, contains everything necessary to turn a file path into an in-memory object containing all the file's information in a columnar format.

Feature flag `polars` adds DataFrame conversion impls for many existing types (frame events, stats, etc.). 

### ssbm_utils

Crate containing all of the various enums you'd expect from a parser, as well as functions to calculate various pieces of ingame behaviour (e.g. knockback trajectories, hitstun/hitlag)

Action State and Item enums should be 100% complete, sans a few "Unknowns". These were adapted directly from the ssbm decomp. As such, conversions to these enums should never fail in a typical replay. Some enums have "bonus" members for type-convenience, but these are given significantly "incorrect" values and have a block comment delimiting the end of the real members.

### py_slp_parse

~1:1 python mapping for slp_parse

### slp_db

WIP long-term storage library to easily table-ify replays and/or their resultant stats and store them via duckdb




Limitiations:

* No live parsing
* 1v1 only
