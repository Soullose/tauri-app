fn main() {
  println!(r"cargo:rustc-link-search=C:\Windows\System32");
  tauri_build::build()
}
