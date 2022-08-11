## Prowl Rust API
A library with some structure around the [Prowl API.](https://www.prowlapp.com/api.php) This is an alternative to things like PushBullet or PushOver, using a [one time fee](https://apps.apple.com/us/app/prowl-easy-push-notifications/id320876271) instead of subscription based model. Additional nicities are being able to set notification priorities and a generous default of 1,000 API calls per hour with the option to allow-list for more.

### Example
After getting your Prowl API Key from [https://www.prowlapp.com/](https://www.prowlapp.com/). You can start sending notifications to devices you are logged into,

From `examples/add.rs`:
```rust
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
```
