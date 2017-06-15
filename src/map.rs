use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use entity::{Entity, RcEntity};
use transform::TransformContext;
use game_scene::GameEvent;
use sdl2_engine_helpers::event_bus::EventBus;
use player::Player;
use rock::Rock;
use std::collections::HashMap;
use std::cmp::Ordering;

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
    pub height: u32
}

pub struct Adjacents {
    pub top_left: Option<(CellState, Vec<RcEntity>)>,
    pub top: Option<(CellState, Vec<RcEntity>)>,
    pub top_right: Option<(CellState, Vec<RcEntity>)>,
    pub left: Option<(CellState, Vec<RcEntity>)>,
    pub right: Option<(CellState, Vec<RcEntity>)>,
    pub bottom_left: Option<(CellState, Vec<RcEntity>)>,
    pub bottom: Option<(CellState, Vec<RcEntity>)>,
    pub bottom_right: Option<(CellState, Vec<RcEntity>)>
}

pub const CELL_SIZE: u32 = 32;

impl Map {
    pub fn new(width: u32, height: u32) -> Map{
        let mut ids = 0;
        let mut entities = vec![];
        let mut cells = vec![CellState::Dirt; (width * height) as usize];
        let player = Entity::new(0,1, Player::new(), &mut ids);
        let borrow = player.clone();
        entities.push(player);
        cells[1] = CellState::Empty;
        cells[width as usize] = CellState::Empty;
        cells[width as usize + 1] = CellState::Empty;
        cells[width as usize + 2] = CellState::Empty;
        cells[(width as usize * 2) + 1] = CellState::Stone;

        let mut map = Map {
            cells: cells,
            entities: entities,
            width: width,
            height: height,
            player: borrow,
            locations: HashMap::new(),
            conflicts: HashMap::new()
        };

        map.set_cell_state(5, 0, CellState::Empty);
        map.set_cell_state(5, 1, CellState::Empty);
        map.set_cell_state(5, 2, CellState::Empty);
        map.set_cell_state(5, 3, CellState::Empty);
        map.set_cell_state(6, 1, CellState::Empty);
        map.set_cell_state(6, 2, CellState::Empty);
        map.set_cell_state(6, 3, CellState::Empty);
        let rock = Entity::new(5, 0, Rock::new(), &mut ids);
        map.entities.push(rock);
        let rock = Entity::new(5, 1, Rock::new(), &mut ids);
        map.entities.push(rock);
        let rock = Entity::new(5, 2, Rock::new(), &mut ids);
        map.entities.push(rock);

        map
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64, camera_pos: (u32, u32)) {
        let (camera_x, camera_y) = camera_pos;
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        let transform = TransformContext::new()
            .transform(-(camera_x as i32), -(camera_y as i32));

        let (width, height) = renderer.logical_size();

        for (i, cell) in self.cells.iter().enumerate() {
            let x = (i as u32 % self.width) * CELL_SIZE;
            let y = (i as u32 / self.height) * CELL_SIZE;
            renderer.set_draw_color(cell.get_color());
            transform.fill_rect(renderer, Rect::new(x as i32, y as i32, CELL_SIZE, CELL_SIZE));
        };

        for entity in &self.entities {
            entity.borrow().render(renderer, &transform, engine_data, tick);
        }

        renderer.present();
    }

    pub fn think(&mut self, event_bus: &mut EventBus<GameEvent>, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64) {
        // Update our location grid
        self.update_location_grid();

        for entity in &self.entities {
            let entity = entity.borrow();
            entity.collisions(event_bus, self.at_pos(entity.state.x, entity.state.y));
        }
        for entity in &self.entities {
            let mut entity = entity.borrow_mut();
            let adjacents = self.adjacents(entity.state.x, entity.state.y);
            entity.think(event_bus, &adjacents, tick);
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
                    // TODO: Explosions!
                    let (x, y) = {
                        let item = item.borrow();
                        (item.state.x, item.state.y)
                    };
                    item.borrow_mut().destroy();
                    event_bus.enqueue(GameEvent::Explosion(x, y));
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
                        let (state, items) = self.at_pos(x, y);
                        match state {
                            CellState::Wall => {},
                            _ => self.set_cell_state(x, y, CellState::Empty)
                        };
                        for item in items {
                            item.borrow_mut().destroy();
                        }
                    }
                },
                GameEvent::Push(dir, item) => {
                    item.borrow_mut().push(dir, tick);
                }
                _ => ()
            }
        }
        // Cleanup items
        self.cleanup();
        // Handle conflicts
        self.handle_conflicts();
        // Do actual movement
        for entity in &self.entities {
            entity.borrow_mut().process(tick);
        }
    }

    pub fn at_pos(&self, x: u32, y: u32) -> (CellState, Vec<RcEntity>) {
        let index = x + (y * self.width);
        let item = self.locations.get(&(x, y));
        return (self.cells[index as usize].clone(), if item.is_some() { item.unwrap().clone() } else { vec![] });
    }

    pub fn adjacents(&self, x: u32, y: u32) -> Adjacents {
        let top_left = if y > 0 && x > 0 {
            Some(self.at_pos(x - 1, y - 1))
        } else {
            None
        };

        let top = if y > 0 {
            Some(self.at_pos(x, y - 1))
        } else {
            None
        };

        let top_right = if y > 0 && x < self.width - 1 {
            Some(self.at_pos(x + 1, y - 1))
        } else {
            None
        };

        let left = if x > 0 {
            Some(self.at_pos(x - 1, y))
        } else {
            None
        };

        let right = if x < self.width - 1 {
            Some(self.at_pos(x + 1, y))
        } else {
            None
        };

        let bottom_left = if y < self.height - 1 && x > 0 {
            Some(self.at_pos(x - 1, y + 1))
        } else {
            None
        };

        let bottom = if y < self.height - 1 {
            Some(self.at_pos(x, y + 1))
        } else {
            None
        };

        let bottom_right = if y < self.height - 1 && x < self.width - 1 {
            Some(self.at_pos(x + 1, y + 1))
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
