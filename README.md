# Rust game development workshop

## Getting started

Make sure you've walked through the presentation in [Intro.md](./Intro.md) before starting, as it introduces ECS and Bevy.

### Installing Rust

Install Rustup (version manager for Rust tooling) from: https://rustup.rs/

This workshop is tested and developed using Rust 1.83.

## Bevy 0.15

This workshop uses Bevy 0.15, but it is quite recently released at the time of writing. Bevy is actively developed and has frequent breaking changes.

### Pre-code

This project is a scaffolding for a Bevy project. It is set up to show how to get an image on screen and use a state. It has a debug inspector console that can be enabled/disabled using the key `d`. Try to use the inspector to learn a bit more about what the default setup does, but keep in mind that the debug inspector plugin also adds some stuff which appears in the inspector

Head to [src/main.rs](./src/main.rs) to get started.

## Workshop structure

This workshop is intended to be a guide for getting started with some game development with Rust and Bevy. I advise you to try to get through the assignments, as they will show you enough of the "rough parts" to get a working game going.

However, you are free and _encouraged_ to add new features on top of this workshop if you want to.

Each assignment will get one feature working. Features might be improved upon in a later assignment, so don't worry too much if it isn't as polished as you want. Or you could try to make it better. Your choice.

**Cooperation** with others is encouraged, please be active in asking for help or contributing.

> [!TIP]
> Assignments might have _tip_ blocks that will help you overcome an assignment.

<details>
<summary>Solution ✅</summary>
If you're stuck, check out these blocks for a _suggested_ solution. You might finish the assignment some other way, but future assignments will assume you solved an assignment in the way the suggested solution does.
</details>

## What game are we making?

> It is the day before Christmas. Santa Claus has a big day ahead, but he has been at a Christmas party with his friends, and is quite intoxicated. 🍻
>
> You are a poor elf working in the gift factory. Your job is to ensure that Santa receives all the presents he needs to make everyone happy for Christmas.
>
> Santa walks erratically around the factory floor, and you have to throw presents that land in his sack so children will get their presents for Christmas day.

### Gameplay

The game developed in this workshop is a top-down 2D shooter-type game. Elfs try to throw presents towards Santa Claus and his sack of presents. You get points for each present you hit.

---

## Game development with Bevy and Rust

### Assignment 1: Movement 🎅

Your pre-code includes code for spawning an image of a santa claus, but the santa does not move!

**Assignment:** Make Santa move to the right of the screen, with a speed of 10 px per frame

<details>
<summary>Solution ✅</summary>

</details>

---

### Bonus exercises

- Santa burps
- Beer cans for Santa to pick up
- Christmas music
- Reindeer blocking presents
- High score
- Persisted high score
