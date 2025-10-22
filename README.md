# TheBus2Cmd, TheBusInfo

TheBus2Cmd and TheBusInfo are API clients (the API is called "Telemetry" in the game) for the bus simulator "**The Bus**" that allow you to send commands to your bus and query its status.<br>

The "Telemetry" feature must be enabled in the game's settings, and the programs must be running on the PC on which "**The Bus**" is running.

## Usage Example: TheBusCmd
In order to control the bus's settings, the game must be in "free play" and the player must be sitting in the driver's seat of the bus.

To toggle the parking brake status:
  ```sh
  TheBusCMD sendevent?event=FixingBrake
  ```

## Usage Example: TheBusInfo
The program runs in an endless loop and displays the changing bus values. It can be terminated with CTRL-C.

  ```sh
  TheBusInfo
  ```
If you don't want to compile the two EXE files yourself, you can also download just the two files **TheBusCmd.EXE** and **TheBusInfo.EXE** directly from the latest release package.

 Have fun!
