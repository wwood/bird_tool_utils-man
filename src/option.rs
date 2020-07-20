use super::*;
use roff::{bold, italic, list};

/// Arguments that take values.
#[derive(Debug, Clone)]
pub struct Opt {
  pub(crate) name: String,
  pub(crate) default: Option<String>,
  pub(crate) help: Option<String>,
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
}

impl Opt {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      default: None,
      help: None,
      short: None,
      long: None,
    }
  }

  /// Set the default value.
  pub fn default_value(mut self, default: &str) -> Self {
    self.default = Some(default.into());
    self
  }

  /// Set the help.
  pub fn help(mut self, help: &str) -> Self {
    self.help = Some(help.into());
    self
  }

  /// Set the short value.
  pub fn short(mut self, short: &str) -> Self {
    self.short = Some(short.into());
    self
  }

  /// Set the long value.
  pub fn long(mut self, long: &str) -> Self {
    self.long = Some(long.into());
    self
  }
}

impl FlagOrOption for Opt {
  fn render(&self) -> String {
    let mut args: Vec<String> = vec![];
    if let Some(ref short) = self.short {
      args.push(bold(&short));
    }
    if let Some(ref long) = self.long {
      if !args.is_empty() {
        args.push(", ".to_string());
      }
      args.push(bold(&long));
    }
    args.push("=".into());
    args.push(italic(&self.name));
    if let Some(ref default) = self.default {
      if !args.is_empty() {
        args.push(" ".to_string());
      }
      args.push("[".into());
      args.push("default:".into());
      args.push(" ".into());
      args.push(italic(&default));
      args.push("]".into());
    }
    let desc = match self.help {
      Some(ref desc) => desc.to_string(),
      None => "".to_string(),
    };
    list(&args, &[desc])
  }
}
