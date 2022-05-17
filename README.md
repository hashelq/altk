# Description
ALTK is a application that helps you insert unicode characters through macroses without changing your input method.
ALTK uses numpad number keys (Numpad0, Numpad1 .. Numpad9) to insert decimal number and NumpadPlus key to convert the input into a character.

It can be illustrated so:
```
some text, (user presses Numpad9, Numpad5, Numpad5, NumpadPlus in a order)λ blah blah
```

# Usage
1. You need to disable the numpad keys, you can use this shell script for this:
```
for i in 90 87 88 89 83 84 85 79 80 81 86; do
  xmodmap -e "keycode $i = "
done
```
2. Simply run the application.
3. Gratz, enjoy.

# License
```
            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
```
