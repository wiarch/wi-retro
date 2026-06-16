use std::path::PathBuf;

use parking_lot::RwLock;

use crate::models::Game;
use crate::persistence::{self, PersistenceError};

#[derive(Debug)]
pub struct AppState {
    inner: RwLock<AppStateInner>,
}

#[derive(Debug)]
struct AppStateInner {
    library_file: PathBuf,
    games: Vec<Game>,
}

impl AppState {
    pub fn new(library_file: PathBuf) -> Result<Self, PersistenceError> {
        let games = persistence::load_games(&library_file)?;
        Ok(Self {
            inner: RwLock::new(AppStateInner {
                library_file,
                games,
            }),
        })
    }

    pub fn games(&self) -> Vec<Game> {
        self.inner.read().games.clone()
    }

    pub fn replace_games(&self, games: Vec<Game>) -> Result<(), PersistenceError> {
        let mut inner = self.inner.write();
        persistence::save_games(&inner.library_file, &games)?;
        inner.games = games;
        Ok(())
    }
}
