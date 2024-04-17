#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    let mut bevy_app = bevy_in_app::init_bevy_game();
    bevy_app.run();
}
