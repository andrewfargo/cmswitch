# Charged Man's Switch

Shuts down the device when it is unplugged. 
Does nothing if the device is unplugged to start.

## Usage

Execution, typically by typing `cmswitch` into the terminal, will start the switch in the foreground. A SIGINT will stop it (^C).

## Future Directions

Eventually, I want to turn this into its own D-Bus object so that other processes can interact with it.
Perhaps it will allow for registering signals, arming and disarming, and manual tripping.

## Other Notes

* Yes, this program is written in rust. No, it doesn't matter thus.

* Until I delete this bullet, I wouldn't trust this program much.
