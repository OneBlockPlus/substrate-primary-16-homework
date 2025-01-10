pub trait Shapes{
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub enum ShapeTypes<X1>{
    Triangle(X1, X1),
    Circle(X1),
    Square(X1),
}

impl<X1> Shapes for ShapeTypes<X1>
where
    X1: Copy + Into<f64>,
{
    fn area(&self) -> f64 {
        match self {
            ShapeTypes::Triangle(x1, y1) => {
                let x = (*x1).into();
                let y = (*y1).into();
                x * y * 0.5
            }
            ShapeTypes::Circle(x1) => {
                let x = (*x1).into();
                std::f64::consts::PI * x * x
            }
            ShapeTypes::Square(x1) => {
                let x = (*x1).into();
                x * x
            }
        }
    }
}