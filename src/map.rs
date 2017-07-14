use crystal::Crystal;
use enemy::{TurnDir, Enemy};
use entity::{Entity, RcEntity};
use exit::Exit;
use game_data::GameData;
use game_scene::GameEvent;
use player::Player;
use rock::Rock;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::event_bus::EventBus;
use std::cmp::Ordering;
use std::collections::HashMap;
use transform::TransformContext;
use map_loader::MapData;
use tileset::FlipContext;

#[derive(Clone, Copy)]
pub enum CellState {
    Empty,
    Dirt,
    Stone,
    Wall
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
                        Some((13, 0, FlipContext::FlipNone))  // Standalone
                    },
                    (
                        _,     true,  _,
                        true,         true,
                        _,     false, _
                    ) => {
                        Some((13, 1, FlipContext::FlipNone))  // Top nub
                    },
                    (
                        _,     false,  _,
                        true,         true,
                        _,     false, _
                    ) => {
                        Some((13, 2, FlipContext::FlipNone))  // Vertical bar
                    },
                    (
                        _,     false,  _,
                        true,         true,
                        _,     true, _
                    ) => {
                        Some((13, 3, FlipContext::FlipNone))  // Bottom nub
                    },
                    (
                        _,     true,  _,
                        true,         false,
                        _,     true,  _
                    ) => {
                        Some((9, 3, FlipContext::FlipNone))  // Left nub
                    },
                    (
                        _,     true,  _,
                        false,        false,
                        _,     true,  _
                    ) => {
                        Some((15, 1, FlipContext::FlipNone))  // Horizontal bar
                    },
                    (
                        _,     true,  _,
                        false,        true,
                        _,     true,  _
                    ) => {
                        Some((9, 3, FlipContext::FlipHorizontal))  // Right nub
                    },
                    (
                        _,     true,  _,
                        true,         false,
                        _,     false, false
                    ) => {
                        Some((9, 0, FlipContext::FlipNone))  // Top-left
                    },
                    (
                        _,     false,  false,
                        true,         false,
                        _,     false, false
                    ) => {
                        Some((9, 1, FlipContext::FlipNone))  // Left
                    },
                    (
                        _,     false, false,
                        true,         false,
                        _,     true , _
                    ) => {
                        Some((9, 2, FlipContext::FlipNone))  // Bottom-left
                    },
                    (
                        _,     true,  _,
                        false,        true,
                        false, false, _
                    ) => {
                        Some((9, 0, FlipContext::FlipHorizontal))  // Top-right
                    },
                    (
                        false,  false, _,
                        false,         true,
                        false,  false, _
                    ) => {
                        Some((9, 1, FlipContext::FlipHorizontal))  // Right
                    },
                    (
                        false, false, _,
                        false,        true,
                        _,     true,  _
                    ) => {
                        Some((9, 2, FlipContext::FlipHorizontal))  // Bottom-right
                    },
                    (
                        _, true,      _,
                        _,            _,
                        false, false, false
                    ) => {
                        Some((14, 1, FlipContext::FlipNone))  // Top
                    },
                    (
                        false, false, false,
                        _,            _,
                        _, true,      _
                    ) => {
                        Some((14, 3, FlipContext::FlipNone))  // Bottom
                    },
                    (
                        _,    true,   _,
                        false,        false,
                        false, false, true
                    ) => {
                        Some((11, 0, FlipContext::FlipHorizontal))  // T
                    },
                    (
                        _,    true,  _,
                        false,       false,
                        true, false, false
                    ) => {
                        Some((11, 0, FlipContext::FlipNone))  // T
                    },
                    (
                        true, false, false,
                        false,       false,
                        true, true,  false
                    ) => {
                        Some((11, 2, FlipContext::FlipNone))  // T
                    },
                    (
                        false, false, true,
                        false,        false,
                        false, true,  true
                    ) => {
                        Some((11, 2, FlipContext::FlipHorizontal))  // T
                    },
                    (
                        true,  false, false,
                        false,        false,
                        false, false, false
                    ) => {
                        Some((10, 0, FlipContext::FlipNone))  // Inside elbow top left
                    },
                    (
                        false, false, true,
                        false,        false,
                        false, false, false
                    ) => {
                        Some((10, 0, FlipContext::FlipHorizontal))  // Inside elbow top right
                    },
                    (
                        false, false, false,
                        false,        false,
                        true, false, false
                    ) => {
                        Some((10, 2, FlipContext::FlipNone))  // Inside elbow bottom left
                    },
                    (
                        false, false, false,
                        false,        false,
                        false, false, true
                    ) => {
                        Some((10, 2, FlipContext::FlipHorizontal))  // Inside elbow bottom right
                    },
                    (
                        _, _, _,
                        _,    _,
                        _, _, _
                    ) => {
                        Some((14, 2, FlipContext::FlipNone))
                    }
                }
            },
            CellState::Stone => Some((8, 0, FlipContext::FlipNone)),
            CellState::Wall => Some((2, 5, FlipContext::FlipNone)),
            CellState::Empty => None
        }
    }
}

