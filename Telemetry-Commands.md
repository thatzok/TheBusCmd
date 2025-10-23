# "The Bus" Telemetry Bus Commands

| Command                                 | Key     | Info                    | Values |          
|-----------------------------------------|---------|-------------------------|--------|
| sendevent?event=ToggleDoorClearance     | KeyDown | Door Clearance          |        |
| setbutton?button=<DoorSelector>&state=1 | KeyDown | Doors                   |        |
| setbutton?button=<DoorSelector>&state=0 | KeyUp   | Doors                   |        |
| sendevent?event=<CashChangeSelect>      | KeyUp   | Cash Change             |        |
| sendeventpress?event=SetGear<g>         | KeyDown | D,N,R Gear Select       | D,N,R  |
| sendeventrelease?event=SetGear<g>       | KeyUp   | D,N,R Gear Select       | D,N,R  |
| sendeventpress?event=MotorStartStop     | KeyDown | Motor Start Button Down |        |
| sendeventrelease?event=MotorStartStop   | KeyUp   | Motor Start Button Up   |        |
| sendevent?event=FixingBrake             | KeyDown | Parking brake           |        |





