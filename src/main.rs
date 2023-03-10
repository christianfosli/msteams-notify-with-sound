use std::{collections::HashMap, time::Duration};

use tracing::{event, span, Level};
use zbus::{
    dbus_proxy,
    export::futures_util::TryStreamExt,
    zvariant::{Str, Structure, Value},
    Connection, MessageStream,
};

#[dbus_proxy(assume_defaults = true)]
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
    replaces_id: u32,
    title: &str,
    body: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let proxy = NotificationsProxy::new(connection).await?;
    let id = proxy
        .notify(
            "msteams-notify",
            replaces_id,
            "mail-message-new",
            title,
            body,
            &[],
            HashMap::from([("sound-name", &Value::new("message-new-instant"))]),
            5000,
        )
        .await?;
    Ok(id)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let connection = Connection::session().await?;
    let connection2 = Connection::session().await?;

    connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus.Monitoring"),
            "BecomeMonitor",
            &(&[] as &[&str], 0u32),
        )
        .await?;

    let mut stream = MessageStream::from(&connection);
    let mut last_notification_id = 0;

    event!(Level::DEBUG, "Monitoring dbus");

    while let Some(msg) = stream.try_next().await? {
        // I couldn't get match rules working so doing an if condition here instead...
        if msg.interface().is_some()
            && msg.interface().unwrap() == "org.freedesktop.Notifications"
            && msg.member().is_some()
            && msg.member().unwrap() == "Notify"
        {
            let msg_fields = msg.body::<Structure>()?.into_fields();
            let app_name: Str = msg_fields[0].clone().try_into()?;
            let title: Str = msg_fields[3].clone().try_into()?;
            let body: Str = msg_fields[4].clone().try_into()?;

            if app_name == *"Google Chrome" && body.contains("teams.microsoft.com") {
                let span = span!(Level::INFO, "Republishing notification");
                let _enter = span.enter();

                tokio::time::sleep(Duration::from_secs(2)).await;

                let id = notify_send(
                    &connection2,
                    last_notification_id,
                    &format!("MS Teams: {title}"),
                    body.as_str(),
                )
                .await?;

                event!(Level::INFO, id = %id, app_name = %app_name, title = %title, "Republished notification");

                last_notification_id = id;
            } else {
                event!(Level::DEBUG, app_name = %app_name, title = %title, "Ignored notification");
            }
        }
    }

    Ok(())
}
