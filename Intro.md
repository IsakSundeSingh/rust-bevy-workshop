---
marp: true
theme: gaia
transition: melt
---

<!-- _class: lead -->

![bg fit](rust-logo.png)

<!-- _color: orange -->
<!-- _footer: By Isak  -->

# Rust

## Game development with Bevy

---

<!-- paginate: true -->

# Agenda

<style scoped>
section {
    font-size: 1.9rem;
}
</style>

- 12:35: Intro til ECS (25 min)
- 13:00: Intro til Bevy (30 min)
- 13:30 - 13:40: Pause (10 min)
- Snacks underveis
- 13:40: Gruppearbeid (10 min)
- 13:50: Oppgaver workshop (40 min)
- 14:30-14:40: Pause (10 min)
- 14:40: Videre workshopping (25 min)
- 15:05: Sync med ny gruppearbeid/fremvisning? (15 min)
- 15:20 - 16:00: Videre jobbing (40 min)
- 16:00-16:30: oppsummering og presentasjoner

---

<!-- _footer: ECS presentation content inspired/borrowed from Peder Voldnes Langdal  -->

# ECS

- Entity Component System
- Program architecture common in game development
- Suited for handling large amounts of data
- More cache-efficient
- Designed around composition over inheritance
- More decoupled than OOP-designed architectures, easier to maintain as projects grow in size

---

# ECS

- Entity: A general-purpose object. Usually only a unique id. Used for _coarse_ game objects, e.g. characters, background objects, particles, etc.
- Component: Characterizes an entity to have an aspect, where the component holds necessary data to model that aspect. E.g. position, image, race (human, animal, etc.), etc.
- System: A process that acts on all entities which have the desired components. E.g. `draw_player` acts on entities with components `Player`, `Position` and `Image`/`Sprite`

---

# ECS

![bg fit 80%](./presentation-images/ecs.png)

---

# Why not inheritance?

- In classical game designs, everything is represented using inheritance
- I

![bg right 80%](./presentation-images/inheritance-based-arch.png)

---

# How do you handle new requirements?

![bg fit 70%](./presentation-images/inheritance-based-arch.png)
![bg fit 70%](./presentation-images/inheritance-based-arch-2.png)

<!-- _footer: Images source https://medium.com/ingeniouslysimple/entities-components-and-systems-89c31464240d -->

---

# ECS vs OOP

![bg fit 85%](./presentation-images/inheritance-based-arch.png)
![bg fit 85%](./presentation-images/ecs-example.png)

---

# How is an entity stored?

![bg right fit](./presentation-images/entities-layout.png)

```rust
// Instead of a structure holding all data
struct Player {
  position: Vec2,
  health: u32
}
// and a vector of all players
let players: Vec<Player> = ...;

// Just store it the other way around
struct Position(Vec2);
struct Health(u32);
struct Players {
  positions: Vec<Position>,
  healths: Vec<Health>
}
// First player is only an index into Players
let player1 = 0;
```

---

# AoS vs SoA

<style scoped>
section {
    font-size: 1.7rem;
}
</style>

- Code tends to use similar data at close timepoints
- Array of structures:
  - A structure holds all its data with multiple fields: `struct Player { pos: Pos, health: Health }`
  - You have an array of those structures: `Vec<Player>`
  - E.g. moving all positions means getting memory from a spread-out area of RAM, since positions are interleaved between health objects, leading to bad cache usage
- Structure of arrays:
  - A structure holds fields with arrays of data: `struct Players { positions: Vec<Pos>, healths: Vec<Health> }`
  - E.g. Moving positions now has good cache locality, since more positions will be in cache when using those values.
