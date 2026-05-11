<!-- SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me> -->
<!-- SPDX-License-Identifier: LGPL-3.0-or-later -->

# CDNZ

CDNZ is a work-in-progress open data format for storing music, primarily intended for use by
[Cadenza][Cadenza], but you're free to use it for your own needs. This crate is the main
implementation of CDNZ, and the specification is actively changing along with it. Once development
largely settles, and the main ideas and opinions of the format are set, a formal specification will
be published.

## Format Description Draft

The format is partly inspired by [MNX][MNX] and [LilyPond][LilyPond], and it will be possible to
convert it to LilyPond in the future, via this crate.

I've spent a good while thinking about the fundamental problem of how we store music, and what
implications it brings. As of now, CDNZ is aiming to be an opiniated music notation format, carrying
a split between what the music *is*, and how the music is *shown*.

It uses a structured, hierarchical model, while still allowing for complex additions that challenge
modern attempts, following in the footsteps of what LilyPond set out to create.


[Cadenza]: https://codeberg.org/twilit-jack/cadenza
[MNX]: https://github.com/w3c-cg/mnx
[LilyPond]: https://lilypond.org/
