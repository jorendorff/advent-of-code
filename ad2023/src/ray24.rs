use std::ops::{Add, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub vel: Vec3,
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, y: f64) -> Vec3 {
        let r = 1.0 / y;
        Vec3 {
            x: self.x * r,
            y: self.y * r,
            z: self.z * r,
        }
    }
}

impl Ray {
    /// Return square of the distance between two **lines** (not really rays).
    /// Could return a distance in the past.
    pub fn nearest_approach_squared(&self, other: Ray) -> f64 {
        // The position of this particle at time t is
        //     self.origin + t * self.vel
        // The position of other is
        //     other.origin + t * other.vel
        // The squared distance between the two is the norm of the difference of these two.
        let d0 = other.origin - self.origin;
        let dv = other.vel - self.vel;
        //     d_squared(t) = norm_squared(d0 + t * dv)
        // Expanding:
        //     d_squared(t) = (
        //           (d0.x + t * dv.x)**2
        //         + (d0.y + t * dv.y)**2
        //         + (d0.z + t * dv.z)**2
        //     )
        //     = (d0.x ** 2 + d0.y ** 2 + d0.z ** 2)
        //       + 2 * (d0.x * dv.x + d0.y * dv.y + d0.z * dv.z) * t
        //       + (dv.x ** 2 + dv.y ** 2 + dv.z ** 2) * t**2
        // Rewriting back in terms of vectors:
        //     d_squared(t) = norm_squared(d0) + 2 * dot(d0, dv) * t + norm_squared(dv) * t**2
        // This is minimized when the derivative is zero:
        //     0 = D_t d_squared(t_min)
        //       = 2 * dot(d0, dv) + 2 * norm_squared(dv) * t_min
        // When dv is nonzero, we can solve for t_min:
        //     t_min = - dot(d0, dv) / norm_squared(dv)
        // then apply the formula for d_squared:
        //     d_squared(t_min) = norm_squared(d0 - dot(d0, dv) / norm_squared(dv) * dv)
        // But if dv is zero, the solution is also easy: the particles are not moving
        // relative to one another, so it's just `d0.norm_squared()`.
        // Maybe there's a better expression for this? It's what I have.
        if dv.is_zero() {
            d0.norm_squared()
        } else {
            // (d0 - d0 . dv * dv) / |dv|^2
            // (d0 - |d0||dv| cos θ * dv) / |dv|^2
            // (|d0| dir(d0) - |d0||dv| cos θ |dv| dir(dv)) / |dv|^2
            // |d0| (dir(d0)/|dv|^2 - cos θ dir(dv))
            (d0 - d0.dot(dv) * dv / dv.norm_squared()).norm_squared()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_approach() {
        for dy in 0..4 {
            let dy = dy as f64;
            for dz in 0..4 {
                let dz = dz as f64;

                let a = Ray {
                    origin: Vec3 {
                        x: -5.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    vel: Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                };
                let b = Ray {
                    origin: Vec3 {
                        x: 5.0,
                        y: dy,
                        z: dz,
                    },
                    vel: Vec3 {
                        x: -1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                };
                assert_eq!(a.nearest_approach_squared(b).round(), dy * dy + dz * dz);
                assert_eq!(b.nearest_approach_squared(a).round(), dy * dy + dz * dz);
            }
        }
    }
}
