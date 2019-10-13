# TODOs 
The things that still need to be done.



SQL Tables

Device :: Table
id : number
name : string


Button :: Table
id: number
name: string
pulses: ???
device: number -> Remote.id


Sequence :: Table
id: number
name: string
steps: ??? (It needs to be a sequence of steps....)









IR-Signal Serialization:

What are we recording: 
- we are recording a de-modulated signal from an IR sensor
- we have the amount of time between on and offs

What data is needed?
- duration of on and off (overarching)


File write out considerations:
- we will have to be able to write out a lot or differnt buttons
- we will have to be able to write out a lot of different controllers
? Do we want to be able to SCP these back to the dev machine? 
  ? Why would we need to do this?

Date storage options: 
- SQL lite DB
- Files (real mess)

Iterations:
0: play back a recorded IR button
1: be able to play back more then one IR button
2: Be able to 




## Soon
- Build simple recorder (record IR signals)

## Not so soon
- 

## Maybe never...
- Make SSH configuration for run.sh