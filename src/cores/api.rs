use std::panic;
use std::panic::UnwindSafe;
use actix_web::{HttpResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::models::response::response;

struct Api {}

pub fn handle<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> HttpResponse where R: Serialize + DeserializeOwned + Clone {
    let result = do_handle(f);
    return response::Response::from_result(&result).to_json_resp();
}

pub fn do_handle<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R, String> where
    R: Serialize + DeserializeOwned + Clone {
    let err_result = panic::catch_unwind(|| {
        return f();
    });

    return match err_result {
        Ok(res) => {
            Ok(res)
        }
        Err(err) => {
            let msg = panic_message::get_panic_message(&err);

            Err(msg.expect("REASON").to_string())
        }
    };
}

