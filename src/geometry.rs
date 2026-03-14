use std::ops::{Add, Sub, Mul, Div};

// ===============================================================================================
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T>
{
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(x :T, y:T)->Self { Self { x, y } }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self)->Self::Output {
        Self{ x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self)->Self::Output {
        Self{ x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: T)->Self::Output {
        Self{ x: self.x * rhs, y: self.y * rhs}
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn div(self, rhs: T)->Self::Output {
        Self{ x: self.x / rhs, y: self.y / rhs}
    }
}

impl<T> Vec2<T>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> Vec2<T>
where
    T: Mul<Output = T> + Sub<Output = T>,
{
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Vec2<f32> {
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
}

pub type Vec2i = Vec2<i32>;

// ===============================================================================================
#[derive(Debug, Clone, Copy)]
struct Vec3<T>
{
    x : T,
    y : T,
    z : T
}

impl<T> Vec3<T> {
    fn new(x: T, y: T, z: T)->Self {
        Self { x, y, z}
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self)->Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self)->Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: T)->Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn div(self, rhs: T)->Self::Output {
        Self{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Clone,
{
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy + Clone,
{
    fn cross(self, rhs: Self) -> Self {
        Self::new(self.y * rhs.z - self.z * rhs.y,
                  self.z * rhs.x - self.x * rhs.z,
                  self.x * rhs.y - self.y * rhs.x)
    }
}

impl Vec3<f32> {
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