# Hierarchical Legion Transform

This is a reference implementation of a hierarchical transform system, implemented in
[Legion](https://github.com/TomGillen/legion). It is based heavily on the
[Amethyst Core
implementation](https://github.com/amethyst/amethyst/tree/master/amethyst_core/src/transform)
with the actual `Transform` struct being identical.

## How It Works

Much like the `specs` based Amethyst implementation, a `Transform` is a struct
containing an isometry and a scale, along with a global matrix that is updated
by the `TransformSystem`. A `Parent` is simply a struct containing nothing but
an `Entity` which is a handle to the parent Entity.

The update systems works in two passes: it first generates a forest of transform
hierarchies that need to be recomputed, then processes the forest in parallel.
Each tree must be processed from top to bottom (either breadth or depth first)
as the parent global transform affects all of it's descendants.
