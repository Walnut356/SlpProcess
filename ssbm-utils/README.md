# ssbm-utils

An unorganized collection of useful functions for calculating various statistics for Super Smash Bros. Melee.

Currently supports:

* hitstun
* hitlag
* staled <-> unstaled damage
* shield damage
* shieldstun
* shield pushback for attacker and defender
* jump arc
* knockback/knockback travel (and associated helper functions such as modifying trajectory by DI)
* Enums for states, bitflags, characters, etc.
* Character stats

The crate is currently designed around being easily extensible to FFI, but eventually there might be a
dedicated version for rust with better ergonomics/type safety.
