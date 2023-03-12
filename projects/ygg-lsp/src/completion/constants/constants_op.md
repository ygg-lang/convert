# WHITESPACE
Can be overwritten

```ygg
@overridable
WHITESPACE @= ASCII_WHITESPACE
ASCII_WHITESPACE @= 
    | \t       // TAB
    | \u{0020} // SPACE
```


# NEWLINE
Can be overwritten

```ygg
@overridable
NEWLINE @= ASCII_NEWLINE
ASCII_NEWLINE @=
    | \r ~ \n  // CRLF Windows
    | \n       // LF   Linux, MAC OSX
```

# ASCII_NEWLINE
New line in ASCII

```ygg
ASCII_NEWLINE @=
    | \r ~ \n  // CRLF Windows
    | \n       // LF   Linux, MAC OSX
```

# UNICODE_NEWLINE
New line in unicode

[UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-32.html) defines a number of characters that conforming applications should recognize as line terminators.

```ygg
UNICODE_NEWLINE @=
    | \r ~ \n  // CRLF
    | \r       // CR   Carriage Return
    | \n       // LF   Line Feed
    | \u{000B} // VT   Vertical Tab
    | \u{000C} // FF   Form Feed
    | \u{0085} // NEL  Next Line
    | \u{2028} // LS   Line Separator
    | \u{2029} // PS   Paragraph Separator
```

# ASCII_WHITESPACE
Whitespace in ASCII

```ygg
ASCII_WHITESPACE @= [ \t]
```

# UNICODE_WHITESPACE
Whitespace in Unicode

```ygg
[]
```



