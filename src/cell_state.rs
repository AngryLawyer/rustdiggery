use tileset::FlipContext;
use map::{Adjacents};
use sdl2::pixels::Color;

#[derive(Clone, Copy)]
pub enum CellState {
    Empty,
    Dirt,
    Stone,
    Wall
}

pub fn get_tileset_sprite(adjacents: &[bool; 8]) -> (u32, u32, FlipContext) {
    let adjacent_tiles = (
        adjacents[0],
        adjacents[1],
        adjacents[2],
        adjacents[3],
        adjacents[4],
        adjacents[5],
        adjacents[6],
        adjacents[7],
    );
    match adjacent_tiles {
        (
            _,     true,  _,
            true,         true,
            _,     true, _
        ) => {
            (5, 0, FlipContext::FlipNone)  // Standalone
        },
        (
            _,     true,  _,
            true,         true,
            _,     false, _
        ) => {
            (5, 1, FlipContext::FlipNone)  // Top nub
        },
        (
            _,     false,  _,
            true,         true,
            _,     false, _
        ) => {
            (5, 2, FlipContext::FlipNone)  // Vertical bar
        },
        (
            _,     false,  _,
            true,         true,
            _,     true, _
        ) => {
            (5, 3, FlipContext::FlipNone)  // Bottom nub
        },
        (
            _,     true,  _,
            true,         false,
            _,     true,  _
        ) => {
            (1, 3, FlipContext::FlipNone)  // Left nub
        },
        (
            _,     true,  _,
            false,        false,
            _,     true,  _
        ) => {
            (7, 1, FlipContext::FlipNone)  // Horizontal bar
        },
        (
            _,     true,  _,
            false,        true,
            _,     true,  _
        ) => {
            (1, 3, FlipContext::FlipHorizontal)  // Right nub
        },
        (
            _,     true,  _,
            true,         false,
            _,     false, false
        ) => {
            (1, 0, FlipContext::FlipNone)  // Top-left
        },
        (
            _,     false,  false,
            true,         false,
            _,     false, false
        ) => {
            (1, 1, FlipContext::FlipNone)  // Left
        },
        (
            _,     false, false,
            true,         false,
            _,     true , _
        ) => {
            (1, 2, FlipContext::FlipNone)  // Bottom-left
        },
        (
            _,     true,  _,
            false,        true,
            false, false, _
        ) => {
            (1, 0, FlipContext::FlipHorizontal)  // Top-right
        },
        (
            false,  false, _,
            false,         true,
            false,  false, _
        ) => {
            (1, 1, FlipContext::FlipHorizontal)  // Right
        },
        (
            false, false, _,
            false,        true,
            _,     true,  _
        ) => {
            (1, 2, FlipContext::FlipHorizontal)  // Bottom-right
        },
        (
            _, true,      _,
            _,            _,
            false, false, false
        ) => {
            (6, 1, FlipContext::FlipNone)  // Top
        },
        (
            false, false, false,
            _,            _,
            _, true,      _
        ) => {
            (6, 3, FlipContext::FlipNone)  // Bottom
        },
        (
            _,    true,   _,
            false,        false,
            false, false, true
        ) => {
            (3, 0, FlipContext::FlipHorizontal)  // T
        },
        (
            _,    true,  _,
            false,       false,
            true, false, false
        ) => {
            (3, 0, FlipContext::FlipNone)  // T
        },
        (
            true, false, false,
            false,       false,
            _, true,  false
        ) => {
            (3, 2, FlipContext::FlipNone)  // T
        },
        (
            false, false, true,
            false,        false,
            false, true,  _
        ) => {
            (3, 2, FlipContext::FlipHorizontal)  // T
        },
        (
            true,  false, false,
            false,        false,
            false, false, false
        ) => {
            (2, 0, FlipContext::FlipNone)  // Inside elbow top left
        },
        (
            false, false, true,
            false,        false,
            false, false, false
        ) => {
            (2, 0, FlipContext::FlipHorizontal)  // Inside elbow top right
        },
        (
            false, false, false,
            false,        false,
            true, false, false
        ) => {
            (2, 2, FlipContext::FlipNone)  // Inside elbow bottom left
        },
        (
            false, false, false,
            false,        false,
            false, false, true
        ) => {
            (2, 2, FlipContext::FlipHorizontal)  // Inside elbow bottom right
        },
        (
            false, false, true,
            false,        false,
            true, false, false
        ) => {
            (3, 1, FlipContext::FlipNone)  // Diagonal
        },
        (
            true,  false, false,
            false,        false,
            false, false, true,
        ) => {
            (3, 1, FlipContext::FlipHorizontal)  // Diagonal
        },
        (
            false, false, _,
            false,        true,
            true, false, _
        ) => {
            (4, 2, FlipContext::FlipNone)  // Side missing bottom corner
        },
        (
            _,    false, false,
            true,        false,
            _,    false, true
        ) => {
            (4, 2, FlipContext::FlipHorizontal)  // Side missing bottom corner
        },
        (
            true, false, _,
            false,        true,
            false, false, _
        ) => {
            (4, 0, FlipContext::FlipNone)  // Side missing top corner
        },
        (
            _,    false,  true,
            true,        false,
            _,    false, false,
        ) => {
            (4, 0, FlipContext::FlipHorizontal)  // Side missing top corner
        },
        (
            _,     true,  _,
            false,        false,
            true,  false, true,
        ) => {
            (6, 0, FlipContext::FlipNone)  // T
        },
        (
            true,  false, true,
            false,        false,
            _,     true,  _
        ) => {
            (7, 0, FlipContext::FlipNone)  // Inverse T
        },
        (
            true,  false, true,
            false,        false,
            _,     false, _
        ) => {
            (4, 3, FlipContext::FlipNone)  // T
        },
        (
            _,     false, _,
            false,        false,
            true,  false, true
        ) => {
            (4, 3, FlipContext::FlipVertical)  // T
        },
        (
            _,     true,  _,
            true,         false,
            _,     false, true
        ) => {
            (2, 3, FlipContext::FlipNone)  // Top-left bend
        },
        (
            _,      true,  _,
            false,         true,
            true,    false, _
        ) => {
            (2, 3, FlipContext::FlipHorizontal)  // Top-right bend
        },
        (
            _,     false, true,
            true,         false,
            _,     true , _
        ) => {
            (3, 3, FlipContext::FlipNone)  // Bottom-left
        },
        (
            true,  false, _,
            false,        true,
            _,      true , _
        ) => {
            (3, 3, FlipContext::FlipHorizontal)  // Bottom-left
        },
        (
            _, _, _,
            _,    _,
            _, _, _
        ) => {
            (6, 2, FlipContext::FlipNone)
        }
    }
}

