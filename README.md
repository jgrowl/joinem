<p align="center">
:warning: This project is NOT in a safe state to run if you do not 
know what you are doing :warning: 

:no_entry: No Scalpers! :no_entry:

Scalpers are not welcome to use this software. This is to level the field.
</p>

# Joinem

### A configurable bot that automatically buys items from newegg and amazon

> If you can't beat the em... Joinem

# disclaimer

I am not responsible for how you use this. You probably shouldn't.
I cannot guarantee that you will not get banned from any services or
over purchase items that you cannot afford. I cannot guarantee your 
personal information will not be compromised by a bug or an adulterated 
version of this software. Use this at your own risk!

# about

This is a bot :robot: that will refresh a product page on newegg or amazon at a
configurable rate. The bot will check to see if the item is in stock and 
that the price is below a configurable amount. If so, the item will be 
purchased by going through whatever process is specific to the vendor.
This requires being able to skip advertisements, surveys, insurance offers,
etc. The bot will use all default options set on each site so it is
important that the correct address and credit card is selected.

# configure

[set items and other configuration](#config)

# setup chrome data directories 

This is a work around for an annoying bug that I have to fix and it will
go away in the future. A copy (per bot) of your chrome data directory 
needs to be added to your JOINEM_DATA folder. This is set to a folder 
named `joinem_data` in the executable directory. 

Basically copy `~/Library/Caches/Google/Chrome` to `joinem_data` 10 times 
if you want the bot monitoring 10 different items.

You must login to the site you are botting before copying these directories.
These folders have your site cookies/cache and are what allow the bots
to be logged in without having to sign in every time for each one.

# run

get and run [chromedriver](#chromedriver)

[//]: # (build binary and make instructions)
run [cargo](#cargo)

# current state

Things are pretty rough right now. Many things probably don't work right.
Basically the app can buy regular items from stores, but actually getting
a 3080 would probably fail badly. Logic needs to be added that will figure 
out if something went wrong. Specifically when an item purchase is attempted
but the item does not actually go in the cart or a message pops up that says
the item is out of stock. With amazon the item will need to refresh and 
resubmit the form until out of stock for example.

Below are a list of things that really should work before this is anywhere
near safe to run. Green items should roughly work, but red are known
issues that will cause failure loops or keep buying over and over.

This list will probably grow:

## general

- [ ] fix chrome data directory failures

## amazon

- [x] purchases highly-available items 
- [ ] purchases low-availability items (ie. 3080)
- [ ] knows to refresh and submit form on amazon until out of stock 
- [ ] reworked logic to be stateless 

## newegg

- [x] purchases highly-available items 
- [ ] purchases low-availability items (ie. 3080)
- [ ] knows when to stop after successful purchase 

# future

## general

- [ ] adds browser-based ui 
- [ ] adds remote control 
- [ ] adds un-logged-in remote communication 
- [ ] provides better logging

# dev run

Tested on:

` Windows 10 Pro`

and

```
macOS Catalina 
stable-x86_64-apple-darwin (default)
rustc 1.48.0 (7eac88abb 2020-11-16)
```

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
