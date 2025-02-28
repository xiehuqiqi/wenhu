use chrono::Local;

fn main() {
    let now = Local::now();
    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP={}",
        now.format("%Y-%m-%d %H:%M:%S %Z")
    );
    println!(
        "cargo:rustc-env=CARGO_PKG_LICENSE={}",
        env!("CARGO_PKG_LICENSE")
    );
}
