use super::*;
use roff::{bold, list};
/// Boolean arguments that can be toggled on or off.
#[derive(Debug, Clone, Default)]
pub struct Flag {
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
  pub(crate) help: Option<String>,
}

impl Flag {
  /// Create a new instance.
  pub fn new() -> Self {
    Self::default()
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

  /// Set the help value.
  pub fn help(mut self, help: &str) -> Self {
    self.help = Some(help.into());
    self
  }
}

impl FlagOrOption for Flag {
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
    let desc = match self.help {
      Some(ref desc) => desc.to_string(),
      None => "".to_string(),
    };
    return list(&args, &[desc]);
  }
}
