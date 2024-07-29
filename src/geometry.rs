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
    let mut dy = pos_1.y - pos_2.y;
    apply_boundary_conditions(&mut dx, &mut dy, sim_box);
    return dx * dx + dy * dy;
}

pub fn apply_boundary_conditions(dx: &mut f64, dy: &mut f64, sim_box: &Box) {
    if *dx > sim_box.lx / 2.0 {
        *dx -= sim_box.lx;
    } else if *dx < -sim_box.lx / 2.0 {
        *dx += sim_box.lx;
    }
    if *dy > sim_box.ly / 2.0 {
        *dy -= sim_box.ly;
    } else if *dy < -sim_box.ly / 2.0 {
        *dy += sim_box.ly;
    }
}

pub fn put_in_box_x(x: f64, sim_box: &Box) -> f64 {
    let mut new_x = x;
    if new_x > sim_box.lx {
        new_x -= sim_box.lx * (new_x / sim_box.lx).floor();
    } else if new_x < -0.0 {
        new_x += sim_box.lx * -(new_x / sim_box.lx).floor();
    }
    return new_x;
}

pub fn put_in_box_y(y: f64, sim_box: &Box) -> f64 {
    let mut new_y = y;
    if new_y > sim_box.ly {
        new_y -= sim_box.ly * (new_y / sim_box.ly).floor();
    } else if new_y < -0.0 {
        new_y += sim_box.ly * -(new_y / sim_box.ly).floor();
    }
    return new_y;
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
