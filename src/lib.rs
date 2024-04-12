mod game;

use game::bevy_main;

use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};

#[no_mangle]
fn android_main(app: AndroidApp) {
    bevy_main();
}
