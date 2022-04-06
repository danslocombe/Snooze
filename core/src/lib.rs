#![allow(unused_parens)]

use std::time::Instant;

#[derive(Default)]
pub struct World {
    pub nodes: Vec<RopeNode>,
    pub ropes: Vec<Rope>,
    pub colliders: Vec<BoxCollider>,
    pub forces: Vec<Box<dyn Force>>,
}

pub fn dan_lerp_max_dist(p0 : Vec2, p : Vec2, k : f32, max_dist : f32) -> Vec2 {
    let new = (p0.mult(k-1.0).add(p)).mult(1.0/k);
    if (new.dist(p0) < max_dist) {
        new
    }
    else {
        p0.project_dist_towards(p, max_dist)
    }
}

impl World {
    pub fn add_node(&mut self, x: f32, y: f32) -> usize {
        self.nodes.push(RopeNode::new(x, y));
        self.nodes.len() - 1
    }

    pub fn add_rope(&mut self, from: usize, to: usize) -> usize {
        debug_assert!(from < self.nodes.len());
        debug_assert!(to < self.nodes.len());
        debug_assert!(from != to);

        self.ropes.push(Rope::new(from, to, &self));
        self.ropes.len() - 1
    }

    pub fn add_box_collider(&mut self, x : f32, y : f32, size : f32) -> usize {

        self.colliders.push(BoxCollider {
            x, y, size
        });

        self.ropes.len() - 1
    }

    pub fn add_rope_length(&mut self, from: usize, to: usize, length : f32) -> usize {
        debug_assert!(from < self.nodes.len());
        debug_assert!(to < self.nodes.len());
        debug_assert!(from != to);

        self.ropes.push(Rope::with_length(from, to, length, &self));
        self.ropes.len() - 1
    }

    pub fn get_node(&self, id: usize) -> &RopeNode {
        &self.nodes[id]
    }

    pub fn get_node_mut(&mut self, id: usize) -> &mut RopeNode {
        &mut self.nodes[id]
    }

    pub fn get_rope(&self, id: usize) -> &Rope {
        &self.ropes[id]
    }

    pub fn get_rope_mut(&mut self, id: usize) -> &mut Rope {
        &mut self.ropes[id]
    }

    pub fn set_node_pos_respect_colliders(&mut self, id : usize, mut p : Vec2)  {
        let max_dist = 4.0;
        {
            let node = self.get_node(id);
            p = dan_lerp_max_dist(node.pos, p, 2.0, max_dist);
        }
        for collider in &self.colliders {
            if collider.inside(p) {
                let new_p = collider.push_out(&self.colliders, p);
                if (new_p.dist(p) > max_dist) {
                    return;
                }

                p = new_p;
                break;
            }
        }

        let node = self.get_node_mut(id);
        node.pos = p;
    }

    pub fn get_tension(&self) -> f32 {
        let mut sum = 0.;
        for rope in &self.ropes {
            if (!rope.broken)
            {
                let actual_len = self.nodes[rope.from].pos.dist(self.nodes[rope.to].pos);
                let norm_len = actual_len / rope.length;
                sum += norm_len * norm_len;
            }
        }

        sum / self.ropes.len() as f32
    }

    // Done here due to borrow pain
    fn tick_rope(&mut self, rope_id: usize) {
        let rope = self.ropes[rope_id];

        if (rope.broken) {
            return;
        }

        let from_0 = self.nodes[rope.from].clone();
        let to_0 = self.nodes[rope.to].clone();
        //let centre = from_0.pos.add(to_0.pos).mult(0.5);
        let centre = from_0.pos.add(to_0.pos.sub(from_0.pos).mult(0.5));

        // TODO trying to get ropes to break?
        ////let dist = from_0.pos.sub(to_0.pos).mag();
        ////if (dist > rope.length * 1.5) {
        ////  // Break!
        ////  self.ropes[rope_id].broken = true;
        ////  return;
        ////}

        let half_len = rope.length / 2.0;

        match (from_0.node_type, to_0.node_type) {
            (NodeType::Fixed, NodeType::Fixed) => {
                // Nothing to do, both ends fixed
                return;
            }
            (NodeType::Fixed, NodeType::Free) => {
                self.nodes[rope.to].pos = centre.project_dist_towards(to_0.pos, half_len);
            }
            (NodeType::Free, NodeType::Fixed) => {
                self.nodes[rope.from].pos = centre.project_dist_towards(from_0.pos, half_len);
            }
            _ => {
                //self.nodes[rope.from].pos = centre.project_dist_towards(from_0.pos, half_len);
                //self.nodes[rope.to].pos = centre.project_dist_towards(to_0.pos, half_len);
                let k = 5.;
                let max_dist = 2.;
                self.nodes[rope.from].pos = dan_lerp_max_dist(from_0.pos, centre.project_dist_towards(from_0.pos, half_len), k, max_dist);
                self.nodes[rope.to].pos = dan_lerp_max_dist(to_0.pos, centre.project_dist_towards(to_0.pos, half_len), k, max_dist);
            }
        }
    }

