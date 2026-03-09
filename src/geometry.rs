#[derive(Debug, Clone, Copy)]
pub struct Vec2
{
    x: f32,
    y: f32
}

impl Vec2 {
    fn new(x :f32, y:f32)->Self { Self { x, y } }
    fn add(self, other: Self)->Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
    fn sub(self, other: Self)->Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
    fn dot(self, other:Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    fn mul_scalar(self, k: f32)->Self {
        Self::new(self.x * k, self.y * k)
    }
    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn normalize(self) -> Self {
        let len = self.length();
        if len <= f32::EPSILON {
            self
        }
        else {
            Self::new(self.x / len, self.y / len)
        }
    }
    fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec3
{
    x : f32,
    y : f32,
    z : f32
}

impl Vec3 {
    fn new(x : f32, y : f32, z : f32) ->Self  {
        Self{ x, y, z }
    }
    fn add(self, other: Self)->Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    fn sub(self, other: Self)->Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    fn dot(self, other:Self)->f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(self, other: Self)->Self {
        Self::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }
    fn mul_scalar(self, k: f32)->Self {
        Self::new(self.x * k, self.y * k, self.z * k)
    }
    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn normalize(self) -> Self {
        let len = self.length();
        if len <= f32::EPSILON {
            self
        } else {
            Self::new(self.x / len, self.y / len, self.z / len)
        }
    }
}