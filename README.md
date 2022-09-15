# Bevy DTM

Bevy support for the DTM image format.
This plugin allows loading of `.dtm` files.

## Usage

```rust
use bevy::prelude::*;
use bevy_dtm::DTMPlugin;

fn main() {
    App::new()
        .add_plugin(DTMPlugin)
        .run();
}
```

## License
DTM Image Format is dual-licensed under either

* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.