    pub fn tick(&mut self, dt_norm: f32) {
        for node in &mut self.nodes {
            node.tick(&self.forces, dt_norm);
        }

        const SIM_ITERS: usize = 8;
        for _ in 0..SIM_ITERS {

            for rid in 0..self.ropes.len() {
                self.tick_rope(rid);
            }

            for node in &mut self.nodes {
                for collider in &self.colliders {
                    if collider.inside(node.pos) {
                        node.pos = collider.push_out(&self.colliders, node.pos);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    Fixed,
    Free,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::new(0., 0.)
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn mult(&self, k: f32) -> Self {
        Self::new(self.x * k, self.y * k)
    }

    pub fn dist(&self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn mag2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn mag(&self) -> f32 {
        self.mag2().sqrt()
    }

    pub fn norm(&self) -> Self {
        self.mult(1.0 / self.mag())
    }

    pub fn project_dist_towards(&self, other: Self, dist: f32) -> Self {
        let diff = other.sub(self.clone());

        let diff_mag = diff.mag();
        let diff_with_dist = diff.mult(dist / diff_mag);

        self.add(diff_with_dist)
    }
}

#[derive(Debug, Clone)]
pub struct RopeNode {
    pub node_type: NodeType,
    pub pos: Vec2,
    prev_pos: Vec2,
}

impl RopeNode {
    fn new(x: f32, y: f32) -> Self {
        Self {
            node_type: NodeType::Free,
            pos: Vec2::new(x, y),
            prev_pos: Vec2::new(x, y),
        }
    }

    fn tick(&mut self, forces: &[Box<dyn Force>], dt_norm: f32) {
        if (self.node_type == NodeType::Fixed) {
            return;
        }

        //let mut vel = self.pos.sub(self.prev_pos).mult(dt_norm);
        let mut vel = self.pos.sub(self.prev_pos);

        const FRIC: f32 = 0.91;
        vel = vel.mult(FRIC);

        for force in forces {
            vel = vel.add(force.get_force(self.pos));
        }

        self.prev_pos = self.pos;
        //self.pos = self.pos.add(vel.mult(dt_norm));
        self.pos = self.pos.add(vel);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rope {
    pub from: usize,
    pub to: usize,
    length: f32,
    pub broken: bool,
}

impl Rope {
    fn with_length(from: usize, to: usize, length : f32, world: &World) -> Self {
        Self {
            from,
            to,
            length,
            broken: false,
        }
    }

    fn new(from: usize, to: usize, world: &World) -> Self {
        let length = world.get_node(from).pos.dist(world.get_node(to).pos);
        Self::with_length(from, to, length, world)
    }
}

pub struct BoxCollider {
    x : f32,
    y : f32,
    size : f32,
}

fn inside_any(colliders : &[BoxCollider], p : Vec2) -> bool {
    for collider in colliders {
        if (collider.inside(p)) {
            return true;
        }
    }

    false
}

impl BoxCollider {
    pub fn inside(&self, p : Vec2) -> bool {
        p.x > self.x && p.x < self.x + self.size
        && p.y > self.y && p.y < self.y + self.size
    }

    pub fn push_out(&self, others : &[BoxCollider], p : Vec2) -> Vec2 {
        let c_b = self.y - self.x;
        let in_half_b = p.y > p.x + c_b;
        let c_a = self.y + self.size + self.x;
        let in_half_a = p.y > -p.x + c_a;

        if (in_half_a && in_half_b) {
            let tp = Vec2::new(p.x, self.y + self.size);
            if (!inside_any(others, tp)) {
                return tp;
            }
        }

        if (in_half_b) {
            let tp = Vec2::new(self.x, p.y);
            if (!inside_any(others, tp)) {
                return tp;
            }
        }

        if (in_half_a || in_half_b) {
            let tp = Vec2::new(self.x + self.size, p.y);
            if (!inside_any(others, tp)) {
                return tp;
            }
        }

        Vec2::new(p.x, self.y)

        /*
        match ((in_half_a, in_half_b)) {
            (true, true) => Vec2::new(p.x, self.y + self.size)
            (false, true) => Vec2::new(self.x, p.y),
            (true, false) => Vec2::new(self.x + self.size, p.y),
            (false, false) => Vec2::new(p.x, self.y),
        }
        */
    }
}

pub trait Force {
    fn get_force(&self, rope_node_pos: Vec2) -> Vec2;
}

pub struct ConstantForce {
    pub force: Vec2,
}

impl Force for ConstantForce {
    fn get_force(&self, _: Vec2) -> Vec2 {
        self.force
    }
}

pub struct InverseSquareForce {
    pub strength: f32,
    pub pos: Vec2,
}

impl Force for InverseSquareForce {
    fn get_force(&self, rope_pos: Vec2) -> Vec2 {
        let delta = self.pos.sub(rope_pos);
        let d2 = delta.mag2();
        if (d2 == 0.0) {
            return Vec2::default();
        }

        let d = delta.mag();
        let mag = self.strength / d2;
        delta.mult(mag / d)
    }
}


pub struct GlobalState {
    pub t: usize,
    pub world: World,
    pub last_tick: Instant,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            t: 0,
            world: World::default(),
            last_tick: Instant::now(),
        }
    }
}