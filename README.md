<img
    src="assets/logo-wide.svg"
    alt="flux logo" class="flux-logo">

`flux` is a refinement type checker for Rust.

[gif]

Here are some verification samples
You can find more in the tests/pos.

# Online Demo

You can try `flux` [online at this site](https://flux.programming.systems)

annotate with reflect. look at AST then copy that and map it into flux IR. spec_collector traverses entire codebase and looks at every attribute for every item and makes a mapping item-> atribute. check if reflect attribute is there, then try to map it into a flux spec. 
reflect structs also with refine_by


# Overview

For an overview, take a look at the [`flux` website](https://flux-rs.github.io).

# Docs

Documentation, including installation and usage guides can be found on the
[website](https://flux-rs.github.io/flux).

# Flux architecture how it works

## Steps (and links to papers about them)

1. Flux surface
2. MIR
3. Horn
4. liquid + SMT

# Flux vs other tools

Automatically infers loop invariant of loops. 

# Resources

- link to flux paper
- presentations about flux


Flux is only keeping track of the type, not exact values.