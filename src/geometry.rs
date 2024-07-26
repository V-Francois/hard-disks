pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub fn distance_sq(pos_1: &Position, pos_2: &Position) -> f64 {
    let dx = pos_1.x - pos_2.x;
    let dy = pos_1.y - pos_2.y;
    return dx * dx + dy * dy;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let pos_1 = Position { x: 0.0, y: 0.0 };
        let pos_2 = Position { x: 1.0, y: 2.0 };
        assert_eq!(distance_sq(&pos_1, &pos_2), 5.0);
    }
}
