use homework4::geometry::{Area, Circle, Square, Triangle};

#[test]
fn test_circle_area() {
    let circle = Circle { radius: 3.0 };
    assert!((circle.area() - 28.27).abs() < 1e-2);
}

#[test]
fn test_triangle_area() {
    let triangle = Triangle {
        base: 4.0,
        height: 5.0,
    };
    assert_eq!(triangle.area(), 10.0);
}

#[test]
fn test_square_area() {
    let square = Square { side: 2.0 };
    assert_eq!(square.area(), 4.0);
}
