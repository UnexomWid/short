# About <a href="https://www.rust-lang.org/"><img align="right" src="https://img.shields.io/badge/Rust-1.63-F74C00?logo=rust" alt="Rust 1.63" /></a>

**short** is a file organizer written in rust. It moves all files in a dir to subdirs based on categories and file extensions.

# Usage

```sh
cargo install --path .

./short

# You can also run it like this to change the working dir,
# but the config file must be in the current process WD!
./short <dir>
```

Short generates a default JSON config file `short.json` which you can modify.

You can create categories based on file extensions and names.

## Config

```js
{
    // If a file matches a category from here, it will be moved to:
    // <working_dir>/<category>
    "categories": {
        "Images": {
            "extensions": ["jpg", "png"]
        },
        "C++ Stuff": {
            "extensions": ["cpp", "cxx"],
            "filenames": ["CMakeLists.txt"] 
        }
    }

    // If a file doesn't fit any category given in this config (i.e. is uncategorized), don't move it anywhere;
    // This must be false for the options below to take effect
    "ignore_uncategorized": false,

    // If a file is uncategorized, move it to <uncategorized_dir>/<ext> where <ext> is the file extension;
    // otherwise, move to <uncategorized_dir>
    "sort_uncategoriezd_by_ext": true,

    // Where to move uncategorized files;
    // If empty, it's the working directory
    "uncategorized_dir": "Other Files",

    // Where to move uncategorized files that have no extensions;
    // This is <uncategorized_dir>/<no_extension_dir>
    "no_extension_dir": "No Extension"
}
```

# License <a href="https://github.com/UnexomWid/short/blob/master/LICENSE"><img align="right" src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>

[MIT](https://github.com/UnexomWid/short/blob/master/LICENSE).