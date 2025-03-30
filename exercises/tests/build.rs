//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的UNIX时间戳（秒）
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取时间失败")
        .as_secs();
    
    // 使用Cargo的特殊println!命令设置环境变量TEST_FOO
    // 这会告诉Cargo设置环境变量，格式为：cargo:rustc-env=VAR=VALUE
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 让构建脚本在时间戳变化时重新运行
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
