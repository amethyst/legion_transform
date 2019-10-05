# Hierarchical Legion Transform

> Not ready for use, only partially complete and still being re-written from the
> ground up every day or two.

A Unity-ECS inspired implementation of a space-transform system, implemented
using [Legion ECS](https://github.com/TomGillen/legion). No external indexes or
caches are used, 100% of hierarchy data is stored as components within the ECS
itself. This makes life easier everywhere else ex. serialization/streaming
and world merging.

## How It Works

The source-of-truth for a hierarchy is stored in `Parent` components. These
components are then used to update `Children` components as the first step in
the transform system. This has the implication that **the `Children` component
is only guaranteed correct right after the transform system is run**. Much like
Unity, the `Children` component is an enum to allow for fixed size,
stack-allocated data when there are less than 8 children, and heap-allocated
data (with a necessary deref) when there are more than 8 children. This logic is
wrapped in a `DynamicArray`, defined as:

```rust
pub enum DynamicArray8<T: Copy + PartialEq> {
    // Fixed size array of up to 8 elements, along with a count.
    Fixed([T; 8], u8),

    // A heap-allocated array of any number of elements.
    Dynamic(Vec<T>),
}
```

The transform system itself does the following (checkboxes for what is already
implemented):

- [ ] Iterate all changed Transforms (which can be mixes and matched) to
      generate the `LocalToGlobal` for all non-parented entities.
- [ ] Iterate all changed Transforms to generate the `LocalToParent` for all
      parented-Entities. (This matrix is independent of the parent itself).
- [x] Update all `Children` entities, including removing re-parented or
      un-parented Entities. This is done efficiently via a `previous_parent`
      field in the `Parent` component. At the end of this step all `Children`
      are guaranteed-correct.
- [x] Create a forest of hierarchy changes. It is guaranteed that any Entity
      will appear at most once in at most one tree within the forest.
- [x] Use the above forest to re-compute the 'depth' of the transform. This
      depth is set as a `DepthTag` on the Entity itself, which implicitly sorts
      hierarchy entities within a Legion world.
- [ ] Use the above `DepthTag` to iterate through entities with any several
      different combinations of transforms, and generate a `LocalToWorld`
      transform using only the parent's `LocalToWorld` and the entity's
      `LocalToParent`.

## Blockers

_These are causing unit tests to fail, but they are manually confirmed-working
if not for the Legion issues._

- Legion has no ability to detect deleted entities or components.
  [GitHub Issue #13](https://github.com/TomGillen/legion/issues/13)
- Legion has a bug where Tags are not being correctly mutated.
  [GitHub Issue #12](https://github.com/TomGillen/legion/issues/12)
