# ttoe
Tick Tack Toe in your terminal.

## How to use
```sh
ttoe [-s <width>x<height>] [-w <win-length>]
```

This will start a new game with the given board size and win length.

## How it looks
This is how it loks with board size of 20x20 and win length of 5:
![image](https://github.com/user-attachments/assets/7246e9bd-e1ab-4a9a-9e8c-18e054eca6ea)

## More options
To see full help use:
```sh
ttoe -h
```

## How to get it

### Build from source
You need to have git, cargo (and rust toolchain) installed. Than you can
just type in your shell:
```sh
# Download from github into folder 'ttoe':
git clone https://github.com/BonnyAD9/ttoe.git
# Checkout to the latest release (this is optional, but you may have buggy
# version)
git checkout `git describe --tag --abbrev=0`
# Build ttoe:
cd ttoe
cargo build -r
# Now ttoe binary is located in './target/release/ttoe'. You can now move it into
# a system binary folder to have it available straight away:
sudo cp ./target/release/ttoe /usr/bin/ttoe
# Now you can just use ttoe:
ttoe -s 3x3
```
