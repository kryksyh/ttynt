Output demo.gif

Require echo

Set Shell "bash"
Set FontSize 32
Set Width 1200
Set Height 600
Set Theme "Dracula+"

Type "Hi there!" Sleep 500ms  Backspace@50ms 9
Type "Basic ttynt usage" Sleep 500ms  Backspace@50ms 17
Type "cat sample.txt | ttynt hello rust" Sleep 500ms  Enter
Sleep 5s
Type "clear" Sleep 500ms  Enter
Type "cat log.txt | ttynt error warning info" Sleep 1500ms  Enter
Sleep 5s
Type "clear" Sleep 500ms  Enter Sleep 500ms

Type "You can highlight the whole line using -l " Sleep 2000ms Ctrl+C
Type "cat log.txt | ttynt error warning info -l" Sleep 2000ms  Enter
Sleep 6s

Type "clear" Sleep 500ms  Enter Sleep 500ms

Type "Or highlight the background with -b " Sleep 2000ms Ctrl+C
Type "cat log.txt | ttynt error warning info -b" Sleep 2000ms  Enter
Sleep 6s

Type "clear" Sleep 500ms  Enter Sleep 500ms

Type "Of course you can combine them " Sleep 2000ms Ctrl+C
Type "cat log.txt | ttynt error warning info -lb" Sleep 2000ms  Enter
Sleep 6s

Type "clear" Sleep 500ms  Enter Sleep 500ms

Type "And you can chain it too " Sleep 2000ms Ctrl+C
Type "cat log.txt |" Ctrl+V Ctrl+J Sleep 500ms
Type "ttynt error warning info |" Ctrl+V Ctrl+J Sleep 500ms
Type "ttynt deprecated -b" Sleep 2000ms  Enter
Sleep 6s

Type "clear" Sleep 500ms  Enter Sleep 500ms

Type "That's all folks, see you!"
Sleep 10s
