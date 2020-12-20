# disclaimer

I am not responsible for how you use this. You probably shouldn't.
I cannot gaurantee that you will not get banned from any services or
over purchase items that you cannot afford. I cannot gaurantee your 
personal information will not be comprimised by a bug or an adulterated 
version of this software. Use this at your own risk!


# run

get and run [chromedriver](#chromedriver)

[set items and configuration](#config)

[//]: # (build binary and make instructions)
run [cargo](#cargo)

# current state

## amazon

- :white_check_mark: purchases uncontested items 
- :x: purchases contested amazon items 
- :x: knows to refresh and submit form on amazon until out of stock 
- :x: reworked logic to be stateless 

## newegg

- :white_check_mark: purchases uncontested items 
- :x: purchases contested items 
- :x: knows when to stop after successful purchase 

# dev platform

Only tested on:

```macOS Catalina 

stable-x86_64-apple-darwin (default)

rustc 1.48.0 (7eac88abb 2020-11-16)```

# dev run

## requirements

### chromedriver

go to https://chromedriver.chromium.org/

download latest for your platform

run `chromedriver` or `chromedriver.exe`

### config 

edit `Joinem.toml`

#### newegg

set your newegg items using `[newegg_items]`

#### amazon

set your amazon items using `[items]`

### cargo

`cargo run`

[//]: # (build binary and make instructions)
[//]: # (FUTURE: dev install when using cargo-watch)
[//]: # (cargo install systemfd cargo-watch)
[//]: # (FUTURE: dev run when using cargo-watch)
[//]: # (systemfd --no-pid -s http::3030 -- cargo watch -x 'run')
