### 作业说明
- 使用`cargo` 创建项目
- 编写红绿灯，三角形库，并添加单元测试
- 在main.rs中调用封装的类库函数，测试验证

- 单元测试输出
```rust
Executing task: C:\Users\0xDerick\.cargo\bin\cargo.exe test --package trafficlights --lib -- tests --show-output 

   Compiling trafficlights v0.1.0 (D:\Code\rust\hello-rust\trafficlights)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.84s        
     Running unittests src\lib.rs (target\debug\deps\trafficlights-d8dbaba9ff495138.exe)

running 4 tests
test tests::test_area_calculation ... ok
test tests::test_sum_u32_no_overflow ... ok
test tests::test_sum_u32_overflow ... ok
test tests::test_traffic_light_duration ... ok

successes:

successes:
    tests::test_area_calculation
    tests::test_sum_u32_no_overflow
    tests::test_sum_u32_overflow
    tests::test_traffic_light_duration

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

 *  Terminal will be reused by tasks, press any key to close it. 
```

- 调用执行输出
```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\trafficlights.exe`
Red light duration: 60 seconds
Yellow light duration: 5 seconds
Green light duration: 45 seconds
Sum of numbers: 15
Overflow occurred!
The area is: 28.274333882308138
The area is: 10
The area is: 36
```
