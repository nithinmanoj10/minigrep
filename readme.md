![minigrep banner](https://github.com/nithinmanoj10/minigrep/blob/master/minigrep.png?raw=true)

A lightweight and easy to use grep tool. It is an extension to the minigrep tool made in [The Book](https://doc.rust-lang.org/book/) and is also based on the GNU/Linux grep tool.

# Installation

## Installing via Crates.io

If you are familiar with Rust, then installing modules via crates.io isn't something new to you.

If you are an absolute beginner and need guidance in using cargo, please check out this [tutorial](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

If you want to know how to install packages using the crates ecosystem, check out this [tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#using-a-crate-to-get-more-functionality)

Click [here](https://crates.io/crates/minigrep_npm) to view the minigrep crate and install it in your project folder

## Installing via GitHub Releases

Download the latest binary file from the release page. Then add the binary file to the directory in your system were you would like to use this application.

# Getting Started

Too see if everything is installed and working properly, run the following

```
cargo run minigrep -v
```

This would output the current version of minigrep installed.

If you have installed via GitHub releases, then replace `cargo run` with `./minigrep` for all the commands being shown. So the above command would look like this

```
./minigrep minigrep -v
```

Running the following will display the help menu

```
cargo run minigrep_help
```

# Usage

```
cargo run [QUERY] [FILE_NAME] [OPTIONS]
```

This will search for QUERY i.e a word in the FILE_NAME provided. For example

poem.txt

```
I'm Nobody! Who are you?
Are you – Nobody – too?
Then there's a pair of us!
Don't tell! they'd advertise – you know!

How dreary – to be – Somebody!
How public – like a Frog –
To tell one's name – the livelong June –
To an admiring Bog!

```

```
cargo run the poem.txt -n
```

Running the above command will give us

```
poem.txt:
2:  Then there's a pair of us!
3:  Don't tell! they'd advertise – you know!
7:  To tell one's name – the livelong June –
```

## Options

The following options or flags can be used to get more refined outputs

`-i | --ignore-case` will ignore case distinctions

`-n | --line-numbers` will output line numbers along with output lines

`-c | --query-count` will output total occurences of query

`-lc | --line-count` will output total count of lines containing query

`-I | --invert-match` will output non-matching lines

## Miscellaneous

`cargo run minigrep -v | --version` outputs minigrep version

`cargo run minigrep [FILE_NAME] -S | --stats` prints tabulated file stats

`cargo run minigrep_help` displays help screen

# Further Contribution

Any help or contribution from the Open Source Community is highly appreciated.
