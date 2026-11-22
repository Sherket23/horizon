//! Android entry point: the system loads this library and calls android_main.
#![cfg(target_os = "android")]

use winit::platform::android::activity::AndroidApp;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let _ = horizon_client::android_loop::ANDROID_APP.set(app);
    horizon_client::entry::main();
}
