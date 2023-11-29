use crate::{action::scene::Scene, action::Action};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use derive_deref::{Deref, DerefMut};
use ratatui::style::{Color, Modifier, Style};
use serde::{de::Deserializer, Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const CONFIG: &str = "
{
  \"keybindings\": {
    \"Home\": {
      \"<H>\": \"ChangeScene(Home)\",
      \"<S>\": \"ChangeScene(Session)\",
      \"<I>\": \"ChangeScene(Internals)\",
      \"<A>\": \"ChangeView(A)\",
      \"<L>\": \"ChangeView(L)\",
      \"<R>\": \"ChangeView(R)\",
      \"<E>\": \"ChangeView(Prompt)\",
      \"<?>\": \"ToggleOverlay(UsageInfo)\",
      \"<q>\": \"Quit\",
      \"<Ctrl-d>\": \"Quit\",
      \"<Ctrl-c>\": \"Quit\",
      \"<Ctrl-z>\": \"Suspend\"
    },
    \"Session\": {
      \"<H>\": \"ChangeScene(Home)\",
      \"<S>\": \"ChangeScene(Session)\",
      \"<I>\": \"ChangeScene(Internals)\",
      \"<A>\": \"ChangeView(A)\",
      \"<L>\": \"ChangeView(L)\",
      \"<R>\": \"ChangeView(R)\",
      \"<.>\": \"ChangeView(Prompt)\",
      \"<?>\": \"ToggleOverlay(UsageInfo)\",
      \"<q>\": \"Quit\",
      \"<Ctrl-d>\": \"Quit\",
      \"<Ctrl-c>\": \"Quit\",
      \"<Ctrl-z>\": \"Suspend\"
    },
    \"Internals\": {
      \"<H>\": \"ChangeScene(Home)\",
      \"<S>\": \"ChangeScene(Session)\",
      \"<I>\": \"ChangeScene(Internals)\",
      \"<A>\": \"ChangeView(A)\",
      \"<L>\": \"ChangeView(L)\",
      \"<R>\": \"ChangeView(R)\",
      \"<?>\": \"ToggleOverlay(UsageInfo)\",
      \"<q>\": \"Quit\",
      \"<Ctrl-d>\": \"Quit\",
      \"<Ctrl-c>\": \"Quit\",
      \"<Ctrl-z>\": \"Suspend\"
    },
  }
}";

/// Defines the application configuration properties.
///
/// This structure holds paths for data and configuration directories.
#[derive(
  Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub struct AppConfig {
  #[serde(default)]
  pub _data_dir: PathBuf,
  #[serde(default)]
  pub _config_dir: PathBuf,
}

/// Represents the main configuration for the application.
///
/// Includes application configuration, keybindings, and styles.
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
  #[serde(default, flatten)]
  pub config: AppConfig,
  #[serde(default)]
  pub keybindings: KeyBindings,
  #[serde(default)]
  pub styles: Styles,
}

impl Config {
  /// Constructs a new configuration instance.
  ///
  /// Attempts to load configuration from various file formats and merges with default config.
  ///
  /// # Returns
  ///
  /// `Result<Self, config::ConfigError>` - The configuration instance or an error.
  pub fn new() -> Result<Self, config::ConfigError> {
    let default_config: Config = json5::from_str(CONFIG).unwrap();
    let data_dir = crate::utils::get_data_dir();
    let config_dir = crate::utils::get_config_dir();
    let mut builder = config::Config::builder()
      .set_default("_data_dir", data_dir.to_str().unwrap())?
      .set_default("_config_dir", config_dir.to_str().unwrap())?;

    let config_files = [
      ("config.json5", config::FileFormat::Json5),
      ("config.json", config::FileFormat::Json),
      ("config.yaml", config::FileFormat::Yaml),
      ("config.toml", config::FileFormat::Toml),
      ("config.ini", config::FileFormat::Ini),
    ];
    let mut found_config = false;
    for (file, format) in &config_files {
      builder = builder.add_source(
        config::File::from(config_dir.join(file))
          .format(*format)
          .required(false),
      );
      if config_dir.join(file).exists() {
        found_config = true;
      }
    }
    if !found_config {
      log::error!(
        "No configuration file found. Application may not behave as expected"
      );
    }

    let mut cfg: Self = builder.build()?.try_deserialize()?;

    for (scene, default_bindings) in &*default_config.keybindings {
      let user_bindings = cfg.keybindings.entry(*scene).or_default();
      for (key, cmd) in default_bindings {
        user_bindings
          .entry(key.clone())
          .or_insert_with(|| cmd.clone());
      }
    }
    for (scene, default_styles) in &*default_config.styles {
      let user_styles = cfg.styles.entry(*scene).or_default();
      for (style_key, style) in default_styles {
        user_styles
          .entry(style_key.clone())
          .or_insert_with(|| *style);
      }
    }

    Ok(cfg)
  }
}

