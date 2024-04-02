'Set the directory to your direct path
'Required so that the shell can be ran from task scheduler
'This is the only variable that should need configuration in this script
Dim api_directory
api_directory = "C:\Users\BKBar\OneDrive\Documents\GitHub\home-inventory\inventory-api"

'Run the built executable
'Users will need to run {cargo build --release} before this can work
Dim WShell
Set WShell = CreateObject("WScript.Shell")
WShell.Run api_directory  & "\target\release\inventory-api.exe", 0
Set WShell = Nothing