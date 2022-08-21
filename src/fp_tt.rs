//! Pros: Easy to understand, stateless, no global state, the functions are modular and reusable in different contexts
//! Cons: Tight coupling to functions, need to keep track of the state

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

pub fn transit(state: &TurtleState, d: Distance) -> TurtleState {
    println!("{:.1}", d);
    let new_pos = calc_new_pos(d, state.angle, &state.pos);
    match &state.pen_state {
        PenState::Down => draw_line(&state.pos, &new_pos),
        _ => {}
    }
    TurtleState {
        pos: new_pos,
        ..*state
    }
}

pub fn turn(state: &TurtleState, a: Degree) -> TurtleState {
    println!("{:.1}", a);
    let new_a = (state.angle + a) % 360.0;
    TurtleState {
        angle: new_a,
        ..*state
    }
}

pub fn pen_up(state: &TurtleState) -> TurtleState {
    TurtleState {
        pen_state: PenState::Up,
        ..*state
    }
}

pub fn pen_down(state: &TurtleState) -> TurtleState {
    TurtleState {
        pen_state: PenState::Down,
        ..*state
    }
}

pub fn set_color(state: &TurtleState, color: PenColor) -> TurtleState {
    TurtleState { color, ..*state }
}

#[cfg(test)]
mod test {
    use crate::fp_tt::{TurtleState, transit, turn};


    #[test]
    fn draw_triangle() {
        let mut tt_state = TurtleState::default();
        tt_state = transit(&tt_state, 100.0);
        tt_state = turn(&tt_state, 120.0);
        tt_state = transit(&tt_state, 100.0);
        tt_state = turn(&tt_state, 120.0);
        tt_state = transit(&tt_state, 100.0);
        tt_state = turn(&tt_state, 120.0);

        assert_eq!(tt_state.pos.x, 0.0);
        assert_eq!(tt_state.pos.y, 0.0);
    }
}
