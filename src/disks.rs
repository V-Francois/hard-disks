use crate::geometry::distance_sq_periodic;
use crate::geometry::Box;
use crate::geometry::Position;

pub struct Disk {
    pub position: Position,
    pub radius: f64,
}

pub fn are_disks_overlapping(disk_1: &Disk, disk_2: &Disk, sim_box: &Box) -> bool {
    let rsq = distance_sq_periodic(&disk_1.position, &disk_2.position, &sim_box);
    let sigma_sq = (disk_1.radius + disk_2.radius) * (disk_1.radius + disk_2.radius);
    return rsq < sigma_sq;
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::disks::are_disks_overlapping;

    #[test]
    fn test_overalap() {
        let sim_box_1 = Box { lx: 1.0, ly: 10.0 };
        let sim_box_2 = Box { lx: 1.0, ly: 6.0 };

        let disk_1 = Disk {
            position: Position { x: 0.0, y: 0.0 },
            radius: 2.0,
        };

        let disk_2 = Disk {
            position: Position { x: 0.0, y: 3.0 },
            radius: 2.0,
        };

        let disk_3 = Disk {
            position: Position { x: 0.0, y: 5.0 },
            radius: 2.0,
        };
        assert!(are_disks_overlapping(&disk_1, &disk_2, &sim_box_1));
        assert!(!are_disks_overlapping(&disk_1, &disk_3, &sim_box_1));
        assert!(are_disks_overlapping(&disk_1, &disk_3, &sim_box_2));
    }
}
