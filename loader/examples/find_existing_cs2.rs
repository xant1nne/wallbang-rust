// Read-only discovery smoke test. Does not open a mapping/injection handle.
fn main() {
    println!(
        "CS2 discovery: {:?}",
        wallbang_loader::find_process::find_existing("cs2.exe")
    );
}
