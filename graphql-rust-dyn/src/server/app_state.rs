use axum::extract::FromRef;
use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub fn new(db: Pool) -> Self {
        Self {
            db: Database { db },
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pub db: Pool,
}

impl FromRef<AppState> for Database {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}
