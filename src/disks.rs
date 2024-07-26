use crate::geometry::distance_sq;
use crate::geometry::Position;

pub struct Disk {
    pub position: Position,
    pub radius: f64,
}

pub fn are_disks_overlapping(disk_1: &Disk, disk_2: &Disk) -> bool {
    let rsq = distance_sq(&disk_1.position, &disk_2.position);
    let sigma_sq = (disk_1.radius + disk_2.radius) * (disk_1.radius + disk_2.radius);
    return rsq < sigma_sq;
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::disks::are_disks_overlapping;

    #[test]
    fn test_overalap() {
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
        assert!(are_disks_overlapping(&disk_1, &disk_2));
        assert!(!are_disks_overlapping(&disk_1, &disk_3));
    }
}
