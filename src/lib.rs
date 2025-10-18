/// Registers a callback to be called when the iOS app enters background.
#[allow(dead_code)]
#[cfg(target_os = "ios")]
pub fn ios_reg_enter_background_observer(callback: fn()) {
    use std::ptr::NonNull;

    use block2::StackBlock;
    use objc2_foundation::{NSNotification, NSNotificationCenter, ns_string};

    unsafe {
        let block = StackBlock::new(move |_notification: NonNull<NSNotification>| {
            println!("UIApplicationDidEnterBackgroundNotification");
            callback();
        });

        let block = block.copy();

        let center = NSNotificationCenter::defaultCenter();

        let notification_name = ns_string!("UIApplicationDidEnterBackgroundNotification");

        let _ = center.addObserverForName_object_queue_usingBlock(
            Some(notification_name),
            None,
            None,
            &block,
        );
    }
}

#[allow(dead_code)]
#[cfg(not(target_os = "ios"))]
pub fn ios_reg_enter_background_observer(_callback: fn()) {}

/// Registers a callback to be called when the user takes a screenshot.
#[allow(dead_code)]
#[cfg(target_os = "ios")]
pub fn ios_reg_user_did_take_screenshot(callback: fn()) {
    use std::ptr::NonNull;

    use block2::StackBlock;
    use objc2_foundation::{NSNotification, NSNotificationCenter, ns_string};

    unsafe {
        let block = StackBlock::new(move |_notification: NonNull<NSNotification>| {
            println!("UIApplicationUserDidTakeScreenshotNotification");
            callback();
        });

        let block = block.copy();

        let center = NSNotificationCenter::defaultCenter();

        let notification_name = ns_string!("UIApplicationUserDidTakeScreenshotNotification");

        let _ = center.addObserverForName_object_queue_usingBlock(
            Some(notification_name),
            None,
            None,
            &block,
        );
    }
}

#[allow(dead_code)]
#[cfg(not(target_os = "ios"))]
pub fn ios_reg_user_did_take_screenshot(_callback: fn()) {}
