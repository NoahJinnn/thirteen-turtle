use crate::common::{Degree, Distance, PenColor, PenState, calc_new_pos, draw_line, Position};
#[allow(dead_code)]
struct Turtle {
    pos: Position,
    angle: Degree,
    pen_state: PenState,
    color: PenColor,
}

#[allow(dead_code)]
impl Turtle {
    fn transit(&mut self, d: Distance) {
        println!("{:.1}", d);
        let new_pos = calc_new_pos(d, self.angle, &self.pos);
        match &self.pen_state {
            PenState::Down => draw_line(&self.pos, &new_pos),
            _ => {}
        }
        self.pos = new_pos;
    }

    fn turn(&mut self, a: Degree) {
        println!("{:.1}", a);
        let new_a = (self.angle + a) % 360.0;
        self.angle = new_a;
    }

    fn pen_up(&mut self) {
        self.pen_state = PenState::Up;
    }

    fn pen_down(&mut self) {
        self.pen_state = PenState::Down;
    }

    fn set_color(&mut self, color: PenColor) {
        self.color = color;
    }
}

#[cfg(test)]
mod test {
    use crate::common::{Position, PenColor, PenState};

    use super::Turtle;

    #[test]
    fn draw_triangle() {
        let mut tt = Turtle {
            pos: Position::default(),
            angle: 0.0,
            color: PenColor::Black,
            pen_state: PenState::Down
        };
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