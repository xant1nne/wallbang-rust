// Read-only Windows media-session diagnostic. Does not attach to CS2.
fn main() {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        if let Some(track) = wallbang_render::music::snapshot() {
            println!("Spotify: {} / {}; playing={}; position={:.1}/{:.1}; cover={}",
                track.artist, track.title, track.playing, track.position, track.duration, track.art.is_some());
            break;
        }
        if std::time::Instant::now() >= deadline {
            println!("Spotify has no available Windows media session");
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    wallbang_render::music::shutdown();
}
