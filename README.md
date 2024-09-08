# Str to Keypad

A quick rust program to convert a string to a keypad number sequence. (1 = nothing, as that is historically voicemail, 2 = ABC, 3 = DEF, etc.)

Space is interpreted as a 0, and numbers are interpreted literally (1 = 1, 2 = 2, etc)

# How to use
To use the program, just call it while passing the string as the variable

**Ex:**
```
>str-to-keypad.exe abc123
```
Returns you
```
222123
```
