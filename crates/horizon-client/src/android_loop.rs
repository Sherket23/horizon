//! One place that creates the window's event loop, so Android can hand it
//! the app that the system gave us.
use winit::{error::EventLoopError, event_loop::EventLoop};

#[cfg(target_os = "android")]
pub static ANDROID_APP: std::sync::OnceLock<winit::platform::android::activity::AndroidApp> =
    std::sync::OnceLock::new();

pub fn new_event_loop() -> Result<EventLoop<()>, EventLoopError> {
    #[cfg(target_os = "android")]
    {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        let app = ANDROID_APP.get().expect("Android app not set").clone();
        EventLoop::builder().with_android_app(app).build()
    }
    #[cfg(not(target_os = "android"))]
    {
        EventLoop::new()
    }
}