/// Custom key bindings for the application.
///
/// Maps scenes to their respective key bindings.
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct KeyBindings(pub HashMap<Scene, HashMap<Vec<KeyEvent>, Action>>);

impl<'de> Deserialize<'de> for KeyBindings {
  /// Custom deserialization for key bindings.
  ///
  /// Converts string representations of key sequences into actual `KeyEvent` sequences.
  ///
  /// # Parameters
  ///
  /// * `deserializer`: Deserializer to deserialize the key bindings.
  ///
  /// # Returns
  ///
  /// `Result<Self, D::Error>` - The key bindings instance or an error.
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let parsed_map =
      HashMap::<Scene, HashMap<String, Action>>::deserialize(deserializer)?;

    let keybindings = parsed_map
      .into_iter()
      .map(|(scene, inner_map)| {
        let converted_inner_map = inner_map
          .into_iter()
          .map(|(key_str, cmd)| (parse_key_sequence(&key_str).unwrap(), cmd))
          .collect();
        (scene, converted_inner_map)
      })
      .collect();

    Ok(KeyBindings(keybindings))
  }
}

/// Parses a string representation of a `KeyEvent`.
///
/// Converts raw string inputs to `KeyEvent` objects, handling modifiers.
///
/// # Parameters
///
/// * `raw`: The string representation of the key event.
///
/// # Returns
///
/// `Result<KeyEvent, String>` - The parsed `KeyEvent` or an error message.
fn parse_key_event(raw: &str) -> Result<KeyEvent, String> {
  let raw_lower = raw.to_ascii_lowercase();
  let (remaining, modifiers) = extract_modifiers(&raw_lower);
  parse_key_code_with_modifiers(remaining, modifiers)
}
/// Extracts key modifiers from a raw string.
///
/// Parses a raw string to identify and extract key modifiers like Ctrl, Alt, and Shift.
///
/// # Parameters
///
/// * `raw`: The raw string representing the key event with potential modifiers.
///
/// # Returns
///
/// Tuple (`&str`, `KeyModifiers`) - The remaining string after extracting modifiers,
/// and the extracted `KeyModifiers`.
fn extract_modifiers(raw: &str) -> (&str, KeyModifiers) {
  let mut modifiers = KeyModifiers::empty();
  let mut current = raw;

  loop {
    match current {
      rest if rest.starts_with("ctrl-") => {
        modifiers.insert(KeyModifiers::CONTROL);
        current = &rest[5..];
      }
      rest if rest.starts_with("alt-") => {
        modifiers.insert(KeyModifiers::ALT);
        current = &rest[4..];
      }
      rest if rest.starts_with("shift-") => {
        modifiers.insert(KeyModifiers::SHIFT);
        current = &rest[6..];
      }
      _ => break, // break out of the loop if no known prefix is detected
    };
  }

  (current, modifiers)
}