pub struct Map {
    cells: Vec<CellState>,
    entities: Vec<RcEntity>,
    locations: HashMap<(u32, u32), Vec<RcEntity>>,
    conflicts: HashMap<(u32, u32), Vec<RcEntity>>,
    pub player: RcEntity,
    pub width: u32,
    pub height: u32,
    pub crystals_to_pass: u32,
    pub crystals_collected: u32,
    pub is_complete: bool,
}

pub type Adjacent = Option<(CellState, Vec<RcEntity>)>;

pub struct Adjacents {
    pub cells: [Adjacent; 8],
}

impl Adjacents {
    pub fn top_left(&self) -> &Adjacent {
        &self.cells[0]
    }

    pub fn top(&self) -> &Adjacent {
        &self.cells[1]
    }

    pub fn top_right(&self) -> &Adjacent {
        &self.cells[2]
    }

    pub fn left(&self) -> &Adjacent {
        &self.cells[3]
    }

    pub fn right(&self) -> &Adjacent {
        &self.cells[4]
    }

    pub fn bottom_left(&self) -> &Adjacent {
        &self.cells[5]
    }

    pub fn bottom(&self) -> &Adjacent {
        &self.cells[6]
    }

    pub fn bottom_right(&self) -> &Adjacent {
        &self.cells[7]
    }

    pub fn as_tile_states(&self) -> [CellState; 8] {
        let mut cells = [CellState::Wall; 8];
        for i in 0..8 {
            match self.cells[i] {
                Some((state, _)) => {
                    cells[i] = state
                },
                _ => ()
            }
        };
        cells
    }
}

pub const CELL_SIZE: u32 = 32;

