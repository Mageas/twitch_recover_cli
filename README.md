# Twitch Recover Cli

Inspired by [twitch_recover](https://github.com/pravindoesstuff/twitch_recover).

Twitch Recover is a free tool that allows you to recover direct m3u8 links (Working with sub only VODs).

## **How to use**

``` text
Usage: twitch_recover_cli <COMMAND>

Commands:
  twitch-tracker  Recover the vod from twitchtracker (alias: tt)
  manual          Manually recover the vod
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## **Build instructions**

Clone the repository:
```
git clone https://gitea.heartnerds.org/Mageas/twitch_recover_cli
```

Move into the project directory:
```
cd twitch_recover_cli
```

Build the project with cargo:
```
cargo build --release
```

The binary is located in `./target/release/twitch_recover_cli`