/// Parses a `KeyCode` with modifiers from a raw string.
///
/// Interprets the key code and combines it with any identified modifiers.
///
/// # Parameters
///
/// * `raw`: The string representing the key code.
/// * `modifiers`: The modifiers to be applied to the key code.
///
/// # Returns
///
/// `Result<KeyEvent, String>` - The parsed `KeyEvent` including modifiers, or an error message.
fn parse_key_code_with_modifiers(
  raw: &str,
  mut modifiers: KeyModifiers,
) -> Result<KeyEvent, String> {
  let c = match raw {
    "esc" => KeyCode::Esc,
    "enter" => KeyCode::Enter,
    "left" => KeyCode::Left,
    "right" => KeyCode::Right,
    "up" => KeyCode::Up,
    "down" => KeyCode::Down,
    "home" => KeyCode::Home,
    "end" => KeyCode::End,
    "pageup" => KeyCode::PageUp,
    "pagedown" => KeyCode::PageDown,
    "backtab" => {
      modifiers.insert(KeyModifiers::SHIFT);
      KeyCode::BackTab
    }
    "backspace" => KeyCode::Backspace,
    "delete" => KeyCode::Delete,
    "insert" => KeyCode::Insert,
    "f1" => KeyCode::F(1),
    "f2" => KeyCode::F(2),
    "f3" => KeyCode::F(3),
    "f4" => KeyCode::F(4),
    "f5" => KeyCode::F(5),
    "f6" => KeyCode::F(6),
    "f7" => KeyCode::F(7),
    "f8" => KeyCode::F(8),
    "f9" => KeyCode::F(9),
    "f10" => KeyCode::F(10),
    "f11" => KeyCode::F(11),
    "f12" => KeyCode::F(12),
    "space" => KeyCode::Char(' '),
    "hyphen" | "minus" => KeyCode::Char('-'),
    "tab" => KeyCode::Tab,
    c if c.len() == 1 => {
      let mut c = c.chars().next().unwrap();
      if modifiers.contains(KeyModifiers::SHIFT) {
        c = c.to_ascii_uppercase();
      }
      KeyCode::Char(c)
    }
    _ => return Err(format!("Unable to parse {raw}")),
  };
  Ok(KeyEvent::new(c, modifiers))
}

/// Converts a `KeyEvent` into a string representation.
///
/// This function is typically used for serializing or logging key events.
///
/// # Parameters
///
/// * `key_event`: The `KeyEvent` to convert.
///
/// # Returns
///
/// `String` - The string representation of the `KeyEvent`.
pub fn _key_event_to_string(key_event: &KeyEvent) -> String {
  let char;
  let key_code = match key_event.code {
    KeyCode::Backspace => "backspace",
    KeyCode::Enter => "enter",
    KeyCode::Left => "left",
    KeyCode::Right => "right",
    KeyCode::Up => "up",
    KeyCode::Down => "down",
    KeyCode::Home => "home",
    KeyCode::End => "end",
    KeyCode::PageUp => "pageup",
    KeyCode::PageDown => "pagedown",
    KeyCode::Tab => "tab",
    KeyCode::BackTab => "backtab",
    KeyCode::Delete => "delete",
    KeyCode::Insert => "insert",
    KeyCode::F(c) => {
      char = format!("f({c})");
      &char
    }
    KeyCode::Char(' ') => "space",
    KeyCode::Char(c) => {
      char = c.to_string();
      &char
    }
    KeyCode::Esc => "esc",
    KeyCode::Null
    | KeyCode::CapsLock
    | KeyCode::Menu
    | KeyCode::ScrollLock
    | KeyCode::Media(_)
    | KeyCode::NumLock
    | KeyCode::PrintScreen
    | KeyCode::Pause
    | KeyCode::KeypadBegin
    | KeyCode::Modifier(_) => "",
  };

  let mut modifiers = Vec::with_capacity(3);

  if key_event.modifiers.intersects(KeyModifiers::CONTROL) {
    modifiers.push("ctrl");
  }

  if key_event.modifiers.intersects(KeyModifiers::SHIFT) {
    modifiers.push("shift");
  }

  if key_event.modifiers.intersects(KeyModifiers::ALT) {
    modifiers.push("alt");
  }

  let mut key = modifiers.join("-");

  if !key.is_empty() {
    key.push('-');
  }
  key.push_str(key_code);

  key
}

