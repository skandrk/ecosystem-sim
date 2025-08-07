//! Physical components - position, health, energy, collision

use bevy_ecs::prelude::*;
use bevy_math::Vec2;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// Entity position in 2D world space
#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn from_vec2(vec: Vec2) -> Self {
        Self::new(vec.x, vec.y)
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn direction_to(&self, other: &Position) -> Vec2 {
        let distance = self.distance_to(other);
        if distance == 0.0 {
            Vec2::ZERO
        } else {
            Vec2::new((other.x - self.x) / distance, (other.y - self.y) / distance)
        }
    }

    pub fn clamp_to_bounds(&mut self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        self.x = self.x.clamp(min_x, max_x);
        self.y = self.y.clamp(min_y, max_y);
    }
}

/// Entity health with damage and healing
#[derive(Component, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            current: max_health.max(0.0),
            max: max_health.max(0.0),
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn ratio(&self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            (self.current / self.max).clamp(0.0, 1.0)
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    pub fn is_low(&self, threshold: f32) -> bool {
        self.ratio() < threshold.clamp(0.0, 1.0)
    }

    pub fn take_damage(&mut self, damage: f32) {
        if damage > 0.0 {
            self.current = (self.current - damage).max(0.0);
        }
    }

    pub fn heal(&mut self, amount: f32) {
        if amount > 0.0 {
            self.current = (self.current + amount).min(self.max);
        }
    }

    pub fn set_current(&mut self, health: f32) {
        self.current = health.clamp(0.0, self.max);
    }

    pub fn set_max(&mut self, max_health: f32) {
        let new_max = max_health.max(0.0);
        self.max = new_max;
        if self.current > new_max {
            self.current = new_max;
        }
    }
}

/// Entity energy with consumption and regeneration
#[derive(Component, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Energy {
    current: f32,
    max: f32,
    movement_cost: f32,
    regen_rate: f32,
}

impl Energy {
    pub fn new(max_energy: f32, movement_cost: f32, regen_rate: f32) -> Self {
        Self {
            current: max_energy.max(0.0),
            max: max_energy.max(0.0),
            movement_cost: movement_cost.max(0.0),
            regen_rate: regen_rate.max(0.0),
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn movement_cost(&self) -> f32 {
        self.movement_cost
    }

    pub fn regen_rate(&self) -> f32 {
        self.regen_rate
    }

    pub fn ratio(&self) -> f32 {
        if self.max <= 0.0 {
            0.0
        } else {
            (self.current / self.max).clamp(0.0, 1.0)
        }
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0.0
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    pub fn is_low(&self, threshold: f32) -> bool {
        self.ratio() < threshold.clamp(0.0, 1.0)
    }

    pub fn can_move(&self) -> bool {
        self.current >= self.movement_cost
    }

    pub fn can_afford(&self, cost: f32) -> bool {
        self.current >= cost
    }

    pub fn consume(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn consume_movement(&mut self) -> bool {
        self.consume(self.movement_cost)
    }

    pub fn add_energy(&mut self, amount: f32) {
        if amount > 0.0 {
            self.current = (self.current + amount).min(self.max);
        }
    }

    pub fn regenerate(&mut self, delta_seconds: f32) {
        if delta_seconds > 0.0 {
            let regen_amount = self.regen_rate * delta_seconds;
            self.add_energy(regen_amount);
        }
    }

    pub fn set_current(&mut self, energy: f32) {
        self.current = energy.clamp(0.0, self.max);
    }

    pub fn set_max(&mut self, max_energy: f32) {
        let new_max = max_energy.max(0.0);
        self.max = new_max;
        if self.current > new_max {
            self.current = new_max;
        }
    }

    pub fn set_movement_cost(&mut self, cost: f32) {
        self.movement_cost = cost.max(0.0);
    }

    pub fn set_regen_rate(&mut self, rate: f32) {
        self.regen_rate = rate.max(0.0);
    }
}

/// Collision detection component
#[derive(Component, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Collision {
    radius: f32,
}

impl Collision {
    pub fn new(radius: f32) -> Self {
        Self {
            radius: radius.max(0.0),
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius.max(0.0);
    }

    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }

    pub fn is_colliding_with(&self, other: &Collision, distance: f32) -> bool {
        distance <= (self.radius + other.radius)
    }

    pub fn min_distance_to(&self, other: &Collision) -> f32 {
        self.radius + other.radius
    }

    pub fn overlap_amount(&self, other: &Collision, distance: f32) -> f32 {
        let min_distance = self.min_distance_to(other);
        if distance < min_distance {
            min_distance - distance
        } else {
            0.0
        }
    }
}
