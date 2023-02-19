# cwd

Current Working Directory, a convenience crate

Instead of copy-pasting this bit of code yet again when I need the fully
qualified current working directory, I decided to turn it into a convenience
crate in the name of code reuse.

## Example

```
use cwd::cwd;

fn main() {
    println!("The current working directory is '{}'", cwd());
}
```

## Caveats

This is best effort. If it can't find the fully qualified current working
directory for some reason, it will fallback to "."

# Support

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/cwd/issues](https://github.com/alfiedotwtf/cwd/issues)

Feel free to fork the repository and submit pull requests :)

Слава Україні!

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Copyright (C) 2023 by Alfie John

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
