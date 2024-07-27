use panic_ext::PanicInfoExt;

use std::panic;
use std::sync::OnceLock;

fn should_create_str_message() {
    panic!("just static string");
}

fn should_create_string_message() {
    panic::panic_any(format!("string with argument={}", 11));
}

const LOCATION: &'static panic::Location<'static> = panic::Location::caller();

fn should_handle_static_str_message() {
    static PANIC_INFO_MESSAGE: OnceLock<String> = OnceLock::new();
    let previous_hook = panic::take_hook();

    panic::set_hook(Box::new(|error| {
        let _ = PANIC_INFO_MESSAGE.set(error.panic_details().to_string());
    }));

    if let Err(error) = panic::catch_unwind(should_create_str_message) {
        panic::set_hook(previous_hook);
        let catch_details = (&*error).panic_details().to_string();
        let panic_message = PANIC_INFO_MESSAGE.get().expect("to set panic");

        let expected_message = format!("{}:7:5: just static string", LOCATION.file());
        assert_eq!(*panic_message, expected_message);
        let expected_message = format!("{}:26:39: just static string", LOCATION.file());
        assert_eq!(catch_details, expected_message);
    } else {
        panic!("Should panic!");
    }
}

fn should_handle_string_message() {
    static PANIC_INFO_MESSAGE: OnceLock<String> = OnceLock::new();
    let previous_hook = panic::take_hook();

    panic::set_hook(Box::new(|error| {
        let _ = PANIC_INFO_MESSAGE.set(error.panic_details().to_string());
    }));

    if let Err(error) = panic::catch_unwind(should_create_string_message) {
        panic::set_hook(previous_hook);

        let catch_details = (&*error).panic_details().to_string();

        let panic_message = PANIC_INFO_MESSAGE.get().expect("to set panic");
        #[cfg(feature = "alloc")]
        {
            let expected_message = format!("{}:11:5: string with argument=11", LOCATION.file());
            assert_eq!(*panic_message, expected_message);
            let expected_message = format!("{}:49:39: string with argument=11", LOCATION.file());
            assert_eq!(catch_details, expected_message);
        }

        #[cfg(not(feature = "alloc"))]
        {
            let expected_message = format!("{}:11:5: panic occurred", LOCATION.file());
            assert_eq!(*panic_message, expected_message);
            let expected_message = format!("{}:49:39: panic occurred", LOCATION.file());
            assert_eq!(catch_details, expected_message);
        }
    } else {
        panic!("Should panic!");
    }
}

#[test]
fn should_handle_panics() {
    should_handle_static_str_message();
    should_handle_string_message();
}
