//! Pros: Easy to understand (OO abstraction), abstract data from behavior
//! Cons: 
//! - Blackbox the state from client
//! - Hard to composite
//! - Hard code (tight coupling) other service: log, drawing...

use crate::common::{calc_new_pos, draw_line, Degree, Distance, PenColor, PenState, Position};
#[allow(dead_code)]
pub struct Turtle {
    pos: Position,
    angle: Degree,
    pen_state: PenState,
    color: PenColor,
}

#[allow(dead_code)]
impl Turtle {
    pub fn new() -> Self {
        Turtle {
            pos: Position::default(),
            angle: 0.0,
            color: PenColor::default(),
            pen_state: PenState::default(),
        }
    }

    pub fn transit(&mut self, d: Distance) {
        println!("{:.1}", d);
        let new_pos = calc_new_pos(d, self.angle, &self.pos);
        match &self.pen_state {
            PenState::Down => draw_line(&self.pos, &new_pos),
            _ => {}
        }
        self.pos = new_pos;
    }

    pub fn turn(&mut self, a: Degree) {
        println!("{:.1}", a);
        let new_a = (self.angle + a) % 360.0;
        self.angle = new_a;
    }

    pub fn pen_up(&mut self) {
        self.pen_state = PenState::Up;
    }

    pub fn pen_down(&mut self) {
        self.pen_state = PenState::Down;
    }

    pub fn set_color(&mut self, color: PenColor) {
        self.color = color;
    }
}

#[cfg(test)]
mod test {
    use super::Turtle;

    #[test]
    fn draw_triangle() {
        let mut tt = Turtle::new();
        tt.transit(100.0);
        tt.turn(120.0);
        tt.transit(100.0);
        tt.turn(120.0);
        tt.transit(100.0);
        tt.turn(120.0);

        assert_eq!(tt.pos.x, 0.0);
        assert_eq!(tt.pos.y, 0.0);
    }
}
