
pub const LIMITS: [(f32, f32); 4] = [
    (0.0, 180.0),  // Base rotation
    (60.0,150.0),  // Lower joint
    (40.0, 120.0), // Upper joint
    (72.0, 180.0), // Claw
];

pub const INNER_ARM_LENGTH: f32 = 134.5;
pub const OUTER_ARM_LENGTH: f32 = 148.0;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct RawPoint {
    pub base: f32,
    pub lower: f32,
    pub upper: f32,
    pub claw: f32,
}

impl RawPoint {
    pub fn values(&self) -> [f32; 4] {
        return [self.base, self.lower, self.upper, self.claw];
    }

    pub fn get_bytes(&self) -> [u8; 4] {
        return [self.base as u8, self.lower as u8, self.upper as u8, self.claw as u8];
    }

    pub fn is_valid(&self) -> bool {
        let values = self.values();
        for i in 0..4 {
            if values[i] < LIMITS[i].0 || values[i] > LIMITS[i].1 {
                return false;
            }
        }

        return true;
    }
}

impl PartialEq for RawPoint {
    fn eq(&self, other: &Self) -> bool {
        return self.base == other.base && self.lower == other.lower && self.upper == other.upper && self.claw == other.claw;
    }
}

impl From<CartesianPoint> for RawPoint {
    fn from(point: CartesianPoint) -> Self {

        let distance_to_goal = point.distance(CartesianPoint {x: 0.0, y: 0.0, z: 0.0});
        let angle_from_horizontal = radians_to_degrees((point.y / distance_to_goal).asin());

        let triangle_a = law_of_cosines(INNER_ARM_LENGTH, distance_to_goal, OUTER_ARM_LENGTH);
        let triangle_b = law_of_cosines(OUTER_ARM_LENGTH, INNER_ARM_LENGTH, distance_to_goal);

        let angle_a = 180.0 - triangle_a - angle_from_horizontal;
        let angle_b = triangle_a + triangle_b + angle_from_horizontal - 90.0;//270.0 - triangle_a - triangle_b - angle_from_horizontal;


        return RawPoint{ base: 90.0, lower: angle_a, upper: angle_b, claw: 90.0 };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CartesianPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl CartesianPoint {
    fn distance(&self, other: CartesianPoint) -> f32 {
        return ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2)).sqrt() as f32;
    }
}

fn law_of_cosines(a: f32, b: f32, c: f32) -> f32 {
    let denom = 2.0 * a * b;
    let num = a.powi(2) + b.powi(2) - c.powi(2);
    let rad_res = (num / denom).acos();
    return radians_to_degrees(rad_res);
}

fn radians_to_degrees(x: f32) -> f32 {
    use std::f32::consts::PI;
    return x * (180.0 / PI);
}
