use bevy::prelude::*;
use approx::assert_ulps_eq;


#[derive(Component)]
pub struct Velocity {
    pub scalar: f32,
    pub direction: f32,
}

impl Velocity {
    pub fn new(scalar: f32, direction_degrees: f32) -> Velocity {
        Velocity {
            scalar,
            direction: direction_degrees,
        }
    }

    pub fn new_xy(x: f32, y: f32) -> Velocity {
        let mut v = Velocity::new(0.0, 0.0);
        v.from_xy(x, y);
        v
    }

    pub fn to_xy(&self) -> (f32, f32) {
        (self.direction.to_radians().cos() * self.scalar,
         self.direction.to_radians().sin() * self.scalar)
    }

    pub fn from_xy(&mut self, x: f32, y: f32) {
        self.scalar = (x.powi(2) + y.powi(2)).sqrt();
        self.direction = y.atan2(x).to_degrees();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_xy_q0() {
        let v = Velocity::new(1.0, 0.0);
        assert_ulps_eq!(v.to_xy().0, 1.0);
        assert_ulps_eq!(v.to_xy().1, 0.0);

        let v = Velocity::new(1.0, 45.0);
        assert_ulps_eq!(v.to_xy().0, 2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, 2_f32.sqrt() / 2.0);

        let v = Velocity::new(1.0, 30.0);
        assert_ulps_eq!(v.to_xy().0, 3_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, 0.5);
    }

    #[test]
    fn to_xy_q1() {
        let v = Velocity::new(1.0, 90.0);
        assert_ulps_eq!(v.to_xy().0, 0.0);
        assert_ulps_eq!(v.to_xy().1, 1.0);

        let v = Velocity::new(1.0, 135.0);
        assert_ulps_eq!(v.to_xy().0, -2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, 2_f32.sqrt() / 2.0);

        let v = Velocity::new(1.0, 150.0);
        assert_ulps_eq!(v.to_xy().0, -3_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, 0.5);
    }

    #[test]
    fn to_xy_q2() {
        let v = Velocity::new(1.0, 180.0);
        assert_ulps_eq!(v.to_xy().0, -1.0);
        assert_ulps_eq!(v.to_xy().1, 0.0);

        let v = Velocity::new(1.0, 225.0);
        assert_ulps_eq!(v.to_xy().0, -2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, -2_f32.sqrt() / 2.0);

        let v = Velocity::new(1.0, 240.0);
        assert_ulps_eq!(v.to_xy().0, -0.5);
        assert_ulps_eq!(v.to_xy().1, -3_f32.sqrt() / 2.0);
    }

    #[test]
    fn to_xy_q3() {
        let v = Velocity::new(1.0, 270.0);
        assert_ulps_eq!(v.to_xy().0, 0.0);
        assert_ulps_eq!(v.to_xy().1, -1.0);

        let v = Velocity::new(1.0, 315.0);
        assert_ulps_eq!(v.to_xy().0, 2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, -2_f32.sqrt() / 2.0);

        let v = Velocity::new(1.0, 330.0);
        assert_ulps_eq!(v.to_xy().0, 3_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.to_xy().1, -0.5);
    }

    #[test]
    fn from_xy_q0() {
        let v = Velocity::new_xy(0.0, 0.0);
        assert_ulps_eq!(v.scalar, 0.0);
        assert_ulps_eq!(v.direction, 0.0);

        let v = Velocity::new_xy(1.0, 0.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 0.0);

        let v = Velocity::new_xy(3_f32.sqrt() / 2.0, 0.5);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 30.0);

        let v = Velocity::new_xy(2_f32.sqrt() / 2.0, 2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 45.0);
    }

    #[test]
    fn from_xy_q1() {
        let v = Velocity::new_xy(0.0, 1.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 90.0);

        let v = Velocity::new_xy(-3_f32.sqrt() / 2.0, 0.5);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 150.0);

        let v = Velocity::new_xy(-2_f32.sqrt() / 2.0, 2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 135.0);
    }

    #[test]
    fn from_xy_q2() {
        let v = Velocity::new_xy(-1.0, 0.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 180.0);

        let v = Velocity::new_xy(-3_f32.sqrt() / 2.0, -0.5);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 210.0 - 360.0);

        let v = Velocity::new_xy(-2_f32.sqrt() / 2.0, -2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 225.0 - 360.0);
    }

    #[test]
    fn from_xy_q3() {
        let v = Velocity::new_xy(0.0, -1.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 270.0 - 360.0);

        let v = Velocity::new_xy(3_f32.sqrt() / 2.0, -0.5);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 330.0 - 360.0);

        let v = Velocity::new_xy(2_f32.sqrt() / 2.0, -2_f32.sqrt() / 2.0);
        assert_ulps_eq!(v.scalar, 1.0);
        assert_ulps_eq!(v.direction, 315.0 - 360.0);
    }
}
