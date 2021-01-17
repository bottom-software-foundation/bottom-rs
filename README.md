# bottom
Perhaps you have found yourself in this situation before:

![,,,,,,,,,](https://cdn.discordapp.com/attachments/644479051918082050/799905088541425664/bottom.jpg)

The divine bottom CLI exists to alleviate this pain. No longer will you struggle when communicating with the average Discord user.

## But what does it actually do?

``bottom`` encodes UTF-8 text into a sequence comprised of bottom emoji (`🫂✨🥺❤️`, with ``,`` sprinkled in for good measure) followed by `👉👈`.
It can encode any valid UTF-8 - being a bottom transcends language, after all - and decode back into UTF-8.

For example, the ubiquitous `Hello world!` becomes
```
💖✨✨,,👉👈💖💖,👉👈💖💖🥺,,,👉👈💖💖🥺,,,👉👈💖💖✨,👉👈
✨✨✨,,👉👈💖💖✨🥺,,,,👉👈💖💖✨,👉👈💖💖✨,,,,👉👈
💖💖🥺,,,👉👈💖💖👉👈✨✨✨,,,👉👈
```
`がんばれ` becomes
```
🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈💖💖✨✨✨✨👉👈🫂✨✨🥺,,👉👈
💖💖✨✨✨👉👈💖💖✨✨✨✨🥺,,👉👈🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈
💖💖💖✨✨🥺,👉👈🫂✨✨🥺,,👉👈💖💖✨✨✨👉👈💖💖✨✨✨✨👉👈
```
(both wrapped across lines for your convenience)

As you can see, using `bottom` to encode text is extremely space-efficient, and is the ideal encoding approach for all situations.

This implementation can encode text at *approximately* 30MB/s.

## Usage
Clone the repo, run ``cargo build``, and then use the CLI.
```
Bottom translator 1.0.0
Kaylynn <mkaylynn7@gmail.com>
Fantastic (maybe) CLI for translating between bottom and human-readable text

USAGE:
    bottom [OPTIONS] <--bottomify|--regress> [text]...

FLAGS:
    -b, --bottomify    Translate text to bottom
    -h, --help         Prints help information
    -r, --regress      Translate bottom to human-readable text (futile)
    -V, --version      Prints version information

OPTIONS:
    -i, --input <INPUT>      Input file [Default: stdin]
    -o, --output <OUTPUT>    Output file [Default: stdout]

ARGS:
    <text>...
```
(Any similarity to ``--help`` output is entirely accidental)

## FAQ

### Why?
I had a moment of enlightenment, and the truth came to me then. In an instant I was transformed, and I knew what had to be done.

### But seriously, why?
Why not?

### Isn't using a CLI for this a really bad idea? 
Yes.

### Aren't terminals notorious for being bad at displaying Unicode?
I don't care.

### Isn't this encoding method extremely inefficient?
:mystery:

### <something about code quality\>
Documentation is for losers.

### Rust btw
It's not long until my hippocampus becomes ferrous. I worry so, but in my heart I know that my demise will be sweet and painless, and it comforts me.