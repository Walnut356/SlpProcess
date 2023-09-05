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

The crate is currently designed around being easily extensible to FFI, but eventually there will be a dedicated version with better ergonomics for rust, alongside FFI libraries for C++ and python.
