use super::*;

impl<T: Num + Copy> Add for mat3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<T: Num + Copy + AddAssign> AddAssign for mat3<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            for j in 0..3 {
                self[(i, j)] += rhs[(i, j)];
            }
        }
    }
}

impl<T: Num + Copy> Sub for mat3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<T: Num + Copy + SubAssign> SubAssign for mat3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            for j in 0..3 {
                self[(i, j)] -= rhs[(i, j)];
            }
        }
    }
}

impl<T: Num + Copy + Neg<Output = T>> Neg for mat3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for i in 0..3 {
            for j in 0..3 {
                result[(i, j)] = -result[(i, j)];
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> Mul for mat3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut result = mat3::new([[T::ZERO; 3]; 3]);
        for i in 0..3 {
            for j in 0..3 {
                let cur = &mut result[(i, j)];
                for t in 0..3 {
                    *cur += self[(i, t)] * rhs[(t, j)];
                }
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> MulAssign for mat3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Num + Copy> Mul<T> for mat3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for mat3<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..3 {
            for j in 0..3 {
                self[(i, j)] *= rhs;
            }
        }
    }
}

impl<T: Num + Copy> Div<T> for mat3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<T: Num + Copy + DivAssign> DivAssign<T> for mat3<T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..3 {
            for j in 0..3 {
                self[(i, j)] /= rhs;
            }
        }
    }
}

impl<T: Num + Copy> Mul<vec3<T>> for mat3<T> {
    type Output = vec3<T>;

    fn mul(self, rhs: vec3<T>) -> vec3<T> {
        vec3(
            vec3::dot(self.row(0), rhs),
            vec3::dot(self.row(1), rhs),
            vec3::dot(self.row(2), rhs),
        )
    }
}