impl Map {
    pub fn new(data: &MapData) -> Map {
        let mut ids = 0;
        let mut entities = vec![];
        let mut cells = vec![CellState::Empty; (data.width * data.height) as usize];
        let mut player = None;

        // Validate
        assert!(data.width * data.height == data.cells.len() as u32);

        for (i, cell) in data.cells.chars().enumerate() {
            let (x, y) = (i as u32 % data.width, i as u32 / data.width);
            let cell_data = match cell {
                '.' => {
                    CellState::Empty
                },
                '\'' => {
                    CellState::Dirt
                },
                '#' => {
                    CellState::Wall
                },
                '+' => {
                    CellState::Stone
                },
                '>' => {
                    let entity = Entity::new(x, y, Enemy::new(TurnDir::CLOCKWISE), &mut ids);
                    entities.push(entity);
                    CellState::Empty
                },
                '<' => {
                    let entity = Entity::new(x, y, Enemy::new(TurnDir::ANTICLOCKWISE), &mut ids);
                    entities.push(entity);
                    CellState::Empty
                },
                'o' => {
                    let entity = Entity::new(x, y, Rock::new(), &mut ids);
                    entities.push(entity);
                    CellState::Empty
                },
                '*' => {
                    let entity = Entity::new(x, y, Crystal::new(), &mut ids);
                    entities.push(entity);
                    CellState::Empty
                },
                '&' => {
                    let entity = Entity::new(x, y, Exit::new(), &mut ids);
                    entities.push(entity);
                    CellState::Empty
                },
                '@' => {
                    player = Some(Entity::new(x, y, Player::new(), &mut ids));
                    CellState::Empty
                },
                _ => {
                    CellState::Empty
                }
            };
            cells[i] = cell_data;
        }
        let player = player.expect("No player defined in map!");
        let borrow = player.clone();
        entities.push(player);
        let mut map = Map {
            cells: cells,
            entities: entities,
            width: data.width,
            height: data.height,
            player: borrow,
            locations: HashMap::new(),
            conflicts: HashMap::new(),
            crystals_to_pass: data.crystals_to_pass,
            crystals_collected: 0,
            is_complete: false,
        };

        map
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, engine_data: &GameData, tick: u64, camera_pos: (u32, u32)) {
        let (camera_x, camera_y) = camera_pos;
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        let transform = TransformContext::new()
            .transform(-(camera_x as i32), -(camera_y as i32));

        let (width, height) = renderer.logical_size();

        for (i, cell) in self.cells.iter().enumerate() {
            let x = i as u32 % self.width;
            let y = i as u32 / self.width;
            let adjacents = self.adjacents(x, y);
            match cell.get_sprite(&adjacents) {
                Some((tile_x, tile_y, flip_state)) => engine_data.assets.tileset.blit_sprite(renderer, tile_x, tile_y, &transform.transform((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32), Some(flip_state)),
                None => ()
            }
        };

        for entity in &self.entities {
            let entity = entity.borrow();
            let (x, y) = entity.get_abs_position();
            let transform = transform.transform(x, y);
            entity.render(renderer, &transform, engine_data, tick);
        }
    }

    pub fn think(&mut self, event_bus: &mut EventBus<GameEvent>, renderer: &mut Canvas<Window>, engine_data: &GameData, tick: u64) {
        // Update our location grid
        self.update_location_grid();

        for entity in &self.entities {
            let entity = entity.borrow();
            entity.collisions(event_bus, self.at_pos(entity.state.x, entity.state.y, Some(entity.state.id)));
        }
        for entity in &self.entities {
            let mut entity = entity.borrow_mut();
            let adjacents = self.adjacents(entity.state.x, entity.state.y);
            entity.think(event_bus, &adjacents, engine_data, tick);
        }
        // Handle dealing with queued events
        while let Some(event) = event_bus.next() {
            match event {
                GameEvent::MoveRequest(direction) => {
                    let mut player = self.player.borrow_mut();
                    let adjacents = self.adjacents(player.state.x, player.state.y);
                    player.input(direction, &adjacents);
                },
                GameEvent::Dig(x, y) => {
                    self.set_cell_state(x, y, CellState::Empty);
                },
                GameEvent::Crushed(item) => {
                    let (x, y) = {
                        let item = item.borrow();
                        (item.state.x, item.state.y)
                    };
                    item.borrow_mut().destroy();
                    event_bus.enqueue(GameEvent::Explosion(x, y));
                },
                GameEvent::Collect(item) => {
                    item.borrow_mut().destroy();
                    self.crystals_collected += item.borrow().score();
                    if self.crystals_collected >= self.crystals_to_pass {
                        for entity in &self.entities {
                            let mut entity = entity.borrow_mut();
                            entity.open_exit();
                        }
                    }
                },
                GameEvent::Explosion(x, y) => {
                    let (x, y) = (x as i64, y as i64);
                    let (width, height) = (self.width as i64, self.height as i64);
                    for &(x, y) in vec![
                        (x - 1, y - 1),
                        (x, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y),
                        (x,  y),
                        (x + 1, y),
                        (x - 1, y + 1),
                        (x, y + 1),
                        (x + 1, y + 1),
                    ].iter().filter(|pair| {
                        let &(x, y) = *pair;
                        x >= 0 && y >= 0 && x < width && y < height
                    }) {
                        let (x, y) = (x as u32, y as u32);
                        let (state, items) = self.at_pos(x, y, None);
                        match state {
                            CellState::Wall => {},
                            _ => self.set_cell_state(x, y, CellState::Empty)
                        };
                        for item in items {
                            if item.borrow().destructable() {
                                item.borrow_mut().destroy();
                            }
                        }
                    }
                },
                GameEvent::Push(dir, item) => {
                    item.borrow_mut().push(dir, tick);
                },
                GameEvent::Complete => {
                    self.is_complete = true
                }
            }
        }
        // Cleanup items
        self.cleanup();
        // Handle conflicts
        self.handle_conflicts();
        // Do actual movement
        if !self.is_complete {
            for entity in &self.entities {
                entity.borrow_mut().process(tick);
            }
        }
    }

    pub fn at_pos(&self, x: u32, y: u32, checking_id: Option<u32>) -> (CellState, Vec<RcEntity>) {
        let index = x + (y * self.width);
        let item = self.locations.get(&(x, y));
        let entities = match (item, checking_id) {
            (Some(items), Some(check)) => {
                items.iter().filter_map(|item| { if item.borrow().state.id != check { Some(item.clone()) } else { None } }).collect::<Vec<_>>()
            },
            (Some(items), None) => {
                items.clone()
            },
            _ => vec![]
        };
        (self.cells[index as usize].clone(), entities)
    }

    pub fn adjacents(&self, x: u32, y: u32) -> Adjacents {
        let top_left = if y > 0 && x > 0 {
            Some(self.at_pos(x - 1, y - 1, None))
        } else {
            None
        };

        let top = if y > 0 {
            Some(self.at_pos(x, y - 1, None))
        } else {
            None
        };

        let top_right = if y > 0 && x < self.width - 1 {
            Some(self.at_pos(x + 1, y - 1, None))
        } else {
            None
        };

        let left = if x > 0 {
            Some(self.at_pos(x - 1, y, None))
        } else {
            None
        };

        let right = if x < self.width - 1 {
            Some(self.at_pos(x + 1, y, None))
        } else {
            None
        };

        let bottom_left = if y < self.height - 1 && x > 0 {
            Some(self.at_pos(x - 1, y + 1, None))
        } else {
            None
        };

        let bottom = if y < self.height - 1 {
            Some(self.at_pos(x, y + 1, None))
        } else {
            None
        };

        let bottom_right = if y < self.height - 1 && x < self.width - 1 {
            Some(self.at_pos(x + 1, y + 1, None))
        } else {
            None
        };

        Adjacents {
            cells: [
                top_left,
                top,
                top_right,
                left,
                right,
                bottom_left,
                bottom,
                bottom_right,
            ]
        }
    }

    pub fn set_cell_state(&mut self, x: u32, y: u32, state: CellState) {
        let index = x + (y * self.width);
        self.cells[index as usize] = state;
    }

    pub fn update_location_grid(&mut self) {
        self.locations.clear();
        for entity in &self.entities {
            let (x, y) = {
                let entity = entity.borrow();
                (entity.state.x, entity.state.y)
            };
            let new_list = match self.locations.remove(&(x, y)) {
                Some(mut list) => {
                    list.push(entity.clone());
                    list
                },
                None => vec![entity.clone()]
            };
            self.locations.insert((x, y), new_list);
        }
    }

    pub fn handle_conflicts(&mut self) {
        /*
         * Check if multiple items are trying to enter a square.
         * If they're hard, only one should move
         */
        self.conflicts.clear();
        for entity in &self.entities {
            match entity.borrow().target_square() {
                Some((x, y)) => {
                    let new_list = match self.conflicts.remove(&(x, y)) {
                        Some(mut list) => {
                            list.push(entity.clone());
                            list
                        },
                        None => vec![entity.clone()]
                    };
                    self.conflicts.insert((x, y), new_list);
                },
                None => ()
            }
        }

        for list in self.conflicts.values() {
            if list.len() > 1 {
                let priority = list.iter().filter(|item| item.borrow().is_hard()).max_by(|left, right| {
                    if right.borrow().state.pos_fraction > left.borrow().state.pos_fraction {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                match priority {
                    Some(item) => {
                        let item = item.borrow();
                        for other in list {
                            match other.try_borrow_mut() {
                                Ok(mut item) => {
                                    item.deflect()
                                },
                                _ => ()
                            }
                        }
                    },
                    None => ()
                }
            }
        }
    }

    pub fn cleanup(&mut self) {
        self.entities = self.entities.iter().filter_map(|entity| {
            if entity.borrow().state.destroyed {
                None
            } else {
                Some(entity.clone())
            }
        }).collect::<Vec<RcEntity>>()
    }
}
