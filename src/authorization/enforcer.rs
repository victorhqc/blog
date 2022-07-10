use casbin::{prelude::*, Enforcer};

pub async fn init_enforcer() -> Result<Enforcer> {
    let model = DefaultModel::from_file("config/model.conf")
        .await
        .expect("Failed to load casbin model");

    let policies = FileAdapter::new("config/policies.csv");

    let enforcer = Enforcer::new(model, policies)
        .await
        .expect("Failed to init enforcer");

    Ok(enforcer)
}
