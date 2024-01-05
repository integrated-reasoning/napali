use crate::{
  action::mode::Mode,
  action::scene::Scene,
  action::Action,
  components::{
    about::About, base::Base, data::Data, internals::Internals,
    session::Session, usage_info::UsageInfo, Component,
  },
  config::Config,
  irx_client::IrxClient,
  router::{Address, Message, Router},
  tui,
};
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use tokio::sync::mpsc;

/// The main application structure for Napali's `App`.
///
/// `App` manages the application's state, including configuration, tick and frame rates,
/// components, and message handling. It integrates with various modules to provide
/// a cohesive application framework.
pub struct App {
  /// Application configuration settings.
  pub config: Config,
  /// The rate at which the application logic updates (in ticks per second).
  pub tick_rate: f64,
  /// The rate at which the application renders frames (in frames per second).
  pub frame_rate: f64,
  /// A vector of components that make up the application.
  pub components: Vec<Box<dyn Component>>,
  /// Flag to determine if the application should quit.
  pub should_quit: bool,
  /// Flag to determine if the application should suspend operations.
  pub should_suspend: bool,
  /// Current operating mode of the application.
  pub mode: Mode,
  /// Current scene being displayed in the application.
  pub scene: Scene,
  /// Stores the last key events processed in the current tick.
  pub last_tick_key_events: Vec<KeyEvent>,
  /// Internal router for managing message passing.
  router: Router,
  /// Channel for sending messages to the application itself.
  pub message_tx_to_self: mpsc::UnboundedSender<Message>,
  /// Client for interacting with the Irx API.
  pub client: IrxClient,
}

impl App {
  /// Constructs a new instance of `App`.
  ///
  /// Initializes the application with specified tick and frame rates, sets up
  /// components, message channels, and default states.
  ///
  /// # Parameters
  ///
  /// * `tick_rate`: The rate at which the app's logic updates.
  /// * `frame_rate`: The rate at which the app renders frames.
  ///
  /// # Returns
  ///
  /// `Result<App, Error>` - Ok with `App` instance if successful, or an error if not.
  ///
  /// # Errors
  ///
  /// Returns an error if initialization fails (e.g., network issues, configuration errors).
  ///
  /// # Examples
  ///
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///     let app = App::new(60.0, 30.0).await.expect("Failed to create App");
  ///     // Use `app` here
  /// }
  /// ```
  pub async fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
    let (message_tx_to_self, _) = mpsc::unbounded_channel::<Message>();
    let (mut router, message_tx_to_router) =
      Router::new(message_tx_to_self.clone()).await?;
    let base = Base::new();
    let internals = Internals::new(message_tx_to_router.clone());
    let about = About::new(message_tx_to_router.clone());
    let usage_info = UsageInfo::default();
    let config = Config::new()?;
    let scene = Scene::Internals;
    let mode = Mode::Navigation;
    let client = IrxClient::new(message_tx_to_router.clone()).await?;
    let data = Data::new();
    let session = Session::new();
    router.register(Address::About, about.message_tx_to_self.clone());
    router.register(Address::Internals, internals.message_tx_to_self.clone());
    router.register(
      Address::StateDisplay,
      internals.get_state_display_tx_handle(),
    );
    router.register(Address::IrxClient, client.message_tx_to_self.clone());
    router.register(Address::Session, session.message_tx_to_self.clone());

    Ok(Self {
      tick_rate,
      frame_rate,
      components: vec![
        Box::new(internals),
        Box::new(about),
        Box::new(data),
        Box::new(session),
        Box::new(base),
        // Overlays (must be listed last):
        Box::new(usage_info),
      ],
      should_quit: false,
      should_suspend: false,
      config,
      mode,
      scene,
      last_tick_key_events: Vec::new(),
      router,
      message_tx_to_self,
      client,
    })
  }

  /// Runs the main event loop of the application.
  ///
  /// This asynchronous method starts the application, handling UI events,
  /// routing messages, and managing application states and transitions.
  ///
  /// # Returns
  ///
  /// `Result<()>` - Ok if the run loop completes normally, or an error if it encounters a problem.
  ///
  /// # Errors
  ///
  /// Returns an error if there are issues in event handling or message processing.
  ///
  /// # Examples
  ///
  /// ```
  /// #[tokio::main]
  /// async fn main() {
  ///     let mut app = App::new(60.0, 30.0).await.expect("Failed to create App");
  ///     app.run().await.expect("Failed to run App");
  /// }
  /// ```
  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();
    action_tx.send(Action::ChangeScene(Scene::default()))?;
    self.router.run();
    self.client.run_responder();

    let mut tui = tui::Tui::new()?
      .tick_rate(self.tick_rate)
      .frame_rate(self.frame_rate);
    // tui.mouse(true);
    tui.enter()?;

    for component in &mut self.components {
      component.register_action_handler(action_tx.clone())?;
      component.register_config_handler(self.config.clone())?;
      component.init(tui.size()?)?;
    }

    loop {
      if let Some(e) = tui.next().await {
        match e {
          tui::Event::Quit => action_tx.send(Action::Quit)?,
          tui::Event::Tick => action_tx.send(Action::Tick)?,
          tui::Event::Render => action_tx.send(Action::Render)?,
          tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
          tui::Event::Key(key) => {
            if let Some(keymap) = self.config.keybindings.get(&self.scene) {
              if let Some(action) = keymap.get(&vec![key]) {
                log::info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
              } else {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                  log::info!("Got action: {action:?}");
                  action_tx.send(action.clone())?;
                }
              }
            };
          }
          _ => {}
        }
        for component in &mut self.components {
          if let Some(action) = component.handle_events(Some(e.clone()))? {
            action_tx.send(action)?;
          }
        }
      }

      while let Ok(action) = action_rx.try_recv() {
        if action != Action::Tick && action != Action::Render {
          log::debug!("{action:?}");
        }
        match action {
          Action::Tick => {
            self.last_tick_key_events.drain(..);
          }
          Action::Quit => self.should_quit = true,
          Action::Suspend => self.should_suspend = true,
          Action::Resume => self.should_suspend = false,
          Action::Resize(w, h) => {
            tui.resize(Rect::new(0, 0, w, h))?;
            tui.draw(|f| {
              for component in &mut self.components {
                let r = component.draw(f, f.size());
                if let Err(e) = r {
                  action_tx
                    .send(Action::Error(format!("Failed to draw: {e:?}")))
                    .unwrap();
                }
              }
            })?;
          }
          Action::ChangeScene(scene) => self.scene = scene,
          Action::Render => {
            tui.draw(|f| {
              for component in &mut self.components {
                let r = component.draw(f, f.size());
                if let Err(e) = r {
                  action_tx
                    .send(Action::Error(format!("Failed to draw: {e:?}")))
                    .unwrap();
                }
              }
            })?;
          }
          _ => {}
        }
        for component in &mut self.components {
          if let Some(action) = component.update(action.clone())? {
            action_tx.send(action)?;
          };
        }
      }
      if self.should_suspend {
        tui.suspend()?;
        action_tx.send(Action::Resume)?;
        tui = tui::Tui::new()?
          .tick_rate(self.tick_rate)
          .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;
      } else if self.should_quit {
        tui.stop();
        break;
      }
    }
    tui.exit()?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use color_eyre::eyre::Result;

  #[tokio::test]
  async fn test_app_new() -> Result<()> {
    let _ = App::new(1.0, 60.0).await?;
    Ok(())
  }
}
