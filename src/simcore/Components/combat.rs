use bevy::prelude::*;

enum FactionEnum {
    Red,
    Blue,
    Yellow,
}

#[derive(Component)]
pub struct Health {pub value: u16,}

#[derive(Component, PartialEq)]
pub struct Faction {pub name: FactionEnum,}