pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Box {
    pub lx: f64,
    pub ly: f64,
}

pub fn distance_sq(pos_1: &Position, pos_2: &Position) -> f64 {
    let dx = pos_1.x - pos_2.x;
    let dy = pos_1.y - pos_2.y;
    return dx * dx + dy * dy;
}

pub fn distance_sq_periodic(pos_1: &Position, pos_2: &Position, sim_box: &Box) -> f64 {
    let mut dx = pos_1.x - pos_2.x;
    if dx > sim_box.lx / 2.0 {
        dx -= sim_box.lx;
    } else if dx < -sim_box.lx / 2.0 {
        dx += sim_box.lx;
    }
    let mut dy = pos_1.y - pos_2.y;
    if dy > sim_box.ly / 2.0 {
        dy -= sim_box.ly;
    } else if dy < -sim_box.ly / 2.0 {
        dy += sim_box.ly;
    }
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

    #[test]
    fn test_distance_periodic() {
        let pos_1 = Position { x: 0.0, y: 0.0 };
        let pos_2 = Position { x: 0.0, y: 4.0 };
        let sim_box = Box { lx: 1.0, ly: 5.0 };
        assert_eq!(distance_sq_periodic(&pos_1, &pos_2, &sim_box), 1.0);
    }
}
