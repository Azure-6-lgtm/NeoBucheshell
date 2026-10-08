# Configuration in NeoBucheshell

Configuration is an important aspect of NeoBucheshell.<br>
By default, The config file is **~/.neobucherc** <br>
**Note:** As of V1.1.0, Windows configuration support is limited but possible

## Configuring the prompt

NeoBucheshell's prompt can be compared to using lego bricks <br>
In NeoBucheshell, Prompt variables are used to customize the prompt <br>
They Include ->
- **$USER**: This variable fetches the username of the user running the shell
- **$HOST**: This variable fetches the hostname of the device running the shell
- **$PWD**:  This variable fetches the current directory
- **$TIME24**: This variable fetches the local time of the device in 24 hour format
- **$TIME12**: This variable fetches the local time of the device in 12 hour format
- **$NEWLINE**: This variable adds a newline in the prompt
- **$SHELLVERSION**: This variable fetches the version of NeoBucheshell
- **$OS**: This variable fetches the Operating system

## Prompt Configuration Syntax

The syntax for customizing the prompt is very simple <br>
```prompt = {insert your prompt template}```
For example <br>
```prompt = [$TIME]$PWD>>```
Neobucheshell will display the brackets and symbols normally <br>
**Note:**You can color the prompt by using ANSI escape codes

## Setting Aliases

Just like prompt. Aliases can be set in **~/.neobucherc** <br>
These Aliases are fundamentally similiar to the aliases in Bash or Fish but the syntax is different

## Alias Configuration Syntax
The syntax for configuring Alias is ->
```alias {name of command} = {command that will be actually executed}```
For example <br>
```alias la = ls -la```
