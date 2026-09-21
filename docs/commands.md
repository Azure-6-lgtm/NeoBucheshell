# Commands in NeoBucheshell
NeoBucheshell provides many in-built commands.
These commands are handled by the shell and not by external programs
There also some special commands seperate from NeoBucheshell's coreutils.

## In-built basic commands
- **touch**: It creates a new file.<br> ```touch {file path} ```
- **rm**: It deletes/removes a file.<br>
``` rm {file path} ```
- **rmdir**: It removes a directory <br>
``` rmdir {directory path}```
- **cp**: It copies a file from source to destination <br>
``` cp {source file} {destination} ```
- **mv**: It moves a file from source to destination <br>
``` mv {source file} {destination} ```
- **which**: Checks if a command is executable or not and returns its path <br>
``` which {command} ```
- **mkdir**: Creates a new directory <br>
``` mkdir {directory} ```
- **cd**: Changes current directory <br>
``` cd {directory path} ```
- **ls**: Lists the files and folders in a directory <br>
``` ls {directory} ``` #or blank to show current Dir 

## Special commands
- **sysinfo**: Shows system information
- **ps**: Shows the processes currently running
- **channel**: Used to join the NeoBucheshell REPL aka channel
``` channel join(channel number) ``` # Use 0 as running channel on multiple threads is still W.I.P

## Minor commands
- **about**: Gives a brief info about the shell
- **ver**: Displays the shell version
- **help**: Displays this file to help beginners learn
