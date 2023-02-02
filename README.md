# Pokemon Stadium (US) ROM Mapping Tools

This is a simple script used for understanding the various binary formats in Pokemon Stadium 1 (US). So far, I've found four formats, as identified by their magic bytes: Fragments, PresJPEG, PERS-SZP, and Yay0.

## Basic Format Info 
### Fragments
These seem to be the same as [jrra documented for Stadium 2](https://jrra.zone/pokemon-stadium-2/#notes). These are relocatable code, data, and bss fragements.

### PRES-JPEG
A wrapper around a standard (not N64 format, at least) JPEG. There are identified by the magic eight bytes `PRESJPEG`. There is no size 

```rust 
struct PresJpeg {
    magic: [u8; 8] = "PRESJPEG",
    // to data from start of header
    offset: u32,
    unk: u32
}
```

### PERS-SZP

This looks like a wrapper for generic file data. Typically, the data is then compressed with `yay0`.

```rust
struct PersSzp {
    magic: [u8; 8], // "PERS-SZP"
    offset: u32,
    // RAM size after decompression?
    size: u32,
    size_again: u32,
    num_updates: u32,
    // (update_value, offset) (maybe?)
    updates: Vec<(u32, usize)>,
}
```

### YAY0

Compression format related to SM64's `mi0`. 

http://hitmen.c02.at/files/yagcd/yagcd/chap16.html
