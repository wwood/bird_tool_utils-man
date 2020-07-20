use crate::{Flag, FlagOrOption, Opt};

/// Add a custom section
#[derive(Debug)]
pub struct Section {
  pub(crate) name: String,
  pub(crate) paragraphs: Vec<String>,
  pub(crate) flags_and_options: Vec<Box<dyn FlagOrOption>>,
}

impl Section {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      paragraphs: vec![],
      flags_and_options: vec![],
    }
  }

  pub fn paragraph(mut self, text: &str) -> Self {
    self.paragraphs.push(text.into());
    self
  }

  /// Add an flag.
  pub fn flag(mut self, flag: Flag) -> Self {
    self.flags_and_options.push(Box::new(flag));
    self
  }

  /// Add an option.
  pub fn option(mut self, opt: Opt) -> Self {
    self.flags_and_options.push(Box::new(opt));
    self
  }
}
