use std::env;

fn main() {
  napi_build_ohos::setup();
  // Link ace_ndk.z which contains napi_module_register symbol
  println!("cargo:rustc-link-lib=dylib=ace_ndk.z");
  println!("cargo:rustc-link-arg=-Wl,-soname,libhello.so");
  
  // Enable GLES backend for blade-graphics
  println!("cargo:rustc-cfg=gles");
}