impl CellState {
    pub fn is_passable(&self) -> bool {
        match *self {
            CellState::Empty | CellState::Dirt => true,
            _ => false
        }
    }

    pub fn get_color(&self) -> Color {
        let BLACK = Color::RGB(0, 0, 0);
        let BROWN = Color::RGB(100, 100, 0);
        let GREY = Color::RGB(100, 100, 100);
        let WHITE = Color::RGB(255, 255, 255);
        match *self {
            CellState::Dirt => BROWN,
            CellState::Stone => GREY,
            CellState::Wall => WHITE,
            CellState::Empty => BLACK
        }
    }

    pub fn get_sprite(&self, adjacents: &Adjacents) -> Option<(u32, u32, FlipContext)> {
        match *self {
            CellState::Dirt => {
                let adjacent_tile_states = adjacents.as_tile_states();
                let mut adjacent_tiles = [false; 8];
                for i in 0..8 {
                    adjacent_tiles[i] = match adjacent_tile_states[i] {
                        CellState::Empty => true,
                        _ => false
                    }
                };
                let (x, y, context) = get_tileset_sprite(&adjacent_tiles);
                Some((x + 8, y, context))
            },
            CellState::Stone => {
                let adjacent_tile_states = adjacents.as_tile_states();
                let mut adjacent_tiles = [false; 8];
                for i in 0..8 {
                    adjacent_tiles[i] = match adjacent_tile_states[i] {
                        CellState::Empty | CellState::Dirt => true,
                        _ => false
                    }
                };
                let (x, y, context) = get_tileset_sprite(&adjacent_tiles);
                Some((x + 8, y + 12, context))
            },
            CellState::Wall => Some((2, 5, FlipContext::FlipNone)),
            CellState::Empty => None
        }
    }
}