/// Parses a raw string into a sequence of `KeyEvent`s.
///
/// Useful for converting user-defined key binding strings into actionable key events.
///
/// # Parameters
///
/// * `raw`: The raw string representing a sequence of key events.
///
/// # Returns
///
/// `Result<Vec<KeyEvent>, String>` - A vector of `KeyEvent`s if successful, or an error message.
pub fn parse_key_sequence(raw: &str) -> Result<Vec<KeyEvent>, String> {
  if raw.chars().filter(|c| *c == '>').count()
    != raw.chars().filter(|c| *c == '<').count()
  {
    return Err(format!("Unable to parse `{raw}`"));
  }
  let raw = if raw.contains("><") {
    raw
  } else {
    let raw = raw.strip_prefix('<').unwrap_or(raw);
    let raw = raw.strip_prefix('>').unwrap_or(raw);
    raw
  };
  let sequences = raw
    .split("><")
    .map(|seq| {
      if let Some(s) = seq.strip_prefix('<') {
        s
      } else if let Some(s) = seq.strip_suffix('>') {
        s
      } else {
        seq
      }
    })
    .collect::<Vec<_>>();

  sequences.into_iter().map(parse_key_event).collect()
}

/// Represents custom style configurations for the application.
///
/// Maps application scenes to their respective style configurations.
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Styles(pub HashMap<Scene, HashMap<String, Style>>);

impl<'de> Deserialize<'de> for Styles {
  /// Custom deserialization for styles.
  ///
  /// Converts string representations of styles into `Style` objects.
  ///
  /// # Parameters
  ///
  /// * `deserializer`: Deserializer to deserialize the styles.
  ///
  /// # Returns
  ///
  /// `Result<Self, D::Error>` - The styles instance or an error.
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let parsed_map =
      HashMap::<Scene, HashMap<String, String>>::deserialize(deserializer)?;

    let styles = parsed_map
      .into_iter()
      .map(|(scene, inner_map)| {
        let converted_inner_map = inner_map
          .into_iter()
          .map(|(str, style)| (str, parse_style(&style)))
          .collect();
        (scene, converted_inner_map)
      })
      .collect();

    Ok(Styles(styles))
  }
}

/// Parses a style configuration line into a `Style` object.
///
/// # Parameters
///
/// * `line`: The style configuration line.
///
/// # Returns
///
/// `Style` - The parsed `Style` object.
pub fn parse_style(line: &str) -> Style {
  let (foreground, background) =
    line.split_at(line.to_lowercase().find("on ").unwrap_or(line.len()));
  let foreground = process_color_string(foreground);
  let background = process_color_string(&background.replace("on ", ""));

  let mut style = Style::default();
  if let Some(fg) = parse_color(&foreground.0) {
    style = style.fg(fg);
  }
  if let Some(bg) = parse_color(&background.0) {
    style = style.bg(bg);
  }
  style = style.add_modifier(foreground.1 | background.1);
  style
}

/// Processes a color string to extract the color and any text modifiers.
///
/// # Parameters
///
/// * `color_str`: The string representing the color and modifiers.
///
/// # Returns
///
/// Tuple (`String`, `Modifier`) - The color string and any identified modifiers.
fn process_color_string(color_str: &str) -> (String, Modifier) {
  let color = color_str
    .replace("grey", "gray")
    .replace("bright ", "")
    .replace("bold ", "")
    .replace("underline ", "")
    .replace("inverse ", "");

  let mut modifiers = Modifier::empty();
  if color_str.contains("underline") {
    modifiers |= Modifier::UNDERLINED;
  }
  if color_str.contains("bold") {
    modifiers |= Modifier::BOLD;
  }
  if color_str.contains("inverse") {
    modifiers |= Modifier::REVERSED;
  }

  (color, modifiers)
}

