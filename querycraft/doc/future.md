# Future of QueryCraft

This doc aims at describing how the querycraft should be used in the near future.
All these functionalities are being developed actively, but may not work right now.

## SQL based Analysis

The goal of QueryCraft is to provide sql-based analysis based on SQLite.
The idea is to allow reversers to write SQLite queries to fetch the information about target binary.
So, how can this be used?

### Load QueryCraft

Before anything can be done in QueryCraft, we first need to load the QueryCraft.
QueryCraft is intended to be used by attaching it to original sqlite instance.
One can do this either programmatically or directly in sqlite shell.
All the other contents will assume a sqlite shell is used (so we do not need to care individual programming language support).

The QueryCraft is a `.so` or `.dll` library.
The loading can be done in sqlite shell by:

```SQL
.load ./libQueryCraft.so
```

If success, no other error messages should be thrown.

### Read the Binary

The next thing of using the QueryCraft is to read the binary into QueryCraft.
This can be done by calling the "qc_read_bin" function.

```SQL
> SELECT qc_read_bin("/path/to/bianry");
"x9187f"
```

If success, a non-empty string will be returned or else an empty string is returned.

A possible failure is that a table called "qc_binary" is already presented, but not follow the schema we used, i.e, a custom table called "qc_binary" is set already.

Currently, custom table name is not supported.

In short, this API just load the binary then insert a row into a table called `qc_binary`.

The `qc_binary` table is then used in later APIs.

The string returned is called a "session id".
It can be used to distinguish the binary.

### Information About The Binary

The information can be fetched from a table called `qc_binary`.

```SQL
> SELECT * FROM qc_binary;
```

### Load the Binary

The next thing we could do is to "load" the binary.

This works like operating system's loading as it splits the binary then add the info about the mappings.

This is done by calling a function called `qc_load`.

```SQL
> SELECT qc_load("sess_id");
```

After this, the segements that should be loaded can be found in `qc_loaded` table.

In that table, we will see:

- id: the session id of the binary
- begin_addr: starting addr of this segment
- end_addr: end addr of this segment
- data: the blob data at this segment

### In Place Disassemble

This is a helper API when one wants to disassemble several bytes and use the result as if it is a table.

```SQL
> SELECT * FROM qc_disasm(x'123456');
```

And for IR (pcode) we can use `qc_disasm_pcode`:

```SQL
> SELECT * FROM qc_disasm_pcode(x'123456');
```

### Algorithms

Since we use the SQLite as a storage engine, all algorithms can have three possible kinds:

- `transform`: the algorithm that is a transformation on table without affecting its columns. Example: dce (Dead Code Elimination).
This kind of algorithm can be used to "optimize" a table. Only inserts or deletes are used in this kind of algorithm.
- `annotate`: the algorithm that is an annotation to the previous table. Example: variable analysis.
In variable analysis, each instruction will be annotated to see if an instruction adds a new variable.
If it does, what the name should be of that variable and what is the space and addr of its pcode.
In this case, a new table will be created and in which the relation will be recorded to the previous table.
Or, a previous existing table of the same shape is specified allowed to use.
Then we can just insert rows to that table instead of setup a new one.
- `onetime`: one time analysis algorithm. Example: disassemble.
Actually it may not be called "algorithm", it is more of a use case that when needed, user calls the algorithm only once and use the result only once.
The `qc_disasm` and `qc_disasm_pcode` are examples of this kind.

TODO: add more real algorithms