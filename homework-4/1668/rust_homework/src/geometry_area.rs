
// 定义一个area trait用于面积计算
pub trait  Area{
    fn area(&self)->Result<f64,String>;
}
//定义一个求圆面积的数据结构，半径为radius
pub struct CircleArea{
    pub radius: f64,
}
//定义一个求正方形面积的数据结构，四个边相等，皆为side_length
pub struct SquareArea{
    pub side_length: f64,
}
//定义一个矩形面积的数据结构，长为side_length,宽为side_width
pub struct  RectangleArea{
    pub side_length: f64,
    pub side_width: f64
}
//定义一个三角形的数据结构,其中边长分别为a,b,c
pub struct TriangleArea{
    pub a: f64,
    pub b: f64,
    pub c: f64
}
//圆面积的具体算法实现
impl Area for CircleArea {
    fn area(&self) -> Result<f64,String> {
        if self.radius<=0.0{
            return Err("半径只能大于0".to_string());
        }
      return   Ok(std::f64::consts::PI*self.radius*self.radius);
    }//////////////////////////
}
//正方形面积的具体算法实现/
impl Area for SquareArea{
    fn area(&self) -> Result<f64,String> {
        if self.side_length<=0.0 {
            return Err("正方形边长只能大于0".to_string())
        }
     return   Ok(self.side_length*self.side_length);
    }
}
//矩形面积的具体算法实现
impl Area for RectangleArea {
    fn area(&self) -> Result<f64,String> {
        if self.side_length<=0.0{
            return Err("矩形长边只能大于0".to_string());
        }
        if self.side_width<=0.0{
            return Err("矩形宽边只能大于0".to_string());
        }

        return    Ok(self.side_width * self.side_length);
    }
}
//三角形面积的具体算法实现,海伦公式
impl Area for TriangleArea{
    fn area(&self) ->Result< f64 ,String>{
        if self.a<=0.0{
            return Err("三角形边a只能大于0".to_string());
        }
        if self.b<=0.0{
            return Err("三角形边b只能大于0".to_string());
        }
        if self.c<=0.0{
            return Err("三角形边c只能大于0".to_string());
        }
        let p: f64 =(self.a+self.b+self.c)*0.5;
        let t: f64=p*self.a*self.b*self.c;
     return   Ok( t.sqrt())
    }
}
//使用泛型 打印面积
pub fn print_area<T: Area>(shape: &T){
    let type_name =std::any::type_name::<T>();
    match shape.area() {
        Ok(area) =>  println!(" {}的面积时 {:.4}",type_name,area),
        Err(e) => println!("计算某图形{}面积 : {}",type_name, e),
    }
}
//测试算法
#[cfg(test)]
mod tests_geometry {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle1 = CircleArea { radius: 0.0 };
        assert_eq!(circle1.area().unwrap_err() ,"半径只能大于0");
        let circle2 = CircleArea { radius: -20.4 };
        assert_eq!(circle2.area().unwrap_err() ,"半径只能大于0");
        let circle3 = CircleArea { radius: 5.0 };
        assert!((circle3.area().unwrap()  - 78.53).abs() < 1e-2);
    }

   #[test]
    fn test_triangle_area() {
       let triangle1 = TriangleArea {
           a: -4.0,
           b: 5.0,
           c: 6.0
       };
       assert_eq!(triangle1.area().unwrap_err() ,"三角形边a只能大于0");
        let triangle2 = TriangleArea {
            a: 4.0,
            b: -5.0,
            c: 6.0
        };
       assert_eq!(triangle2.area().unwrap_err() ,"三角形边b只能大于0");
       let triangle3 = TriangleArea {
           a: 4.0,
           b: 5.0,
           c: -6.0
       };
       assert_eq!(triangle3.area().unwrap_err() ,"三角形边c只能大于0");
       let triangle = TriangleArea {
           a: 4.0,
           b: 5.0,
           c: 6.0
       };
        assert_eq!(triangle.area().unwrap(), 30.0);
    }

    #[test]
    fn test_square_area() {
        let square1 = SquareArea {
            side_length: -12.4
        };
        assert_eq!(square1.area().unwrap_err(),"正方形边长只能大于0");
        let square = SquareArea {side_length: 12.4 };
        assert!((square.area().unwrap() - 153.76).abs() < 1e-4);
    }
    #[test]
    fn test_rectangle_area() {
        let rectangle1 =  RectangleArea {
            side_length: -12.4 ,side_width:89.0
        };
        assert_eq!(rectangle1.area().unwrap_err(),"矩形长边只能大于0");
        let rectangle2 =  RectangleArea {
            side_length: 12.4 ,side_width:-89.0
        };
        assert_eq!(rectangle2.area().unwrap_err(),"矩形宽边只能大于0");
        let rectangle = RectangleArea{side_length: 12.4 ,side_width:89.0};
        assert!((rectangle.area().unwrap() - 1103.6).abs() < 1e-4);
    }
}