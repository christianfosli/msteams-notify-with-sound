use std::collections::HashMap;

use zbus::{dbus_proxy, zvariant::Value, Connection};

#[dbus_proxy]
trait Notifications {
    /// Call the org.freedesktop.Notifications.Notify D-Bus method
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

async fn notify_send(
    connection: &Connection,
    title: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let proxy = NotificationsProxy::new(&connection).await?;
    _ = proxy
        .notify(
            "msteams-notify",
            0,
            "mail-message-new",
            title,
            body,
            &[],
            HashMap::from([("sound-name", &Value::new("message-new-instant"))]),
            5000,
        )
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::session().await?;
    // TODO: Listen for Teams notification message on dbus, then peform below
    // See https://dbus.pages.freedesktop.org/zbus/client.html
    // and https://bazile.org/writing/2019/audible_gnome_notifications.html
    notify_send(&connection, "Hello from rust", "some body").await?;
    Ok(())
}
