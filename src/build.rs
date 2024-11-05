


fn main() {
  if cfg!(target_os = "windows") {
    extern crate winres;
    static_vcruntime::metabuild();
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.compile().unwrap();
  }
}

