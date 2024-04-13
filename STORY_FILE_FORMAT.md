This is the file format standard of the RPG stories.

An example of a file is:
```
version=0.1 write_speed=10ms write_mode=char
1;~health~=100
2;~mana~=100
3;>Hello traveller!
4;>What is your name? (1-15 characters)
5;~name=r/\w{1,15}/
6;>Ah, the [brilliant;wonderful;excellent] {name}! Am I saying that right? (yes/no)
8,7;[yes;no]
4;>Oh, sorry!
9;>Well {name}, I have a quest for you...
```

# File preamble/variables
The variables / preamble are on the first line, space seperated.
Version is **required**.

|Variable Name|Values|Example|
|---|---|---|
|version|0.1|`version=0.1`|
|write_speed|xxxxms|`write_speed=10ms`|
|write_mode|char/word/line|`write_mode=char`|

# Content lines
The content lines order is important as the line number is the node number starting from 0.
I.e. line 2 is node #0.

The first portion of the line is the next nodes as comma seperated numbers terminated by a semicolon.
I.e: `1,2,3;`

The second portion of the line is the actual line content straight with no leading space.

Example of some content lines are:
```
5;~name=r/\w{1,15}/
6;>Ah, the [brilliant;wonderful;excellent] {name}! Am I saying that right? (yes/no)
8,7;[yes;no]
```
