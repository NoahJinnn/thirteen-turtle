//! Pros: 
//! - Immubility: Easy to reason about
//! - Stateless, no global state: Easy to composite 
//! - Functions are modular and reusable in different contexts
//! Cons: 
//! - Hardcode dependency in functions, 
//! - Keep track of the state

#[allow(dead_code)]
use crate::common::{calc_new_pos, draw_line, Degree, Distance, PenColor, PenState, Position};
pub struct TurtleState {
    pub pos: Position,
    pub angle: Degree,
    pub pen_state: PenState,
    pub color: PenColor,
}

impl Default for TurtleState {
    fn default() -> Self {
        TurtleState {
            pos: Position::default(),
            angle: 0.0,
            pen_state: PenState::default(),
            color: PenColor::default(),
        }
    }
}

#[allow(dead_code)]
impl TurtleState {
    pub fn transit(&self, d: Distance) -> TurtleState {
        println!("{:.1}", d);
        let new_pos = calc_new_pos(d, self.angle, &self.pos);
        match &self.pen_state {
            PenState::Down => draw_line(&self.pos, &new_pos),
            _ => {}
        }
        TurtleState {
            pos: new_pos,
            ..*self
        }
    }

    pub fn turn(&self, a: Degree) -> TurtleState {
        println!("{:.1}", a);
        let new_a = (self.angle + a) % 360.0;
        TurtleState {
            angle: new_a,
            ..*self
        }
    }

    pub fn pen_up(&self) -> TurtleState {
        TurtleState {
            pen_state: PenState::Up,
            ..*self
        }
    }

    pub fn pen_down(&self) -> TurtleState {
        TurtleState {
            pen_state: PenState::Down,
            ..*self
        }
    }

    pub fn set_color(self: &TurtleState, color: PenColor) -> TurtleState {
        TurtleState { color, ..*self }
    }
}

#[cfg(test)]
mod test {
    use crate::fp_tt::TurtleState;

    #[test]
    fn draw_triangle() {
        let tt_state = TurtleState::default()
            .transit(100.0)
            .turn(120.0)
            .transit(100.0)
            .turn(120.0)
            .transit(100.0)
            .turn(120.0);

        assert_eq!(tt_state.pos.x, 0.0);
        assert_eq!(tt_state.pos.y, 0.0);
    }
}
