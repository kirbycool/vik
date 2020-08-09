# Vik

*WIP*

Vik(irby) is a vi like text editor written in rust.

## Building

You should be able to build Vik with standard `cargo` tooling.

For a debug build:
```
cargo run
```

For a release build:
```
cargo run build
```

## Goals

The primary goal of Vik is to teach me about building a text editor. However, I'd like to get it to a point where it's usable for some of my daily editing tasks and eventually some coding.

Features I'd like to implement:
- **Basic motions** - Start with the vim motions that I use in my editing flow.
- **Text Objects**
- **Registers** - Enough for common yank/paste operations
- **Syntax Highlighting** - Highlighting engine and maybe tree-sitter integration ala neovim.

### Non-Goals

There's some features I'm not going to prioritize
- **Broad terminal support** - If it works in my terminal, it's good enough. I don't want to get in the weeds of supporting the endless variety of old terminals
- **Macros** - Maybe at some point, but currently I don't use macros very often
- **Config** - This will probably move to a goal as the project continues, but I want to focus on core editor features rather than embedding a scripting language.
- **Plugins** - Maybe I'll want a feature that makes more sense as a plugin, but who else would use this?

## Implementation Notes

This uses the [tui crate](https://github.com/fdehau/tui-rs) to handle drawing the ui in the terminal.

For simplicity, I've implemented the text buffer as a `String` while I figure out the editing basics, but I'd like to rewrite this to a piece table for performance (and curiosity) reasons.
