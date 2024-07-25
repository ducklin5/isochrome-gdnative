use std::collections::HashMap;

use gdnative::api::{PhysicsBody2D, TileMap};
use gdnative::prelude::*;

use super::floor_agent::FloorAgent;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::build)]
pub struct Floor {
    is_enabled: bool,
    layer_cache: HashMap<String, i64>,
    mask_cache: HashMap<String, i64>,
    agents: Vec<String>,
}

#[methods]
impl Floor {
    fn new(_owner: &Node2D) -> Self {
        Floor {
            is_enabled: true,
            layer_cache: HashMap::new(),
            mask_cache: HashMap::new(),
            agents: Vec::new(),
        }
    }

    fn build(_builder: &ClassBuilder<Self>) {}

    #[export]
    fn _ready(&mut self, _owner: TRef<Node2D>) {
        let owner = _owner.upcast::<Node>();
        self.cache_collision(_owner, owner, None);
    }

    fn cache_collision(
        &mut self,
        _owner: TRef<Node2D>,
        node: TRef<Node>,
        only_cache_new: Option<bool>,
    ) {
        let only_cache_new: bool = if let Some(value) = only_cache_new {
            value
        } else {
            true
        };

        let path_str = _owner.get_path_to(node).to_string();
        if !only_cache_new || !self.layer_cache.contains_key(&path_str) {
            let mut collidable = false;
            let mut collision_layer = 0;
            let mut collision_mask = 0;
            if let Some(node_pb2d) = node.cast::<PhysicsBody2D>() {
                collision_layer = node_pb2d.collision_layer();
                collision_mask = node_pb2d.collision_mask();
                collidable = true;
            } else if let Some(node_tmap) = node.cast::<TileMap>() {
                collision_layer = node_tmap.collision_layer();
                collision_mask = node_tmap.collision_mask();
                collidable = true;
            }
            if collidable {
                self.layer_cache.insert(path_str.clone(), collision_layer);
                self.mask_cache.insert(path_str, collision_mask);
            }
        }
        if let Some(node_fagent) = node.cast_instance::<FloorAgent>() {
            node_fagent
                .map(|_, fagent_owner: TRef<Node>| {
                    let path_str = _owner.get_path_to(fagent_owner).to_string();
                    if !self.agents.contains(&path_str) {
                        fagent_owner
                            .connect(
                                "collision_changed",
                                _owner,
                                "_on_agent_collision_changed",
                                VariantArray::new_shared(),
                                0,
                            )
                            .unwrap();
                        self.agents.push(path_str);
                    }
                })
                .unwrap();
        }

        for child in node.get_children().into_iter() {
            let child_ref = unsafe {
                child
                    .try_to_object::<Node>()
                    .expect("Failed to convert node variant to node_ref")
                    .assume_safe()
            };
            self.cache_collision(_owner, child_ref, None);
        }
    }

    #[export]
    fn enable_collision(&mut self, _owner: TRef<Node2D>) {
        self.is_enabled = true;
        let owner = _owner.upcast::<Node>();
        self._recursive_enable_collision(_owner, owner);
    }

    fn _recursive_enable_collision(
        &mut self,
        _owner: TRef<Node2D>,
        node: TRef<Node>,
    ) {
        if let Some(node_pb2d) = node.cast::<PhysicsBody2D>() {
            let path_str = _owner.get_path_to(node_pb2d).to_string();
            let collision_layer = self.layer_cache[&path_str];
            let collision_mask = self.mask_cache[&path_str];

            node_pb2d.set_collision_layer(collision_layer);
            node_pb2d.set_collision_mask(collision_mask);
        } else if let Some(node_tmap) = node.cast::<TileMap>() {
            let path_str = _owner.get_path_to(node_tmap).to_string();
            let collision_layer = self.layer_cache[&path_str];
            let collision_mask = self.mask_cache[&path_str];

            node_tmap.set_collision_layer(collision_layer);
            node_tmap.set_collision_mask(collision_mask);
        }

        for child in node.get_children().into_iter() {
            let child_ref = unsafe {
                child
                    .try_to_object::<Node>()
                    .expect("Failed to convert node variant to node_ref")
                    .assume_safe()
            };

            self._recursive_enable_collision(_owner, child_ref);
        }
    }

    #[export]
    fn disable_collision(&mut self, _owner: TRef<Node2D>) {
        let owner = _owner.upcast::<Node>();
        self.cache_collision(_owner, owner, Some(!self.is_enabled));
        self.is_enabled = false;
        self._recursive_disable_collision(_owner, owner);
    }

    fn _recursive_disable_collision(
        &mut self,
        _owner: TRef<Node2D>,
        node: TRef<Node>,
    ) {

        if let Some(node_pb2d) = node.cast::<PhysicsBody2D>() {
            node_pb2d.set_collision_layer(0);
            node_pb2d.set_collision_mask(0);
        } else if let Some(node_tmap) = node.cast::<TileMap>() {
            node_tmap.set_collision_layer(0);
            node_tmap.set_collision_mask(0);
        }

        for child in node.get_children().into_iter() {
            let child_ref = unsafe {
                child
                    .try_to_object::<Node>()
                    .expect("Failed to convert node variant to node_ref")
                    .assume_safe()
            };
            self._recursive_disable_collision(_owner, child_ref);
        }
    }

    #[export]
    fn _on_agent_collision_changed(
        &mut self,
        _owner: TRef<Node2D>,
        node: Variant,
    ) {
        let node_ref = unsafe {
            node.try_to_object::<Node>()
                .expect("Failed to convert node variant to node_ref")
                .assume_safe()
        };

        self.cache_collision(_owner, node_ref, Some(false));
        if !self.is_enabled {
            self._recursive_disable_collision(_owner, node_ref);
        }
    }

    #[export]
    fn get_spawn(&mut self, _owner: TRef<Node2D>) -> Ref<Node> {
        _owner.as_ref().get_node("main/spawn").unwrap()
    }
    #[export]
    fn get_class(&self, _owner: &Node2D) -> Variant {
        return Variant::from_str("Floor");
    }
}
