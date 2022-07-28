use cubeb::{Context, Result};
use std::env;
use std::ffi::CString;

pub fn init<T: Into<Vec<u8>>>(ctx_name: T) -> Result<Context> {
    // allow user to select backend via env var
    let backend = env::var("CUBEB_BACKEND").ok();

    // a little dance to convert to `Option<&CStr>`
    let backend_c = backend.clone().map(|s| CString::new(s).unwrap());
    let backend_c = backend_c.as_ref().map(|c| c.as_c_str());

    // setup context
    let ctx_name = CString::new(ctx_name).unwrap();
    let ctx = Context::init(Some(ctx_name.as_c_str()), backend_c);

    // alert when the prefered backend was not available
    if let Ok(ref ctx) = ctx {
        if let Some(ref backend) = backend {
            let ctx_backend = ctx.backend_id();
            if backend != ctx_backend {
                eprintln!("Requested backend `{}', got `{}'", backend, ctx_backend);
            }
        }
    }

    ctx
}