/// Parses a color string into a `Color`.
///
/// # Parameters
///
/// * `s`: The color string.
///
/// # Returns
///
/// `Option<Color>` - The parsed `Color` or `None` if the color cannot be parsed.
fn parse_color(s: &str) -> Option<Color> {
  let s = s.trim_start();
  let s = s.trim_end();
  if s.contains("bright color") {
    let s = s.trim_start_matches("bright ");
    let c = s
      .trim_start_matches("color")
      .parse::<u8>()
      .unwrap_or_default();
    Some(Color::Indexed(c.wrapping_shl(8)))
  } else if s.contains("color") {
    let c = s
      .trim_start_matches("color")
      .parse::<u8>()
      .unwrap_or_default();
    Some(Color::Indexed(c))
  } else if s.contains("gray") {
    let c = 232
      + s
        .trim_start_matches("gray")
        .parse::<u8>()
        .unwrap_or_default();
    Some(Color::Indexed(c))
  } else if s.contains("rgb") {
    let red = (s.as_bytes()[3] as char).to_digit(10).unwrap_or_default() as u8;
    let green =
      (s.as_bytes()[4] as char).to_digit(10).unwrap_or_default() as u8;
    let blue = (s.as_bytes()[5] as char).to_digit(10).unwrap_or_default() as u8;
    let c = 16 + red * 36 + green * 6 + blue;
    Some(Color::Indexed(c))
  } else if s == "bold black" {
    Some(Color::Indexed(8))
  } else if s == "bold red" {
    Some(Color::Indexed(9))
  } else if s == "bold green" {
    Some(Color::Indexed(10))
  } else if s == "bold yellow" {
    Some(Color::Indexed(11))
  } else if s == "bold blue" {
    Some(Color::Indexed(12))
  } else if s == "bold magenta" {
    Some(Color::Indexed(13))
  } else if s == "bold cyan" {
    Some(Color::Indexed(14))
  } else if s == "bold white" {
    Some(Color::Indexed(15))
  } else if s == "black" {
    Some(Color::Indexed(0))
  } else if s == "red" {
    Some(Color::Indexed(1))
  } else if s == "green" {
    Some(Color::Indexed(2))
  } else if s == "yellow" {
    Some(Color::Indexed(3))
  } else if s == "blue" {
    Some(Color::Indexed(4))
  } else if s == "magenta" {
    Some(Color::Indexed(5))
  } else if s == "cyan" {
    Some(Color::Indexed(6))
  } else if s == "white" {
    Some(Color::Indexed(7))
  } else {
    None
  }
}

