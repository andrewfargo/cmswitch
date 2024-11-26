# Charged Man's Switch

Shuts down the device when it is unplugged. 
Does nothing if the device is unplugged to start.

## Usage

Execution, typically by typing `cmswitch` into the terminal, will start the switch in the foreground. A SIGINT will stop it (^C).

To run in the background `cmswitch -d` or `cmswitch -d start` will run the switch as a daemon. `cmswitch -d stop` will stop it.

## Other Notes

* Yes, this program is written in rust. No, it doesn't matter thus.

* Until I delete this bullet, I wouldn't trust this program much.
