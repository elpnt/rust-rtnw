use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn test_point_at_parameter() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 2.0, 3.0),
        };
        let t: f32 = 2.0;
        assert_eq!(ray.point_at_parameter(t), Vec3::new(2.0, 4.0, 6.0));
    }
}
