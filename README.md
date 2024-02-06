# About <a href="https://www.rust-lang.org/"><img align="right" src="https://img.shields.io/badge/Rust-1.63-F74C00?logo=rust" alt="Rust 1.63" /></a>

**short** is a file organizer written in rust. It moves all files in a dir to subdirs based on categories and file extensions.

# Usage

```sh
cargo install --path .

# Run short in the current dir
./short

# Run short in <working_dir>
./short <working_dir>
```

Short generates a default JSON config file `short.json` if it doesn't exist. Note that short searches for the config in the current working dir.

If you run `./short <working_dir>`, the config file will be `<working_dir>/short.json`.

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
    },

    // If an unsorted file is moved, but a file with the same name already exists in the destination,
    // then overwrite it;
    // If this is false, short will append numbers to the file name to solve the conflict
    "auto_overwrite": false,

    // If a file doesn't fit any category above (i.e. is uncategorized), don't move it anywhere;
    // This must be false for the options below to take effect
    "ignore_uncategorized": false,

    // If a file is uncategorized, move it to <working_dir>/<uncategorized_dir>/<ext>
    // where <ext> is the file extension;
    // otherwise, move to <working_dir>/<uncategorized_dir>
    "sort_uncategoriezd_by_ext": true,

    // Where to move uncategorized files; relative to <working_dir>
    // If empty, it's the working directory
    "uncategorized_dir": "Other Files",

    // Where to move uncategorized files that have no extensions;
    // This is <working_dir>/<uncategorized_dir>/<no_extension_dir>
    "no_extension_dir": "No Extension"
}
```

# License <a href="https://github.com/UnexomWid/short/blob/master/LICENSE"><img align="right" src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>

[MIT](https://github.com/UnexomWid/short/blob/master/LICENSE).