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

pub fn get_tileset_sprite(adjacents: &Adjacents) -> (u32, u32, FlipContext) {
    let adjacent_tile_states = adjacents.as_tile_states();
    let mut adjacent_tiles = [false; 8];
    for i in 0..8 {
        adjacent_tiles[i] = match adjacent_tile_states[i] {
            CellState::Empty => true,
            _ => false
        }
    };

    let adjacent_tiles = (
        adjacent_tiles[0],
        adjacent_tiles[1],
        adjacent_tiles[2],
        adjacent_tiles[3],
        adjacent_tiles[4],
        adjacent_tiles[5],
        adjacent_tiles[6],
        adjacent_tiles[7],
    );
    match adjacent_tiles {
        (
            _,     true,  _,
            true,         true,
            _,     true, _
        ) => {
            (4, 0, FlipContext::FlipNone)  // Standalone
        },
        (
            _,     true,  _,
            true,         true,
            _,     false, _
        ) => {
            (4, 1, FlipContext::FlipNone)  // Top nub
        },
        (
            _,     false,  _,
            true,         true,
            _,     false, _
        ) => {
            (4, 2, FlipContext::FlipNone)  // Vertical bar
        },
        (
            _,     false,  _,
            true,         true,
            _,     true, _
        ) => {
            (4, 3, FlipContext::FlipNone)  // Bottom nub
        },
        (
            _,     true,  _,
            true,         false,
            _,     true,  _
        ) => {
            (0, 3, FlipContext::FlipNone)  // Left nub
        },
        (
            _,     true,  _,
            false,        false,
            _,     true,  _
        ) => {
            (6, 1, FlipContext::FlipNone)  // Horizontal bar
        },
        (
            _,     true,  _,
            false,        true,
            _,     true,  _
        ) => {
            (0, 3, FlipContext::FlipHorizontal)  // Right nub
        },
        (
            _,     true,  _,
            true,         false,
            _,     false, false
        ) => {
            (0, 0, FlipContext::FlipNone)  // Top-left
        },
        (
            _,     false,  false,
            true,         false,
            _,     false, false
        ) => {
            (0, 1, FlipContext::FlipNone)  // Left
        },
        (
            _,     false, false,
            true,         false,
            _,     true , _
        ) => {
            (0, 2, FlipContext::FlipNone)  // Bottom-left
        },
        (
            _,     true,  _,
            false,        true,
            false, false, _
        ) => {
            (0, 0, FlipContext::FlipHorizontal)  // Top-right
        },
        (
            false,  false, _,
            false,         true,
            false,  false, _
        ) => {
            (0, 1, FlipContext::FlipHorizontal)  // Right
        },
        (
            false, false, _,
            false,        true,
            _,     true,  _
        ) => {
            (0, 2, FlipContext::FlipHorizontal)  // Bottom-right
        },
        (
            _, true,      _,
            _,            _,
            false, false, false
        ) => {
            (5, 1, FlipContext::FlipNone)  // Top
        },
        (
            false, false, false,
            _,            _,
            _, true,      _
        ) => {
            (5, 3, FlipContext::FlipNone)  // Bottom
        },
        (
            _,    true,   _,
            false,        false,
            false, false, true
        ) => {
            (2, 0, FlipContext::FlipHorizontal)  // T
        },
        (
            _,    true,  _,
            false,       false,
            true, false, false
        ) => {
            (2, 0, FlipContext::FlipNone)  // T
        },
        (
            true, false, false,
            false,       false,
            true, true,  false
        ) => {
            (2, 2, FlipContext::FlipNone)  // T
        },
        (
            false, false, true,
            false,        false,
            false, true,  true
        ) => {
            (2, 2, FlipContext::FlipHorizontal)  // T
        },
        (
            true,  false, false,
            false,        false,
            false, false, false
        ) => {
            (1, 0, FlipContext::FlipNone)  // Inside elbow top left
        },
        (
            false, false, true,
            false,        false,
            false, false, false
        ) => {
            (1, 0, FlipContext::FlipHorizontal)  // Inside elbow top right
        },
        (
            false, false, false,
            false,        false,
            true, false, false
        ) => {
            (1, 2, FlipContext::FlipNone)  // Inside elbow bottom left
        },
        (
            false, false, false,
            false,        false,
            false, false, true
        ) => {
            (1, 2, FlipContext::FlipHorizontal)  // Inside elbow bottom right
        },
        (
            _, _, _,
            _,    _,
            _, _, _
        ) => {
            (5, 2, FlipContext::FlipNone)
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
                let (x, y, context) = get_tileset_sprite(adjacents);
                Some((x + 9, y, context))
            },
            CellState::Stone => Some((8, 0, FlipContext::FlipNone)),
            CellState::Wall => Some((2, 5, FlipContext::FlipNone)),
            CellState::Empty => None
        }
    }
}

