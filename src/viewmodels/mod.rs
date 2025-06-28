use ratatui::crossterm::event::Event;
#[allow(dead_code)]
/// A trait representing a view model that can handle key events and expose its state.
///
/// # Associated Types
/// - `State`: The type representing the state of the view model. Must have a `'static` lifetime.
///
/// # Required Methods
/// - `handle_key`: Handles a key event, potentially mutating the view model.
/// - `state`: Returns a reference to the current state.
pub trait ViewModel {
    /// Handles a key event, potentially mutating the view model.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to handle.
    fn handle_key(&mut self, event: Event);

    /// Returns a reference to the current state.
    fn state(&self) -> ViewModelState;
}

/// `ViewModelState` is an enum intended to represent the various possible
/// state structs used by different screens in the application. Each variant
/// of this enum should wrap the state struct corresponding to a specific screen,
/// allowing for type-safe handling and dynamic dispatch of view model states.
///
/// Extend this enum by adding variants for each screen's state struct, for example:
///
/// ```
/// enum ViewModelState {
///     HomeScreen(HomeScreenState),
///     SettingsScreen(SettingsScreenState),
///     // Add more variants as needed
/// }
pub enum ViewModelState {}
