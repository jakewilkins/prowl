#[tokio::main]
async fn main() {
    let ice_cube_reciepe = "".to_owned()
        + "Empty the ice cubes that are left in the trays (if there are any left) into the bin."
        + "Take the trays over to the sink and fill them with cold water. (Hot water will freeze faster and more clear)."
        + "Place the water filled ice trays back in the freezer."
        + "Replace the ice bin if you had to remove it."
        + "Shut the door to the freezer."
        + "Be sure to leave for around 4-6 hours at least to make sure it is frozen."
        + "If you want to experiment, you can freeze things like fruit infused waters or juices."
        + "\n\n"
        + "from https://www.food.com/recipe/ice-cubes-420398";

    let notification_description = format!("The automated system has detected low ice cube count. Please order or make more! The reciepe is as follows:\n\n{ice_cube_reciepe}");

    let notification = prowl::Notification::new(
        vec!["REPLACE-ME-WITH-YOUR-PROWL-API-KEY".to_string()],
        Some(prowl::Priority::VeryLow),
        Some("https://www.food.com/recipe/ice-cubes-420398".to_string()),
        "My Rust Sample".to_string(),
        "Low Ice Cubes".to_string(),
        notification_description,
    )
    .expect("Invalid notification");

    notification
        .add()
        .await
        .expect("Failed to send notification");
}
