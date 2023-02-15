<h3 align="center">svg2colored-png</h3>

<p align="center">
  SVG to PNG converter in all kinds of colors
</p>

## Install

You can install this in 3 ways:

 1) `cargo install svg2colored-png`
 2) Download the prebuilt binaries from [here](https://github.com/MCorange99/svg2colored-png/releases/latest)
 3) Build `svg2colored-png` yourself. Instructions [here](#building)

## Usage

To convert some SVG files in the directory `in`
to PNG in lets say black(#000000) and white(#ffffff) to the folder `out`, you should run this:

```sh
svg2colored-png -i ./in -o ./out -c ffffff,000000
```

And it will convert all svg files in `in/` to png files in `out/`

## Building

You will need one thing, rust, of course.
To install the compiler and its
tools please look at [this](https://www.rust-lang.org/learn/get-started) website,
and after you install it come back here.

Clone the repo to your desired folder

```sh
# if you have ssh set up on github and or want to contribute
git clone git@github.com:MCorange99/svg2colored-png.git 

# if you just want to build the project
git clone https://github.com/MCorange99/svg2colored-png.git
```

Make sure you are in the project directory.

```sh
cd svg2colored-png
```

And then build it!

```sh
cargo build --release
```

Your compiled executable should be in `[project_folder]/target/release/svg2colored-png`

## Contributing

Feel free to make a [pull request](https://github.com/MCorange99/svg2colored-png/pulls)
with any contributions you feel should be here!

## Licensing

This project uses the GPL-3.0 license, view it [here](/LICENSE.md)

## Authors

[MCorange](https://github.com/MCorange99) <<mcorangecodes@gmail.com>>
