mod mailing_list_viewmodel;

use ratatui::crossterm::event::KeyEvent;
#[allow(dead_code)]
pub trait ViewModel {
    fn handle_key(&self, event: KeyEvent);

    fn state(&self) -> ViewModelState;
}

pub enum ViewModelState {}
