use crate::{viewmodels::ViewModel, views::View};
use std::collections::HashMap;

use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::model::Model;

#[allow(dead_code)]
pub struct App {
    model: Model,
    current_view: View,
    viewmodels: HashMap<View, Box<dyn ViewModel>>,
}

impl App {
    #[allow(dead_code)]
    pub fn new(model: Model) -> color_eyre::Result<Self> {
        Ok(App {
            model,
            current_view: View::MailingListSelection,
            viewmodels: HashMap::new(),
        })
    }

    #[allow(dead_code)]
    pub fn get_current_view(&self) -> View {
        self.current_view
    }

    #[allow(dead_code)]
    pub fn get_current_viewmodel(&mut self) -> &mut Box<dyn ViewModel> {
        self.viewmodels
            .entry(self.current_view)
            .or_insert_with(|| match self.current_view {
                View::MailingListSelection => {
                    todo!()
                }
                View::BookmarkedPatchsets => {
                    todo!()
                }
                View::LatestPatchsets => {
                    todo!()
                }
                View::PatchsetDetails => {
                    todo!()
                }
                View::EditConfig => {
                    todo!()
                }
            })
    }

    #[allow(dead_code, unreachable_code)]
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.get_current_view()
                .draw_screen(self.get_current_viewmodel().state());

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                self.get_current_viewmodel().handle_key(key);
            }
        }
    }
}
