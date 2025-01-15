/*
实现一个函数，为 u32 类型的整数集合求和，参数类型为 &[u32]，
返回类型为 Option，溢出时返回 None
*/

pub mod set_sum{

  pub fn int_set_sum(ints:&[u32])->Option<u32>{
       let mut sum:u32 = 0;
       for i in 0..ints.len(){
         sum = sum + ints[i];
       }
       if sum>0{
        Some(sum)
       }else{
        None
       }
   }

}