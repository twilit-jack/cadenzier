# Why does Cadenza use its own proprietary format, CMXL?

The choice of using a proprietary format for Cadenza was a heavy one, but there is merit to this
choice, and it's not just "I want people to be locked into my ecosystem" (which is also not fully
true, Cadenza supports various export options, and I also deeply care about preventing vendor
lock-in).

While making Cadenza's internal logic use MusicXML and then not even save projects as `.mxl` might
seem silly or even mischevious, there are a few good reasons behind this.

## 1. MusicXML is a general format, which restricts me in doing whatever I want with it.

Cadenza, like any other music notation editor, has its own metadata to store. However, this metadata
is in neither the `.mxl` nor the `.musicxml` spec. And as a result, Cadenza can't open any random
`.mxl` file and immediately assume that Cadenza things are going to be inside.

Furthermore, you, as the user, can't look at a `.mxl` file and immediately conclude if it's a
general MusicXML file, or a Cadenza file. While a different file extension can help, that's not as
clean (other apps would need to know that `.cmxl` files are the same `.mxl` format they support),
or as robust (you can freely remove or change the file extension of any file).

## 2. Different name, different mindset.

If we look at less tech-savvy people, it's not always easy to get the difference between MusicXML
and Cadenza MusicXML if they're basically the same. They'll assume that any MusicXML file contains
everything Cadenza has to offer (e.g. author metadata, artist's statement). Furthermore, it makes
them assume that those same features are accessible in other apps, as it's MusicXML, the format that
can be opened by every music notation app.

`.cmxl` makes them think: "Okay, this is Cadenza's format, this isn't MusicXML." which for someone
who doesn't need or want to understand Cadenza's inner workings is completely fine.

## 3. I just like zstd and tar more than zip.

This is just a preference thing. CMXL uses a zstd-compressed archive, while MXL uses a zip archive.
Now it's not a pure preference thing, as zstd does come with some advantages over the deflate
algorithm that zip uses. Mainly, it's faster and results in better compression rates. And if I'm
going to make my own format, it's going to use the better stuff. I don't need compatibility with
other apps (what MusicXML was made for), I just need something actually good.

## But this doesn't even matter!

Cadenza allows you to choose the MXL format if you need it, so you can stop raging about vendor
lock-in. You can export to MXL or uncompressed MusicXML, too. So in the end, it's a matter of
preference.
