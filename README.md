# cwd

Current Working Directory, a convenience crate 

Instead of copy-pasting this bit of code each time I need the fully qualified
current working directory, I decided to turn it into  a convenience crate...

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

Copyright (C) 2022 by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License and GNU Free Documentation License
as published by the Free Software Foundation, either version 3 of the GPL or
1.3 of the GFDL, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
