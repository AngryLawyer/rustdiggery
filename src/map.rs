use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use entity::{Entity, RcEntity};
use transform::TransformContext;
use game_scene::GameEvent;
use sdl2_engine_helpers::event_bus::EventBus;

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
    pub player: RcEntity,
    pub width: u32,
    pub height: u32
}

pub struct Adjacents {
    pub top: Option<(CellState, Option<RcEntity>)>,
    pub left: Option<(CellState, Option<RcEntity>)>,
    pub bottom: Option<(CellState, Option<RcEntity>)>,
    pub right: Option<(CellState, Option<RcEntity>)>
}

pub const CELL_SIZE: u32 = 32;

impl Map {
    pub fn new(width: u32, height: u32) -> Map{
        let mut entities = vec![];
        let mut cells = vec![CellState::Dirt; (width * height) as usize];
        let player = Entity::new(1,1);
        let borrow = player.clone();
        entities.push(player);
        cells[1] = CellState::Empty;
        cells[width as usize] = CellState::Empty;
        cells[width as usize + 1] = CellState::Empty;
        cells[width as usize + 2] = CellState::Empty;
        cells[(width as usize * 2) + 1] = CellState::Stone;

        Map {
            cells: cells,
            entities: entities,
            width: width,
            height: height,
            player: borrow
        }
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64, camera_pos: (u32, u32)) {
        let (camera_x, camera_y) = camera_pos;
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        //TODO: Camera offset?
        let transform = TransformContext::new()
            .transform(-(camera_x as i32), -(camera_y as i32));

        let (width, height) = renderer.logical_size();

        for (i, cell) in self.cells.iter().enumerate() {
            let x = (i as u32 % self.width) * CELL_SIZE;
            let y = (i as u32 / self.height) * CELL_SIZE;
            renderer.set_draw_color(cell.get_color());
            transform.fill_rect(renderer, Rect::new(x as i32, y as i32, CELL_SIZE, CELL_SIZE));
        };

        for entity in self.entities.iter() {
            entity.borrow().render(renderer, &transform, engine_data, tick);
        }

        renderer.present();
    }

    pub fn think(&mut self, event_bus: &mut EventBus<GameEvent>, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64) {
        // TODO: Check for collisions
        {
            let player = self.player.borrow();
            player.collisions(event_bus, self.at_pos(player.x, player.y));
        }
        // Handle dealing with queued events
        while let Some(event) = event_bus.next() {
            match event {
                GameEvent::MoveEvent(direction) => {
                    let mut player = self.player.borrow_mut();
                    let adjacents = self.adjacents(player.x, player.y);
                    player.input(direction, &adjacents);
                },
                GameEvent::DigEvent(x, y) => {
                    self.set_cell_state(x, y, CellState::Empty);
                },
                _ => ()
            }
        }
        // Do actual movement
        let mut player = self.player.borrow_mut();
        player.think(tick);
    }

    /*pub fn render(&self, context: &graphics::Context, gl: &mut GlGraphics, tick: u64) {
        for entity in self.entities.iter() {
            entity.borrow().render(context, gl, tick);
        }
    }*/

    pub fn at_pos(&self, x: u32, y: u32) -> (CellState, Option<RcEntity>) {
        let index = x + (y * self.width);
        return (self.cells[index as usize].clone(), None);
    }

    pub fn adjacents(&self, x: u32, y: u32) -> Adjacents {
        let top = if y > 0 {
            Some(self.at_pos(x, y - 1))
        } else {
            None
        };

        let left = if x > 0 {
            Some(self.at_pos(x - 1, y))
        } else {
            None
        };

        let bottom = if y < self.height - 1 {
            Some(self.at_pos(x, y + 1))
        } else {
            None
        };

        let right = if x < self.width - 1 {
            Some(self.at_pos(x + 1, y))
        } else {
            None
        };

        Adjacents{top: top, left: left, bottom: bottom, right: right}
    }

    pub fn set_cell_state(&mut self, x: u32, y: u32, state: CellState) {
        let index = x + (y * self.width);
        self.cells[index as usize] = state;
    }
}

