# Hierarchical Legion Transform

[![Build Status][build_img]][build_lnk]

[build_img]: https://travis-ci.org/AThilenius/legion_transform.svg?branch=master
[build_lnk]: https://travis-ci.org/AThilenius/legion_transform

A hierarchical space transform system, implemented using [Legion
ECS](https://github.com/TomGillen/legion). The implementation is based heavily
on the new Unity ECS Transformation layout.

Todo

- [ ] Hierarchy maintenance
  - [x] Remove changed `Parent` from `Children` list of the previous parent.
  - [x] Add changed `Parent` to `Children` list of the new parent.
  - [x] Update `PreviousParent` to the new Parent.
  - [x] Handle Entities with removed `Parent` components.
  - [x] Handle Entities with `Children` but without `LocalToWorld` (move their
        children to non-hierarchical).
  - [ ] Handle deleted Legion Entities (requires
        [Legion #13](https://github.com/TomGillen/legion/issues/13))
- [x] Local to world and parent transformation
  - [x] Handle homogeneous `Matrix4<f32>` calculation for combinations of:
    - [x] Translation
    - [x] Rotation
    - [x] Scale
    - [x] NonUniformScale
  - [x] Handle change detection and only recompute `LocalToWorld` when needed.
  - [x] Recompute `LocalToParent` each run, always.
- [ ] Transform hierarchy propagation
  - [x] Collect roots of the hierarchy forest
  - [x] Recursively re-compute `LocalToWorld` from the `Parent`'s `LocalToWorld`
        and the `LocalToParent` of each child.
  - [ ] Find an elegant solution to multi-threading this.
  - [ ] Compute all changes and flush them to a `CommandBuffer` rather than
        direct mutation of components.

## Blockers

- Legion has no ability to detect deleted entities or components.
  [GitHub Issue #13](https://github.com/TomGillen/legion/issues/13)
