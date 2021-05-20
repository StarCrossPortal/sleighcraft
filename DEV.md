# Development Guide

## Introduction

This is the repo for bincraft ghidra release.
The organization of the code in this repo is roughly:

- ghidra: submodule to the [forked ghidra](https://github.com/StarCrossPortal/ghidra). The branch `bincraft_ghidra` of the forked ghidra is where we put our forked version. The `master` branch of that repo is for tracking upstream changes, we should keep it follow the upstream.
- ghidracraft: our plugin. Currently only examples. But in the future, if we decide to write a custom default plugin for ghidra, it should reside in this directory.
- sleighcraft: one of the Ghidra-related tool in Rust.
- bindings: bindings to Ghidra-related tools. Currently only `sleighcraft`
- images: for README

So, the structure follows as:

- ghidra related tools reside in a new directory of its name (xxxCraft, maybe). When possible, write it in Rust and add bindings afterwards.
- the binding to that tool should reside in the `bindings` directory
- all custom default Ghidra plugin (extension) should be in the `ghidracraft` directory
- custom changes to the Ghidra itself should be PRed to the [forked version of Ghidra here](https://github.com/StarCrossPortal/ghidra)

## Procedure

### SleighCraft

As a self-contained tool (at least for now, in the future, it should relate to our forked version of ghidra), you can use normal Rust toolchain to develop it.

The `Cargo.toml` at the root dir specifies the work spaces.

If you know anything about Rust development, it should be similar to just other Rust projects.

Bindings are alike, just a normal workspace within.

The dev procedure follows normal github dev procedure.

###  GhidraCraft

The plugin of Ghidra. Currently only examples inside.

To dev it, you need a working Ghidra release, either built yourself or released version.
(However, we prefer the self-built version.)

Set the environment `GHIDRA_INSTALL_DIR` to the released ghidra directory, and do `gradle buildExtension`.

A zip file within `dist` file is the extension you build.

The debugging experience is not yet completed. Currently you might need to start ghidra with `ghidraDev` (within ghidra release, `support/`) and attach java debugger to it.

The dev procedure follows normal github dev procedure.

### Upstream Ghidra

You should refer to [official ghidra dev doc](https://github.com/NationalSecurityAgency/ghidra/blob/master/DevGuide.md) to get a glance of how things should be doing.

However, we don't use Eclipse that much.

VSCode or IntelliJ IDEA is preferred. But to launch and debug it, you might need settings like:

(This is mine, in VSCode `launch.json`)

```json
        {
            "type": "java",
            "name": "Launch Ghidra",
            "request": "launch",
            "projectName": "Framework Utility",
            "mainClass": "ghidra.GhidraLauncher",
            "vmArgs": "-Djava.system.class.loader=ghidra.GhidraClassLoader -DbinaryPath=build/classes/java/main:build/resources/main/:bin/default/::src/main/resources/",
            "classPaths": ["${workspaceFolder}\\Ghidra\\Framework\\Utility\\build\\classes\\java\\main"],
            "args": "ghidra.GhidraRun"
        },
```

IDEA setting is almost the same, you should set it according to this one.

And since this repo (bincraft) is just a shelter repo for our forked version of ghidra, the dev procedure can be a little bit different.

Decide if you are developing a bincraft-only feature or upstream-compatible feature (or bug fix).

- bincraft-only: feature (or bug fix). Fork [this repo](https://github.com/StarCrossPortal/ghidra). Be sure to develop your code based on `bincraft_ghidra` branch.
- upstream-compatible: fork either original official [ghidra](https://github.com/StarCrossPortal/ghidra) or [our version](https://github.com/StarCrossPortal/ghidra) but be sure to based on `master` branch (which should always track upstream `master`).

Now your dev is done. Also, based on your type of feature or fix, you have two choices.

- bincraft-only: fire a pull request directly to [our repo](https://github.com/StarCrossPortal/ghidra), to `bincraft_ghidra` branch.
- upstream-compatible: first, fire a pull request to [our repo](https://github.com/StarCrossPortal/ghidra) as well. Since we may merge it quicker than official. Then, fire a PR directly to the official repo, follow normal ghidra contribute procedure.