# Output Values

Kleene Purple: `#211930`
Foreground color: `#F4F4F4`
Cursor: `rgba(0,255,0,0.5)`, `#00BB00`
- A higher opacity than `0.33` makes dark grey hard to read
- But `0.33` makes the cursor hard to find, try `0.5`

```
#000000 : Black
#CC6666 : Red
#66CC66 : Green
#CCCC00 : Yellow
#6689CC : Blue
#CC66CC : Magenta
#00CCCC : Cyan
#FFFFFF : White
#8B8680 : Bright Black
#EF8989 : Bright Red
#89EF89 : Bright Green
#EFEF00 : Bright Yellow
#89ACEF : Bright Blue
#EF89EF : Bright Magenta
#00EFEF : Bright Cyan
#AEA9A3 : Bright White
```

# ANSI 256 Versions

```
#000000: Black
#D75F5F: Red
#5FD75F: Green
#D7D700: Yellow
#5F87D7: Blue
#D75FD7: Magenta
#00D7D7: Cyan
#D7D7D7: White
#878787: Bright Black
#FF8787: Bright Red
#87FF87: Bright Green
#FFFF00: Bright Yellow
#87AFFF: Bright Blue
#FF87FF: Bright Magenta
#00FFFF: Bright Cyan
#FFFFFF: Bright White
```

Bright white and white are consistently problematic, `tig` uses white to mean the selected commit, bright makes sense as highlighted, but Emacs seems to use bright white for `#FFFFFF`, which it's strange if it's not also set to `#FFFFFF`.

Updating these yet again after noting that all builtin Terminal and iTerm colors have bright white lighter than white. So what we've done is increase the brightness of `White` and have `Bright White` be `#FFFFFF`.
