# Release History

## 0.3.0

---

* Fixed several bitflag display bugs
* Completed character state enum (covers all possible charater states)
* Completed item enum (covers all possible items)
* Added missing damaged states `FLY_REFLECT_CEIL` and `FLY_REFLECT_WALL` to `is_damaged()`
* Added macro `mf!() that converts melee frame indexes to array indexes
* Added slightly more ergonomic handling for turnip types and missile types
* `LCancel` enum renamed to `LCancelState`
* updated deps

## 0.2.0

---

* Restructured exports
* Added prelude module
* Added costume enum
* Added module-level docstrings
* Added utility functions and macros for Velocity and Position structs

## 0.1.0

---

Initial