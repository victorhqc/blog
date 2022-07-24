use casbin::{prelude::*, Enforcer};
use std::{env, fs, str};

pub async fn init_enforcer() -> Result<Enforcer> {
    let model_payload = include_bytes!("../../config/model.conf");
    let model_str = str::from_utf8(model_payload).expect("Failed to load model.conf");

    let policies_payload = include_bytes!("../../config/policies.csv");
    let mut tmp_policies_dir = env::temp_dir();
    tmp_policies_dir.push("policies.csv");
    fs::write(&tmp_policies_dir, policies_payload).expect("Failed to write the policies.csv");

    let model = DefaultModel::from_str(model_str)
        .await
        .expect("Failed to load casbin model");

    let policies = FileAdapter::new(tmp_policies_dir);

    let enforcer = Enforcer::new(model, policies)
        .await
        .expect("Failed to init enforcer");

    Ok(enforcer)
}
