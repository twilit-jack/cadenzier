<!-- SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me> -->
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# CDNZ

CDNZ is an open data format for storing music, primarily intended for use by [Cadenza][Cadenza], but
can also be used elsewhere. This crate is both a Rust implementation, and a specification.

The format is partly inspired by [MNX][MNX] and [LilyPond][LilyPond], and can be converted to
LilyPond code using this lib. Cadenza can also convert it to LilyPond and then PDF, SVG, and MIDI.

## Specification

This crate also partly functions as the specification of the CDNZ format.

In the future, a solid specification will be provided. In the meantime, the Rust documentation will
have to suffice.

## License

The CDNZ spec and docs, and this Rust implementation, are both licensed under either of:

- Apache License, Version 2.0 (<https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (<https://opensource.org/licenses/MIT>)

at your option.


[Cadenza]: https://codeberg.org/twilit-jack/cadenza
[MNX]: https://github.com/w3c-cg/mnx
[LilyPond]: https://lilypond.org/
