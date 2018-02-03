# Kilac wire protocol (kwp)  
This document specifies the Kilac wire protocol (kwp).
Kwp is used in the talking between Kilac and other programs.

## Basics
Kwp is transmitted between websockets.
When sending command to Kilac, Kilac returns a command id first.
`cmd <id> queued`
When Kilac has processed the command, it transmits message in form
`id <id> <message>`.
Every command and returning message is terminated with newline `\n`.

Errors are written in form `error <code> <explanation>`.
Explanation may not be transmitted.
Error codes are listed in the following table

## Initiating connection
Command
```
new connection db <dbtype> addr <addr>
```
returns following
```
connection estb pid <id>
```
or error.
Field `pid` is a permanent id that can be used to identify later to continue connection.
If frequent user connection can also be initiated with command
```
new connection pid <id>
```

## Info about connection
Command
```
info
```
returns information about connection
```
info <time open> <first connected> <pid>
```

## Calculating
Command
```
calculate <comp> (<series>) (<task>)
```
calculates specified elements. Fields `series` and `task` are optional.

## Verifying scripts
Command
```
verify <comp> <series> <task> (<subtask>)
```
Verifies script and returns either compiled message or error.
