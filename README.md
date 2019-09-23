# Hierarchical Legion Transform

This is a reference implementation of a hierarchical transform system,
implemented in [Legion](https://github.com/TomGillen/legion). It is based
heavily on the [Amethyst Core
implementation](https://github.com/amethyst/amethyst/tree/master/amethyst_core/src/transform)
with the actual `Transform` struct being identical.

See: https://community.amethyst.rs/t/legion-ecs-discussion/965/59

## How It Works

Much like the `specs` based Amethyst implementation, a `Transform` is a struct
containing an isometry and a scale, along with a global matrix that is updated
by the `TransformSystem`. A `Parent` is simply a struct containing nothing but
an `Entity` which is a handle to the parent Entity. In Legion `Parent` is stored
as a `Tag` on the Entity.

The update systems works in two passes: it first generates a forest of transform
hierarchies that need to be recomputed, then processes the forest in parallel.
Each tree must be processed from top to bottom (either breadth or depth first)
as the parent global transform affects all of it's descendants.

## Todo:

- [x] Detect and handle changes to `Transform` values
- [x] Detect and handle changes to `Transform` hierarchy layout.
- [x] Build a forest of (independent) transform hierarchies that need to be
      updated.
- [x] Re-compute `global_matrix` for `Transform` with and without parents.
- [ ] Add parallel support to computation (currently runtime throwing because of
      multiple mutable borrows).
- [ ] Consider how to handle Scaling (and double check that it's done
      correctly).
- [x] Unit test re-parenting and un-parenting.
