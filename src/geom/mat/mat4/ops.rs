use super::*;

impl<T: Num + Copy> Add for Mat4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<T: Num + Copy + AddAssign> AddAssign for Mat4<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self[(i, j)] += rhs[(i, j)];
            }
        }
    }
}

impl<T: Num + Copy> Sub for Mat4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<T: Num + Copy + SubAssign> SubAssign for Mat4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self[(i, j)] -= rhs[(i, j)];
            }
        }
    }
}

impl<T: Num + Copy + Neg<Output = T>> Neg for Mat4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[(i, j)] = -result[(i, j)];
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> Mul for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut result = Mat4::new([[T::ZERO; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                let cur = &mut result[(i, j)];
                for t in 0..4 {
                    *cur += self[(i, t)] * rhs[(t, j)];
                }
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> MulAssign for Mat4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Num + Copy> Mul<T> for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Mat4<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..4 {
            for j in 0..4 {
                self[(i, j)] *= rhs;
            }
        }
    }
}

impl<T: Num + Copy> Div<T> for Mat4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<T: Num + Copy + DivAssign> DivAssign<T> for Mat4<T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..4 {
            for j in 0..4 {
                self[(i, j)] /= rhs;
            }
        }
    }
}

impl<T: Num + Copy> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Vec4<T> {
        vec4(
            Vec4::dot(self.row(0), rhs),
            Vec4::dot(self.row(1), rhs),
            Vec4::dot(self.row(2), rhs),
            Vec4::dot(self.row(3), rhs),
        )
    }
}
