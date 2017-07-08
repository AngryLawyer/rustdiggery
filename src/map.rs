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

#[derive(Clone)]
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
    pub top_left: Adjacent,
    pub top: Adjacent,
    pub top_right: Adjacent,
    pub left: Adjacent,
    pub right: Adjacent,
    pub bottom_left: Adjacent,
    pub bottom: Adjacent,
    pub bottom_right: Adjacent,
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
            let x = (i as u32 % self.width);
            let y = (i as u32 / self.width);
            renderer.set_draw_color(cell.get_color());
            transform.fill_rect(renderer, Rect::new((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32, CELL_SIZE, CELL_SIZE)).expect("Could not draw cell");
        };

        for entity in &self.entities {
            let entity = entity.borrow();
            let (x, y) = entity.get_abs_position();
            let transform = transform.transform(x, y);
            entity.render(renderer, &transform, engine_data, tick);
        }

        renderer.present();
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
            top_left: top_left,
            top: top,
            top_right: top_right,
            left: left,
            right: right,
            bottom_left: bottom_left,
            bottom: bottom,
            bottom_right: bottom_right
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
