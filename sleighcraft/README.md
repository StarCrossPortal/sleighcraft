# Sleighcraft

## Dev

The code are splited into:

- `src/cpp`: the CPP written original sleigh engine, extracted from ghidra's decompiler code
- `src/sleigh`: the sleigh specifications, extracted from ghidra's processor code
- `src/sla`: compiled sleigh specifications, are used by providing out-of-the-box decoding features

Since the `src/sla` are compiled, we need to manually compile them using the `sleighc` sleigh compiler.

To do this, go to the `src/cpp`, and follow the instruction there (TL;DR version: compile the `sleighc` from those cpp files then generate the `sla`s).

So, the rule of thumb is: everytime you modified any files in `src/sleigh`, do compile the `src/sla` to the updated version.

We actually have two possible solutions considering this problem, one is to build the `sleighc` then run it against the `sleigh` specs to generate the updated version,
but this will require the user of the library always build a `sleighc` in their machine then run it against the specs which can be a little bit time consuming.

The bad situation of current choice is that there can be a little bit maintainance burden to keep the `src/sla` and `src/sleigh` updated the same time, but for now it is acceptable.