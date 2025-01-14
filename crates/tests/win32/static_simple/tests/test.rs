// Remove target_arch when upstream metadata generator supports other targets
#![cfg(all(windows, target_arch = "x86_64", target_env = "msvc"))]

use test_win32_static_simple::*;
use windows::core::*;
use StaticComponent::Win32::Simple::*;

#[test]
fn test() -> Result<()> {
    unsafe {
        SimpleFunction();
        Ok(())
    }
}
