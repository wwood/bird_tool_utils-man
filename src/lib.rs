#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate roff;

mod arg;
mod author;
mod environment;
mod example;
mod flag;
mod man;
mod option;
mod section;

pub mod prelude;

pub use arg::Arg;
pub use author::Author;
pub use environment::Env;
pub use example::Example;
pub use flag::Flag;
pub use man::Manual;
pub use option::Opt;
pub use section::Section;

trait FlagOrOption: std::fmt::Debug {
  fn render(&self) -> String;
}
