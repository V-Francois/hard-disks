pub struct Disk {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub fn are_disks_overlapping(disk_1: &Disk, disk_2: &Disk) -> bool {
    let dx = disk_1.x - disk_2.x;
    let dy = disk_1.y - disk_2.y;
    let rsq = dx * dx + dy * dy;
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
            x: 0.0,
            y: 0.0,
            radius: 2.0,
        };

        let disk_2 = Disk {
            x: 0.0,
            y: 3.0,
            radius: 2.0,
        };

        let disk_3 = Disk {
            x: 0.0,
            y: 5.0,
            radius: 2.0,
        };
        assert!(are_disks_overlapping(&disk_1, &disk_2));
        assert!(!are_disks_overlapping(&disk_1, &disk_3));
    }
}
