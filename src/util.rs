use crate::statics::{APP_VERSION_MAP, MODEL_LIST, VERSION_MAP};

#[derive(Debug)]
pub struct Info {
    pub app_version: (&'static str, &'static str),
    pub user_id: String,
    pub user_agent: String,
    pub device: String,
}

pub fn generate_randam_info() -> Info {
    let version = VERSION_MAP[rand::random_range(0..VERSION_MAP.len())];
    let build = version.builds[rand::random_range(0..version.builds.len())];
    let model = MODEL_LIST[rand::random_range(0..MODEL_LIST.len())];
    let device = format!("{}.{}", version.sdk, model);
    let user_agent = format!(
        "Dalvik/2.1.0 (Linux; U; Android {}; {}/{})",
        version.id, model, build
    );
    let app_version = APP_VERSION_MAP[rand::random_range(0..APP_VERSION_MAP.len())];
    let hex = b"0123456789abcdef";
    let user_id = (0..32)
        .map(|_| hex[rand::random_range(0..hex.len())] as char)
        .collect();

    Info {
        app_version,
        user_id,
        user_agent,
        device,
    }
}
