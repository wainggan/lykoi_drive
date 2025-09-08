#![doc = include_str!("../readme.md")]
/*!
## example

using [glfw](https://docs.rs/glfw/latest/glfw/),

```ignore
let mut glfw = glfw::init(|error, desc| println!("{}: {}", error, desc)).unwrap();

let (mut window, events) = glfw.create_window(640, 480, "meow", glfw::WindowMode::Windowed).unwrap();
window.make_current();

lykoi::drive::gl::raw::load_with(|s| glfw.get_proc_address_raw(s));
```

use the [`extra`] module for ergonomics!

```should_panic
todo!();
```

*/


mod full;
pub use full::*;

pub mod extra;

pub use lykoi_gl as gl;