/// Test module for `Config`, `KeyBindings`, `Styles`, and associated parsing functions.
#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  /// Tests the default configuration for style parsing.
  ///
  /// Ensures that an empty style string is parsed as a default `Style` object.
  #[test]
  fn test_parse_style_default() {
    let style = parse_style("");
    assert_eq!(style, Style::default());
  }

  /// Tests the parsing of a foreground color in a style string.
  ///
  /// Verifies that a simple color name is correctly parsed as the foreground color.
  #[test]
  fn test_parse_style_foreground() {
    let style = parse_style("red");
    assert_eq!(style.fg, Some(Color::Indexed(1)));
  }

  /// Tests the parsing of a background color in a style string.
  ///
  /// Checks if the background color is correctly interpreted from the style string.
  #[test]
  fn test_parse_style_background() {
    let style = parse_style("on blue");
    assert_eq!(style.bg, Some(Color::Indexed(4)));
  }

  /// Tests the parsing of style modifiers along with colors.
  ///
  /// Confirms that both foreground and background colors as well as text modifiers are correctly parsed.
  #[test]
  fn test_parse_style_modifiers() {
    let style = parse_style("underline red on blue");
    assert_eq!(style.fg, Some(Color::Indexed(1)));
    assert_eq!(style.bg, Some(Color::Indexed(4)));
  }

  /// Tests processing of a color string with modifiers.
  ///
  /// Validates correct extraction of color and modifiers from a given string.
  #[test]
  fn test_process_color_string() {
    let (color, modifiers) =
      process_color_string("underline bold inverse gray");
    assert_eq!(color, "gray");
    assert!(modifiers.contains(Modifier::UNDERLINED));
    assert!(modifiers.contains(Modifier::BOLD));
    assert!(modifiers.contains(Modifier::REVERSED));
  }

  /// Tests the parsing of RGB color values.
  ///
  /// Verifies that RGB color strings are correctly converted into their respective color values.
  #[test]
  fn test_parse_color_rgb() {
    let color = parse_color("rgb123");
    let expected = 16 + 36 + 2 * 6 + 3;
    assert_eq!(color, Some(Color::Indexed(expected)));
  }

  /// Tests parsing of unknown color values.
  ///
  /// Ensures that an unknown color string returns `None`.
  #[test]
  fn test_parse_color_unknown() {
    let color = parse_color("unknown");
    assert_eq!(color, None);
  }

  /// Tests loading configuration for a specific scene.
  ///
  /// Verifies that keybindings for a given scene (e.g., `Scene::Home`) are correctly loaded from the configuration.
  #[test]
  fn test_config_home() -> Result<()> {
    let c = Config::new()?;
    assert_eq!(
      c.keybindings
        .get(&Scene::Home)
        .unwrap()
        .get(&parse_key_sequence("<q>").unwrap_or_default())
        .unwrap(),
      &Action::Quit
    );
    Ok(())
  }

  /// Tests loading configuration for the 'Internals' scene.
  ///
  /// Ensures keybindings specific to the 'Internals' scene are correctly loaded and parsed.
  #[test]
  fn test_config_internals() -> Result<()> {
    let c = Config::new()?;
    assert_eq!(
      c.keybindings
        .get(&Scene::Internals)
        .unwrap()
        .get(&parse_key_sequence("<q>").unwrap_or_default())
        .unwrap(),
      &Action::Quit
    );
    Ok(())
  }

  /// Tests parsing of simple key events.
  ///
  /// Verifies that individual key events are correctly parsed from their string representations.
  #[test]
  fn test_simple_keys() {
    assert_eq!(
      parse_key_event("a").unwrap(),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty())
    );

    assert_eq!(
      parse_key_event("enter").unwrap(),
      KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())
    );

    assert_eq!(
      parse_key_event("esc").unwrap(),
      KeyEvent::new(KeyCode::Esc, KeyModifiers::empty())
    );
  }

  /// Tests parsing of key events with modifiers.
  ///
  /// Confirms that key events with modifiers like Ctrl and Alt are correctly interpreted.
  #[test]
  fn test_with_modifiers() {
    assert_eq!(
      parse_key_event("ctrl-a").unwrap(),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)
    );

    assert_eq!(
      parse_key_event("alt-enter").unwrap(),
      KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)
    );

    assert_eq!(
      parse_key_event("shift-esc").unwrap(),
      KeyEvent::new(KeyCode::Esc, KeyModifiers::SHIFT)
    );
  }

  /// Tests parsing of key events with multiple modifiers.
  ///
  /// Checks that combinations of multiple modifiers are correctly handled.
  #[test]
  fn test_multiple_modifiers() {
    assert_eq!(
      parse_key_event("ctrl-alt-a").unwrap(),
      KeyEvent::new(
        KeyCode::Char('a'),
        KeyModifiers::CONTROL | KeyModifiers::ALT
      )
    );

    assert_eq!(
      parse_key_event("ctrl-shift-enter").unwrap(),
      KeyEvent::new(
        KeyCode::Enter,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT
      )
    );
  }

  /// Tests reverse parsing from `KeyEvent` to string.
  ///
  /// Verifies that a `KeyEvent` object can be accurately converted back into its string representation.
  #[test]
  fn test_reverse_multiple_modifiers() {
    assert_eq!(
      _key_event_to_string(&KeyEvent::new(
        KeyCode::Char('a'),
        KeyModifiers::CONTROL | KeyModifiers::ALT
      )),
      "ctrl-alt-a".to_string()
    );
  }

  /// Tests parsing of invalid key strings.
  ///
  /// Ensures that invalid key strings result in an error.
  #[test]
  fn test_invalid_keys() {
    assert!(parse_key_event("invalid-key").is_err());
    assert!(parse_key_event("ctrl-invalid-key").is_err());
  }

  /// Tests case insensitivity in key parsing.
  ///
  /// Confirms that the key parsing logic correctly handles case-insensitive input.
  #[test]
  fn test_case_insensitivity() {
    assert_eq!(
      parse_key_event("CTRL-a").unwrap(),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)
    );

    assert_eq!(
      parse_key_event("AlT-eNtEr").unwrap(),
      KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)
    );
  }
}
