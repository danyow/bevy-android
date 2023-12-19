#[no_mangle]
fn android_main(android_app: bevy::winit::AndroidApp) {
    let _ = bevy::winit::ANDROID_APP.set(android_app);

    println!("Starting launcher: Mobile");
    bevy_android::app(true).run();
}
