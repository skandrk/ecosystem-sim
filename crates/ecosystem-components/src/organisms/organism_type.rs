//! Organism type definitions for the ecosystem simulation

use bevy_math::Vec2;
#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

/// Types of organisms in the ecosystem simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum OrganismType {
    /// Blue organisms (prey)
    Blue,

    /// Red organisms (predators)
    Red,

    /// Plant organisms (stationary food sources)
    Plant,
}

/// Visual properties for rendering organisms
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct SpriteStats {
    /// RGB color values (0.0 to 1.0)
    pub color: (f32, f32, f32),

    /// Sprite size (radius for circular organisms)
    pub size: f32,

    /// Shape type for rendering
    pub shape: SpriteShape,

    /// Optional texture path (for future sprite assets)
    pub texture_path: Option<&'static str>,
}

/// Shape types for organism rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub enum SpriteShape {
    Circle,
    Square,
    Triangle,
}

impl OrganismType {
    /// Get all organism types as a slice
    pub fn all() -> &'static [OrganismType] {
        &[OrganismType::Blue, OrganismType::Red, OrganismType::Plant]
    }

    /// Get display name for this organism type
    pub fn display_name(&self) -> &'static str {
        match self {
            OrganismType::Blue => "Blue Organism",
            OrganismType::Red => "Red Organism",
            OrganismType::Plant => "Plant",
        }
    }

    /// Get short name for this organism type
    pub fn short_name(&self) -> &'static str {
        match self {
            OrganismType::Blue => "Blue",
            OrganismType::Red => "Red",
            OrganismType::Plant => "Plant",
        }
    }

    /// Get visual properties for this organism type
    pub fn sprite_stats(&self) -> SpriteStats {
        match self {
            OrganismType::Blue => SpriteStats {
                color: (0.2, 0.6, 1.0), // Light blue
                size: 8.0,
                shape: SpriteShape::Circle,
                texture_path: None,
            },
            OrganismType::Red => SpriteStats {
                color: (1.0, 0.2, 0.2), // Bright red
                size: 12.0,
                shape: SpriteShape::Triangle,
                texture_path: None,
            },
            OrganismType::Plant => SpriteStats {
                color: (0.2, 0.8, 0.3), // Green
                size: 6.0,
                shape: SpriteShape::Square,
                texture_path: None,
            },
        }
    }

    /// Get just the color for this organism type
    pub fn color(&self) -> (f32, f32, f32) {
        self.sprite_stats().color
    }

    /// Get just the size for this organism type
    pub fn size(&self) -> f32 {
        self.sprite_stats().size
    }

    /// Get just the shape for this organism type
    pub fn shape(&self) -> SpriteShape {
        self.sprite_stats().shape
    }
}

impl SpriteStats {
    /// Convert color to Vec3 for Bevy
    pub fn color_vec3(&self) -> bevy_math::Vec3 {
        bevy_math::Vec3::new(self.color.0, self.color.1, self.color.2)
    }

    /// Get size as Vec2 for rectangular sprites
    pub fn size_vec2(&self) -> Vec2 {
        Vec2::new(self.size, self.size)
    }

    /// Check if this sprite uses a texture
    pub fn has_texture(&self) -> bool {
        self.texture_path.is_some()
    }
}

impl Default for OrganismType {
    fn default() -> Self {
        OrganismType::Blue
    }
}

impl Default for SpriteShape {
    fn default() -> Self {
        SpriteShape::Circle
    }
}

impl std::fmt::Display for OrganismType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organism_types() {
        let all_types = OrganismType::all();
        assert_eq!(all_types.len(), 3);
        assert!(all_types.contains(&OrganismType::Blue));
        assert!(all_types.contains(&OrganismType::Red));
        assert!(all_types.contains(&OrganismType::Plant));
    }

    #[test]
    fn test_sprite_stats() {
        let blue_stats = OrganismType::Blue.sprite_stats();
        assert_eq!(blue_stats.color, (0.2, 0.6, 1.0));
        assert_eq!(blue_stats.size, 8.0);
        assert_eq!(blue_stats.shape, SpriteShape::Circle);

        let red_stats = OrganismType::Red.sprite_stats();
        assert_eq!(red_stats.shape, SpriteShape::Triangle);
        assert!(red_stats.size > blue_stats.size); // Predators are bigger

        let plant_stats = OrganismType::Plant.sprite_stats();
        assert_eq!(plant_stats.shape, SpriteShape::Square);
    }

    #[test]
    fn test_color_helpers() {
        assert_eq!(OrganismType::Blue.color(), (0.2, 0.6, 1.0));
        assert_eq!(OrganismType::Red.size(), 12.0);
        assert_eq!(OrganismType::Plant.shape(), SpriteShape::Square);
    }

    #[test]
    fn test_sprite_stats_helpers() {
        let stats = OrganismType::Blue.sprite_stats();
        let color_vec = stats.color_vec3();
        assert_eq!(color_vec.x, 0.2);
        assert_eq!(color_vec.y, 0.6);
        assert_eq!(color_vec.z, 1.0);

        let size_vec = stats.size_vec2();
        assert_eq!(size_vec.x, 8.0);
        assert_eq!(size_vec.y, 8.0);

        assert!(!stats.has_texture()); // No textures in MVP
    }
}
