mod cmswitch;
use cmswitch::CmSwitch;

use dbus::Error as DbusError;

fn main() -> Result<(), DbusError> {
    let cmswitch = CmSwitch::new()?;
    cmswitch.arm()?;

    // Stop the program somehow
    // I'm intentionally making arm non-blocking for future use.

    cmswitch.disarm()
}
