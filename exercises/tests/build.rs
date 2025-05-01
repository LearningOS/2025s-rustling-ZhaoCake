//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.


use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
 // 获取当前时间戳（Unix时间），用于 tests7.rs
 let timestamp = SystemTime::now()
 .duration_since(UNIX_EPOCH)
 .unwrap()
 .as_secs();
    
 // 为 tests7.rs 设置环境变量 TEST_FOO
 // 这个环境变量在编译时设置，测试时会检查它是否在预期范围内
 println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
   // 为 tests8.rs 设置 feature 标志 "pass"
    // 这样 tests8.rs 中的条件编译会使用这个特性标志，避免测试失败
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
