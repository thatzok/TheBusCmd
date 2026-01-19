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


## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU General Public License](LICENSE) for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
