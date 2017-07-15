use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_data::GameData;
use game_scene::GameEvent;
use cell_state::CellState;
use map::{CELL_SIZE, Adjacents};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::event_bus::EventBus;
use transform::TransformContext;

const PUSH_AMPLITUDE: u32 = 10;

pub struct Rock {
    momentum: bool,
    pushing: (Movement, u32, u64),
}

impl Rock {
    pub fn new() -> Box<EntityType> {
        Box::new(Rock {
            momentum: false,
            pushing: (Movement::NEUTRAL, 0, 0)
        })
    }
}

pub fn handle_collisions(state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>), momentum: bool) {
    if !momentum {
        return;
    }

    match cell_state {
        (_, ref items) if items.len() > 0 => {
            for item in items {
                if !item.borrow().is_hard() {
                    event_bus.enqueue(GameEvent::Crushed(item.clone()));
                }
            }
        },
        _ => ()
    }
}

pub fn think(state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64, momentum: &mut bool) {
    if state.pos_fraction != 0.0 {
        return;
    }

    match (
        adjacents.left(),
        adjacents.right(),
        adjacents.bottom_left(),
        adjacents.bottom(),
        adjacents.bottom_right()
    ) {
        (_, _, _, &Some((CellState::Empty, ref items)), _) if items.len() == 0 => {
            state.movement = Movement::DOWN;
            state.cell_move_state = CellMoveState::EXITING;
            *momentum = true;
        },
        (_, _, _, &Some((CellState::Empty, ref items)), _) if items.len() > 0 && *momentum => {
            if !items.first().unwrap().borrow().is_hard() {
                state.movement = Movement::DOWN;
                state.cell_move_state = CellMoveState::EXITING;
                *momentum = true;
            } else {
                state.movement = Movement::NEUTRAL;
                *momentum = false;
            }
        },
        (_, &Some((CellState::Empty, ref right_items)), _, &Some((CellState::Empty, ref underneath)), &Some((CellState::Empty, ref bottom_right_items))) if right_items.len() == 0 && bottom_right_items.len() == 0 => {
            if underneath.first().unwrap().borrow().is_hard() {
                state.movement = Movement::RIGHT;
                state.cell_move_state = CellMoveState::EXITING;
                *momentum = true;
            } else {
                state.movement = Movement::NEUTRAL;
                *momentum = false;
            }
        },
        (&Some((CellState::Empty, ref left_items)), _, &Some((CellState::Empty, ref bottom_left_items)), &Some((CellState::Empty, ref underneath)), _) if left_items.len() == 0 && bottom_left_items.len() == 0 => {
            if underneath.first().unwrap().borrow().is_hard() {
                state.movement = Movement::LEFT;
                state.cell_move_state = CellMoveState::EXITING;
                *momentum = true;
            } else {
                state.movement = Movement::NEUTRAL;
                *momentum = false;
            }
        },
        _ => {
            state.movement = Movement::NEUTRAL;
            *momentum = false;
        }
    };
}

impl EntityType for Rock {
    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        handle_collisions(state, event_bus, cell_state, self.momentum);
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, engine_data: &GameData, tick: u64) {
        think(state, event_bus, adjacents, tick, &mut self.momentum);

        if state.pos_fraction != 0.0 {
            return;
        }
        let reset = match self.pushing {
            (ref dir, amplitude, push_tick) => {
                if tick - 1 > push_tick {
                    true
                } else if amplitude >= PUSH_AMPLITUDE {
                    match (dir, adjacents.left(), adjacents.right()) {
                        (&Movement::LEFT, &Some((CellState::Empty, ref items)), _) if items.len() == 0 => {
                            state.movement = Movement::LEFT;
                            state.cell_move_state = CellMoveState::EXITING;
                        },
                        (&Movement::RIGHT, _, &Some((CellState::Empty, ref items))) if items.len() == 0 => {
                            state.movement = Movement::RIGHT;
                            state.cell_move_state = CellMoveState::EXITING;
                        },
                        _ => ()
                    };
                    false
                } else {
                    false
                }
            }
        };
        if reset {
            self.pushing = (Movement::NEUTRAL, 0, 0);
        }
    }

    fn is_hard(&self) -> bool {
        true
    }

    fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, state: &EntityState, engine_data: &GameData, tick: u64) {
        renderer.set_draw_color(Color::RGB(255, 255, 100));
        transform.fill_rect(
            renderer,
            Rect::new(
                0,
                0,
                CELL_SIZE,
                CELL_SIZE
            )
        ).expect("Could not draw entity");
    }

    fn push(&mut self, direction: Movement, tick: u64) {
        match self.pushing {
            (_, amplitude, _) => {
                self.pushing = (direction, amplitude + 1, tick);
            }
        }
    }
}
