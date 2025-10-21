# TheBus Telemetry Bus commands

| <command>                                             | Key     | Info                    | Values |          
|-------------------------------------------------------|---------|-------------------------|--------|
| sendevent?event=ToggleDoorClearance                   | KeyDown | Door Clearance          |        |
| setbutton?button=<DoorSelector>&state=1               | KeyDown | Doors                   |        |
| setbutton?button=<DoorSelector>&state=0               | KeyUp   | Doors                   |        |
| sendevent?event=<CashChangeSelect>                    | KeyUp   | Cash Change             |        |
| setbutton?button=GearSwitch&state=<GearSelection - 1> | KeyUp   | D,N,R Gear Select       |        |
| sendeventpress?event=MotorStartStop                   | KeyDown | Motor Start Button Down |        |
| sendeventrelease?event=MotorStartStop                 | KeyUp   | Motor Start Button Up   |        |
| sendevent?event=FixingBrake                           | KeyDown | Fixing Brake            |        |





