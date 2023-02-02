use std::collections::HashMap;

use zbus::{
    dbus_proxy, export::futures_util::TryStreamExt, zvariant::Value, Connection, MessageStream,
};

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
    proxy: &NotificationsProxy<'_>,
    title: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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

    connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus.Monitoring"),
            "BecomeMonitor",
            &(&[] as &[&str], 0u32),
        )
        .await?;

    let mut stream = MessageStream::from(connection);

    while let Some(msg) = stream.try_next().await? {
        // I couldn't get match rules working so doing an if condition here instead...
        if msg.interface().is_some()
            && msg.interface().unwrap() == "org.freedesktop.Notifications"
            && msg.member().is_some()
            && msg.member().unwrap() == "Notify"
        {
            dbg!(&msg);
            // TODO: Deserialize msg and check if it comes from MS Teams, if so republish it with sound
        }
    }

    Ok(())
}
