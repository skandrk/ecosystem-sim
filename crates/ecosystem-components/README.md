# Ecosystem Components Directory Structure

```
ecosystem-components/src/
├── lib.rs                    # Main exports, prelude, version info
├── components/               # ECS component definitions
│   ├── mod.rs               # Component module exports
│   ├── physical.rs          # Position, Health, Energy, Collision
│   ├── behavioral.rs        # AI, Movement components
│   └── markers.rs           # Organism, type markers
├── config/                   # Configuration system
│   ├── mod.rs               # Config module exports
│   ├── organism_config.rs   # Sealed trait system
│   └── presets.rs           # BlueConfig, RedConfig, PlantConfig, etc.
├── organisms/                # Organism types and factory
│   ├── mod.rs               # Organism module exports
│   ├── organism_type.rs     # OrganismType enum definition
│   └── factory.rs           # Organism spawning logic
└── utils/                    # Data structure utilities
    ├── mod.rs               # Utils module exports
    ├── spatial.rs           # Spatial math helpers
    └── validation.rs        # Input validation utilities
```

## Module Descriptions

### `lib.rs`

Main library entry point containing exports, prelude definitions, and version information.

### `components/`

Contains Entity Component System (ECS) component definitions:

- **`physical.rs`** - Physical world components (Position, Health, Energy, Collision)
- **`behavioral.rs`** - Behavior-related components (AI, Movement)
- **`markers.rs`** - Marker components for organism identification and typing

### `config/`

Configuration system for the ecosystem:

- **`organism_config.rs`** - Sealed trait system for organism configuration
- **`presets.rs`** - Predefined configurations (BlueConfig, RedConfig, PlantConfig, etc.)

### `organisms/`

Organism type definitions and creation logic:

- **`organism_type.rs`** - OrganismType enumeration
- **`factory.rs`** - Logic for spawning and creating organisms

### `utils/`

Utility modules for data structures and common operations:

- **`spatial.rs`** - Spatial mathematics and geometry helpers
- **`validation.rs`** - Input validation utilities
