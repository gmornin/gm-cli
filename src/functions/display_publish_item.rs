use goodmorning_bindings::services::v1::V1TexUserPublish;

pub fn display_publish_item(item: &V1TexUserPublish) -> String {
    format!("{}. {} ({}):\n{}", item.id, item.title, item.ext, item.desc)
}
