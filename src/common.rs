use std::f32::consts::PI;

pub type Distance = f32;
pub type Degree = f32;
pub enum PenState {
    Up,
    Down,
}
pub enum PenColor {
    Black,
    Red,
    Blue,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0.0, y: 0.0 }
    }
}

pub fn calc_new_pos(d: Distance, a: Degree, cur_pos: &Position) -> Position {
    let angle_in_rads = a * (PI / 180.0) * 1.0;
    let x0 = cur_pos.x;
    let y0 = cur_pos.y;
    let x1 = x0 + d * angle_in_rads.cos();
    let y1 = y0 + d * angle_in_rads.sin();

    Position { x: x1.round(), y: y1.round() }
}

pub fn draw_line(old_pos: &Position, new_pos: &Position) {
    println!(
        "Draw line from ({},{}) to ({},{})",
        old_pos.x, old_pos.y, new_pos.x, new_pos.y
    );
}
