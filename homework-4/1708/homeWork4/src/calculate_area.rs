/**
   实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，
   比如 圆形，三角形，正方形，需要用到泛型和泛型约束
*/

pub mod calculate_area{
  
 pub fn shapes_calculate_area<T:Shapes>(shape :T)->f32{
    shape.shape_calculate_area()
  }

  pub trait Shapes{
    fn shape_calculate_area(&self)->f32;
  }
  
  pub struct Circle{
    pub radius:f32,
    pub pi:f32
  }

  pub struct Triangle{
    pub bottom:f32,
    pub high:f32
  }

  pub struct Square{
    pub side:f32
  }

  impl Shapes for Square{
    fn shape_calculate_area(&self)->f32{
      &self.side * &self.side
    }
  }
  
  impl Shapes for Triangle{
    fn shape_calculate_area(&self)->f32{
      0.5 * &self.bottom * &self.high
    }
  }

  impl Shapes for Circle{
    fn shape_calculate_area(&self)->f32{
      &self.pi * &self.radius * &self.radius
    }
  }
  

}
