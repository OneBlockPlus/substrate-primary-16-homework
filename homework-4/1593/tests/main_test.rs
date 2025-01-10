use std::process::Command;

#[test]
fn test_main_output() {
    // 构建并运行二进制文件
    let output = Command::new("cargo")
        .args(["run", "--quiet"]) // `--quiet` 阻止 Cargo 输出构建信息
        .output()
        .expect("Failed to execute main binary");

    // 确保程序成功运行
    assert!(output.status.success());

    // 将输出转换为字符串
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // 检查是否包含预期输出
    assert!(stdout.contains("Red light duration: 60 seconds"));
    assert!(stdout.contains("Overflow occurred!"));
    assert!(stdout.contains("The area is: 78.54")); // 圆的面积
    assert!(stdout.contains("The area is: 6.00")); // 三角形的面积
    assert!(stdout.contains("The area is: 4.00")); // 正方形的面积
}
