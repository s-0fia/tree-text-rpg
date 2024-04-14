version=0.1 write_speed=10ms write_mode=char
1;~health~=100
2;~mana~=100
3;>Hello traveller!
4;>What is your name? (1-15 characters)
5;~name=r/[a-zA-Z]{1,15}/
6;>Ah, the [brilliant;wonderful;excellent] {name}! Am I saying that right? (yes/no)
8,7;[yes;no]
4;>Oh, sorry!
;>Well {name}, I have a quest for you...
