<!-- SPDX-FileCopyrightText: 2026 Twilit Jack <twilit.jack@proton.me> -->
<!-- SPDX-License-Identifier: CC-BY-NC-SA-4.0 -->

# Cadenza

Cadenza is an up-and-coming [free][Free Software] music notation editor, featuring
[Helix][Helix]-like music input, written in Go, based on [LilyPond][LilyPond].

It is meant to be an alternative to the paid options like Sibelius and Dorico, and
subsciption-ridden MuseScore. Furthermore, it will offer a Helix-like input mode which might be
unconventional for casual composers, but is so much faster to type in, it might reach similar
speeds to the MS-DOS program called [SCORE][SCORE].

**Cadenza is currently in early development, and there is little to show yet.**
I'm still fleshing out the internals, envisioning the UI, and actively developing it.

If you wish to learn a bit of backstory, the next section is for you. Otherwise, come back in
about 5–15 months and Cadenza will probably be something usable. :)

## A bit of backstory

I'm a young music composer and euphonium player, and I've always liked composing music.

I started music composition with MuseScore, as that's what I found at the time. A bit later, I
began to venture into the territories of NeoVim. I was very fond of the fast input you could achieve
with it, and wanted it everywhere. Including MuseScore, which is sadly lacking in keyboard speed.

Due to this distaste, I began looking for a solution. I found one. LilyPond.

It's a music engraving programming language, and I quite liked it. I dabbled in it for a bit, but
stayed a bit attached to MuseScore, as it's way faster and easier to orchestrate bigger musical
pieces in.

However, as the months grew, I became increasingly fed up with all of the paid things that MuseGroup
are pushing. Muse Sounds (which they launched as a free feature, however the free Muse Sounds are
now hidden away amongst the paid ones, and installing them on Linux is a bit tricky), the website,
unnessecary subscriptions, etc.

I wanted something else.

I came to discover that a lot of the other options for music composition and engraving were paid
options. Sibelius, Finale, Dorico. 

That wasn't what I wanted. I wanted freedom.

I began looking more and more into LilyPond, however, I became increasingly weary of the tough
points it faces with code organisation and dealing with many instruments at once. Then came things
like Makefiles to "alleviate" the problem of managing different parts, however, it was another
complex thing to add into the mix.

---

Now, I happen to be a jack-of-all-trades, and a part of that includes programming. And I saw this
situation, and began to feel a sense of urgency. A sense of necessity in the music notation world
for something new. That's why I began making Cadenza.

Since about the start of 2026, I've been programming and architecturing Cadenza. I've been doing it
with a few goals in mind:
- Creating a free, copyleft music editor
- Making score creation fast
- Sending a message

I want to iterate a bit more on those points. I'll start with number 2, though, as it's the most
straight-forward.

### Making score creation fast

There's been an MS-DOS program called [SCORE][SCORE], which contains a very archaic way of entering
notes, however, it's blazingly fast to write scores in it once you've got it down.

Cadenza aims to be similar, however, instead of using a custom archaic input method like SCORE, it's
aiming to instead use a more similar input method to the "post-modern" TUI text editor,
[Helix][Helix]. That's also in part because I use it myself.

### Creating a free, copyleft music editor

MuseScore is not free anymore. It's getting replaced by proprietary parts, and a subscription-based
model. Cadenza won't betray you like that. If you feel it is betraying you, create a fork.

However, I feel that this shouldn't be the case. I deeply care about music, and I want to create
something I want to use myself. I won't sell out Cadenza to a company, I won't let greed get the
better of this project. I want something usable.

### Sending a message

As previously mentioned, I don't like proprietary nonsense. The only way we'll progress as a society
is if we all get together, and don't be greedy.

I can only make something like this because I'm building on FOSS software myself. LilyPond, Helix,
Linux, Go, Fyne… I can go on for days. And for these gifts, I feel enticed to give something back.
And that is Cadenza. 

## Licensing

Copyright © 2026 Twilit Jack

Cadenza's code is licensed under the [GNU Affero General Public License v3.0 or later][AGPL].
I chose the AGPL, because of MuseScore's direction into a more cloud-based approach, which I'm
heavily against. For actually FOSS projects, I'm more than willing to allow use of my code.

Cadenza's assets, like images, documentation, etc., are licensed under the [Creative Commons
BY-NC-SA 4.0 license][CC BY-NC-SA 4.0]. You're free to use the assets as long as you follow the
[terms][CC BY-NC-SA 4.0] of the CC BY-NC-SA 4.0. 

If you have any questions about Cadenza or using it in different contexts, feel free to message me
on Codeberg (twilit_jack) or Matrix (@twilit.jack:matrix.org).


[Free Software]: https://www.gnu.org/philosophy/free-sw.html
[Helix]: https://helix-editor.com
[LilyPond]: https://lilypond.org
[SCORE]: https://en.wikipedia.org/wiki/SCORE_(software)
[AGPL]: https://www.gnu.org/licenses/agpl-3.0.en.html
[CC BY-NC-SA 4.0]: https://creativecommons.org/licenses/by-nc-sa/4.0/
