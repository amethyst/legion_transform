#![allow(dead_code)]
use crate::{components::*, ecs::prelude::*};
use rayon::prelude::*;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

/// Used to create a forest of hierarchy deltas, which is needed to correctly compute each Entities
/// new depth.
#[derive(Debug)]
struct TreeNode {
    pub entity: Entity,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(entity: Entity) -> Self {
        TreeNode {
            entity,
            children: Vec::new(),
        }
    }
}

/// Used to tag each entity with a `Parent` with a depth. This effectively sorts the hierarchy,
/// guaranteeing that parents are processed before children. And it fits with Legions model of
/// efficient parallel iteration.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DepthTag(u32);

/// Used to buffer world mutations within a context that already has a reference to the world.
enum WorldMutation {
    AddEntityToParentChildren(Entity, Entity),
    RemoveEntityFromParentChildren(Entity, Entity),
}

impl WorldMutation {
    pub fn apply(self, world: &mut World) {
        match self {
            Self::AddEntityToParentChildren(entity, parent) => {
                // Get or create `Children` component, and add `entity` to it.
                if let Some(mut parent_children) = world.get_component_mut::<Children>(parent) {
                    (*parent_children).0.push(entity);
                    return;
                }
                world.add_component(parent, Children::with(&[entity]));
            }
            Self::RemoveEntityFromParentChildren(entity, parent) => {
                if let Some(mut parent_children) = world.get_component_mut::<Children>(parent) {
                    (*parent_children).0.retain(|e| *e != entity);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct TransformSystemBundle;
impl TransformSystemBundle {
    pub fn build(&mut self, _: &mut World) -> Vec<Box<dyn Schedulable>> {
        let child_update_system = SystemBuilder::<()>::new("ChildUpdateSystem")
            // Here, we assume ALL entities always have a Parent/child
            // We garuntee this with the seperate `changed` filter on Transform, and always add them.
            .with_query(<Write<Parent>>::query().filter(changed::<Parent>()))
            .with_query(<Write<Children>>::query())
            .build(move |commands, _resource, queries| {
                let mut additions_this_frame =
                    HashMap::<Entity, SmallVec<[Entity; 8]>>::with_capacity(16);

                let (parent_changes_query, children_query) = queries;

                for (entity, mut parent) in parent_changes_query.iter_entities() {
                    if let Some(previous_parent) = parent.previous_parent {
                        // If the previous parent IS the new parent, then there is nothing to do.
                        if previous_parent == parent.entity {
                            continue;
                        }

                        if let Some((_, mut parent_children)) =
                            children_query.par_iter_chunks().find_map_any(|mut chunk| {
                                chunk
                                    .iter_entities()
                                    .find(|(entity, _)| *entity == previous_parent)
                            })
                        {
                            (*parent_children).0.retain(|e| *e != entity);
                        }
                    }

                    parent.previous_parent = Some(parent.entity);

                    if let Some((_, mut parent_children)) =
                        children_query.par_iter_chunks().find_map_any(|mut chunk| {
                            chunk
                                .iter_entities()
                                .find(|(entity, _)| *entity == parent.entity)
                        })
                    {
                        // This is the parent
                        (*parent_children).0.push(entity);
                        log::trace!("Pushing component");
                    } else {
                        // The parent doesnt have a children entity, lets add it
                        additions_this_frame
                            .entry(parent.entity)
                            .or_insert_with(Default::default)
                            .push(entity);
                    }
                }

                additions_this_frame.iter().for_each(|(k, v)| {
                    commands.add_component(*k, Children::with(v));
                });
            });

        let set_depths_system = SystemBuilder::<()>::new("SetDepthsSystem")
            .with_query(Write::<Parent>::query())
            .build(move |_commands, _resource, _queries| {
                log::trace!("Enter: SetDepthsSystem");
            });

        vec![child_update_system, set_depths_system]
    }

    pub fn set_depths_system(world: &mut World) {
        // Because re-tagging entities is expensive, we first fully build out a forest of updated
        // hierarchies before iterating through that to set DepthTags.
        let mut forest: HashMap<Entity, TreeNode> = HashMap::new();
        let mut visited: HashSet<Entity> = HashSet::new();

        // Parents there were changed from the previous system run. Collected into a vector.
        let changed_parents: Vec<_> = Read::<Parent>::query()
            .filter(changed::<Parent>())
            .iter_entities(world)
            .map(|(e, _)| e)
            .collect();

        for entity in changed_parents {
            TransformSystemBundle::explore_tree_dfs(entity, &mut forest, &mut visited, world);
        }

        let trees: Vec<_> = forest.values().collect();
        for tree_root in trees.iter() {
            let entity = tree_root.entity;

            // The starting depth of the parent of `entity`, or 0.
            let start_depth = 1 + {
                // Only entities with changed parents are in this list, so just unwrap without
                // check.
                let parent_entity = world.get_component::<Parent>(entity).unwrap().entity;
                world
                    .get_component::<Parent>(parent_entity)
                    .map(|pe_cmp| pe_cmp.depth)
                    .unwrap_or(0)
            };

            // Recursively set the tree depth (and tags).
            TransformSystemBundle::set_depths_recursive(tree_root, world, start_depth);
        }
    }

    #[inline]
    fn explore_tree_dfs(
        entity: Entity,
        forest: &mut HashMap<Entity, TreeNode>,
        visited: &mut HashSet<Entity>,
        world: &World,
    ) {
        // If the node was visited already, then continue on.
        if visited.contains(&entity) {
            return;
        }

        // Explore it DFS, which will rotate any nodes it comes across that are already roots in
        // the forest into the tree.
        let mut node = TreeNode::new(entity);
        TransformSystemBundle::explore_dfs(&mut node, forest, visited, world);

        // Add it both the forest root and mark it visited.
        forest.insert(entity, node);
        visited.insert(entity);
    }

    #[inline]
    fn explore_dfs(
        parent_node: &mut TreeNode,
        forest: &mut HashMap<Entity, TreeNode>,
        visited: &mut HashSet<Entity>,
        world: &World,
    ) {
        // Gather and iterate children.
        let parent = parent_node.entity;
        let children: Vec<_> = world
            .get_component::<Children>(parent)
            .map(|c| c.0.iter().cloned().collect())
            .unwrap_or(Vec::new());
        for child_entity in children {
            // Regardless of it the child is visited, if it's in the root of forest we need to
            // rotate the entire tree to a child of the parent node.
            if let Some(node) = forest.remove(&child_entity) {
                // Add the entire tree under the root and return.
                parent_node.children.push(node);
                return;
            }

            // This node was visited already but isn't the root of a tree then stop searching.
            if visited.contains(&child_entity) {
                return;
            }

            // Visit the child recursively.
            visited.insert(child_entity);
            let mut child_node = TreeNode::new(child_entity);
            TransformSystemBundle::explore_dfs(&mut child_node, forest, visited, world);
            parent_node.children.push(child_node);
        }
    }

    #[inline]
    fn set_depths_recursive(node: &TreeNode, world: &mut World, depth: u32) {
        let parent = node.entity;
        let original_depth = {
            let mut parent_component = world.get_component_mut::<Parent>(parent).unwrap();
            let original_depth = (*parent_component).depth;
            (*parent_component).depth = depth;
            original_depth
        };

        if original_depth != depth {
            world.add_tag(parent, DepthTag(depth));
        }

        for child in node.children.iter() {
            TransformSystemBundle::set_depths_recursive(child, world, depth + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use legion::{command::CommandBuffer, resource::Resources};
    #[test]
    fn correct_children() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut world = Universe::new().create_world();
        let mut commands = CommandBuffer::default();
        let resources = Resources::default();

        let systems = TransformSystemBundle::default().build(&mut world);

        // Add 3 entities
        let entities = world.insert(
            (),
            vec![
                (TransformSimilarity3::identity(),),
                (TransformSimilarity3::identity(),),
                (TransformSimilarity3::identity(),),
            ],
        );
        let (parent, e1, e2) = (entities[0], entities[1], entities[2]);

        // Parent `e1` and `e2` to `parent`.
        world.add_component(e1, Parent::new(parent));
        world.add_component(e2, Parent::new(parent));

        // Run the system on it
        systems[0].run(&resources, &mut world);
        systems[0].command_buffer_mut().write(&mut world);

        assert_eq!(
            world
                .get_component::<Children>(parent)
                .unwrap()
                .0
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![e1, e2]
        );

        // Parent `e1` to `e2`.
        (*world.get_component_mut::<Parent>(e1).unwrap()).entity = e2;

        // Run the system on it
        systems[0].run(&resources, &mut world);
        systems[0].command_buffer_mut().write(&mut world);

        assert_eq!(
            world
                .get_component::<Children>(parent)
                .unwrap()
                .0
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![e2]
        );

        assert_eq!(
            world
                .get_component::<Children>(e2)
                .unwrap()
                .0
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![e1]
        );

        // TODO: This wont work until Legion supports change detection for deleted entities :(
        // Remove e1
        world.delete(e1);

        // Run the system on it
        systems[0].run(&resources, &mut world);
        systems[0].command_buffer_mut().write(&mut world);

        assert_eq!(
            world
                .get_component::<Children>(parent)
                .unwrap()
                .0
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            vec![e2]
        );
    }

    #[test]
    fn depth_test() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut world = Universe::new().create_world();
        let mut commands = CommandBuffer::default();
        let resources = Resources::default();

        let systems = TransformSystemBundle::default().build(&mut world);

        // Add 3 entities
        let entities = world.insert(
            (),
            vec![
                (TransformSimilarity3::identity(),),
                (TransformSimilarity3::identity(),),
                (TransformSimilarity3::identity(),),
            ],
        );
        let (parent, e1, e2) = (entities[0], entities[1], entities[2]);

        // Parent `e1` and `e2` to `parent`.
        world.add_component(e1, Parent::new(parent));
        world.add_component(e2, Parent::new(parent));

        // Run the systems on it
        systems[0].run(&resources, &mut world);
        systems[0].command_buffer_mut().write(&mut world);
        TransformSystemBundle::set_depths_system(&mut world);

        // Both should be at a depth of 1.
        assert_eq!(*world.get_tag::<DepthTag>(e1).unwrap(), DepthTag(1));
        assert_eq!(*world.get_tag::<DepthTag>(e2).unwrap(), DepthTag(1));

        // Re-parent `e2` to `e1`.
        (*world.get_component_mut::<Parent>(e2).unwrap()).entity = e1;

        // Run the systems on it

        systems[0].run(&resources, &mut world);
        systems[0].command_buffer_mut().write(&mut world);

        TransformSystemBundle::set_depths_system(&mut world);

        // This fails because Legion is not correctly mutating Tags.
        assert_eq!(*world.get_tag::<DepthTag>(e1).unwrap(), DepthTag(1));
        assert_eq!(*world.get_tag::<DepthTag>(e2).unwrap(), DepthTag(2));
    }
}
