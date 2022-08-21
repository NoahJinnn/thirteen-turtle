//! Client -> API -> Class
//! Pros: 
//! - Encapsulation
//! - Provide service boundaries: validation, monitoring, internal routing, load balancing...
//! Cons: 
//! - API couples with implementation
//! - Still stateful

use std::fmt::{Display};
use std::str::FromStr;
use crate::{common::{Distance, Degree, PenColor}, oo_tt::Turtle};

#[derive(Debug, Clone)]
pub struct ApiErr;

impl Display for ApiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Api error")
    }
}

#[allow(dead_code)]
type Result<T> = std::result::Result<T, ApiErr>;

pub fn validate_distance(d: &str) -> Result<Distance> {
    let d = f32::from_str(d).unwrap(); // throw exceptions is not supported by Rust, so unwrap() is quick way to demo
    Ok(d)
}

pub fn validate_angle(a: &str) -> Result<Degree> {
    let a = f32::from_str(a).unwrap();
    Ok(a)
}

pub fn validate_color(color: &str) -> Result<PenColor> {
    let color = PenColor::from(color);
    Ok(color)
}

pub fn turtle_oo_api(command: &str) -> Result<bool> {
    let mut tt = Turtle::new();
    let str_v: Vec<&str> = command.split(" ").collect();
    match str_v[0] {
        "Move" => {
            println!("{}", str_v[1]);
            let d = validate_distance(str_v[1])?;
            tt.transit(d);
        },
        "Turn" => {
            println!("{}", str_v[1]);
            let a = validate_angle(str_v[1])?;
            tt.turn(a);
        },
        "PenUp" => {
            tt.pen_up();
        }, 
        "PenDown" => {
            tt.pen_down();
        },
        "SetColor" => {
            println!("{}", str_v[1]);
            let color = validate_color(str_v[1])?;
            tt.set_color(color);
        }
        _ => return Err(ApiErr)
    }
    Ok(true)
}

#[cfg(test)]
mod test {
    use super::turtle_oo_api;

    #[test]
    fn draw_triangle() {
        turtle_oo_api("Move 100.0");
        turtle_oo_api("Turn 120.0");
        turtle_oo_api("Move 100.0");
        turtle_oo_api("Turn 120.0");
        turtle_oo_api("Move 100.0");
        turtle_oo_api("Turn 120.0");
    }
}