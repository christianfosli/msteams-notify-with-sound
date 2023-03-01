# MS Teams notify with sound

**Problem**

Using the Microsoft Teams PWA on a Linux distrubution with Gnome,
it can be easy to miss notifications.

The built-in notifications (at least with PWA installed via Chrome) only appear for a short period of time,
and there is no sound when they appear, so unless you are constantly looking at the screen,
or constantly checking in MS Teams itself it's easy to miss something.

**Solution**

This rust-based console app listens to the notification on dbus and republishes the notification
with a notification sound and in a way that it doesn't dissapear from the notifications center right away.

:warning: **Work-In-Progress**

It seems to do its job now but there are some rough edges I'd like to improve still:

- [ ] The same notification sometimes appears twice. Check/fix.

- [ ] Maybe we should close the previous notification when a new one is recieved??

## Usage

* Install rust/cargo

* Clone this repo

* Slightly adjust src/main.rs if you need to,
  for example if you installed the Teams PWA with Microsoft Edge instead of Google Chrome
  you'll want something like this

  ```patch
  diff --git a/src/main.rs b/src/main.rs
  index 31ad89c..3c8fcef 100644
  --- a/src/main.rs
  +++ b/src/main.rs
  @@ -76,7 +76,7 @@ async fn main() -> Result<(), Box<dyn std::error::Error>> {
               let title: Str = msg_fields[3].clone().try_into()?;
               let body: Str = msg_fields[4].clone().try_into()?;
 
  -            if app_name == Str::from("Google Chrome") && body.contains("teams.microsoft.com") {
  +            if app_name == Str::from("Microsoft Edge") && body.contains("teams.microsoft.com") {
                   println!("Republishing notification \"{}\" in 2 seconds", title);
                   tokio::time::sleep(Duration::from_secs(2)).await;
  ```

* Install with cargo (by default to ~/.cargo/bin)

  ```
  cargo install --locked --path .
  ```

* Run with `RUST_LOG=Debug msteams-notify`

  * This runs the msteams-notify binary we installed on the previous step (assuming ~/.cargo/bin is available in your $PATH).
    You can also use standard cargo commands like `cargo run` to run it straght from the repo without "installing" it first.

* OR Run it in the background automatically with systemd. E.g.

  ```sh
  systemctl --user enable ./msteams-notify.service
  systemctl --user start msteams-notify
  ```

  In case of issues enabling the service, see [#1](https://github.com/christianfosli/msteams-notify-with-sound/issues/1)

  * Troubleshooting the systemd service:

    ```sh
    systemctl --user status msteams-notify
    journalctl --user -u msteams-notify
    ```
