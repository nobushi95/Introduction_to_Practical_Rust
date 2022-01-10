use crate::coordinates::Coordinates;

pub struct Matrix(pub [[f64; 2]; 2]);

pub trait LinearTransform : Coordinates {
    fn transform(self, matrix: &Matrix) -> Self
    where
        Self: Sized
    {
        let mut cart = self.to_cartesian();
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;

        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * x + m[1][0] * y;
        Self::from_cartesian(cart)
    }

    // デフォルト実装
    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }
}