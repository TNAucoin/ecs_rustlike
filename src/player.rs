use super::{xy_idx, Player, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        //get potential location index on the map
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        // only move if destination is not the location of wall tile
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
