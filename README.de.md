# TheBus2Cmd, TheBusInfo

[English version](README.md)

TheBus2Cmd und TheBusInfo sind API-Clients (die API wird im Spiel "Telemetry" genannt) für den Bussimulator "**The Bus**", die es ermöglichen, Befehle an den Bus zu senden und seinen Status abzufragen.<br>

Die "Telemetry"-Funktion muss in den Einstellungen des Spiels aktiviert sein, und die Programme müssen auf dem PC laufen, auf dem auch "**The Bus**" ausgeführt wird.

## Download & Installation

1. Gehe zur Seite des [neuesten Releases](https://github.com/YourUsername/TheBusCmd/releases/latest).
2. Lade die Datei `TheBusCmd_x.x.x_Windows.zip` herunter.
3. Entpacke den Inhalt der ZIP-Datei in einen Ordner deiner Wahl.
4. Du findest dort zwei ausführbare Dateien: `TheBusCmd.exe` und `TheBusInfo.exe`.

## Anwendungsbeispiel: TheBusCmd
Um die Einstellungen des Busses zu steuern, muss sich das Spiel im Modus "Freies Spiel" befinden und der Spieler muss auf dem Fahrersitz des Busses sitzen.

Um den Status der Feststellbremse umzuschalten:
  ```sh
  TheBusCMD sendevent?event=FixingBrake
  ```

## Anwendungsbeispiel: TheBusInfo
Das Programm läuft in einer Endlosschleife und zeigt die sich ändernden Werte des Busses an. Es kann mit STRG-C beendet werden.

  ```sh
  TheBusInfo
  ```

Viel Spaß!


## Lizenz

Dieses Programm ist freie Software: Sie können es unter den Bedingungen der GNU General Public License, wie sie von der Free Software Foundation veröffentlicht wurde, entweder in Version 3 der Lizenz oder (nach Ihrer Wahl) in jeder späteren Version, weitergeben und/oder modifizieren.

Dieses Programm wird in der Hoffnung verteilt, dass es nützlich sein wird, aber OHNE JEGLICHE GEWÄHRLEISTUNG; sogar ohne die implizite Gewährleistung der MARKTGÄNGIGKEIT oder der EIGNUNG FÜR EINEN BESTIMMTEN ZWECK. Siehe die [GNU General Public License](LICENSE) für weitere Details.

Sie sollten eine Kopie der GNU General Public License zusammen mit diesem Programm erhalten haben. Wenn nicht, siehe <https://www.gnu.org/licenses/>.
