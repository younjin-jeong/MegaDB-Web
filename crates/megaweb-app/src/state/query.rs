use leptos::prelude::*;
use megaweb_types::query::{QueryHistoryEntry, QueryResult};

/// State for a single query tab.
#[derive(Debug, Clone)]
pub struct QueryTab {
    pub id: uuid::Uuid,
    pub title: String,
    pub sql: String,
    pub result: Option<QueryResult>,
    pub is_running: bool,
}

impl QueryTab {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title: "New Query".to_string(),
            sql: String::new(),
            result: None,
            is_running: false,
        }
    }
}

impl Default for QueryTab {
    fn default() -> Self {
        Self::new()
    }
}

/// Global query state: tabs, active tab, history.
#[derive(Debug, Clone)]
pub struct QueryState {
    pub tabs: Vec<QueryTab>,
    pub active_tab_index: usize,
    pub history: Vec<QueryHistoryEntry>,
}

impl Default for QueryState {
    fn default() -> Self {
        Self {
            tabs: vec![QueryTab::new()],
            active_tab_index: 0,
            history: vec![],
        }
    }
}

impl QueryState {
    pub fn active_tab(&self) -> &QueryTab {
        &self.tabs[self.active_tab_index]
    }

    pub fn active_tab_mut(&mut self) -> &mut QueryTab {
        &mut self.tabs[self.active_tab_index]
    }

    pub fn add_tab(&mut self) {
        self.tabs.push(QueryTab::new());
        self.active_tab_index = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            return;
        }
        self.tabs.remove(index);
        if self.active_tab_index >= self.tabs.len() {
            self.active_tab_index = self.tabs.len() - 1;
        }
    }
}

/// Provide query state as a context.
pub fn provide_query_state() {
    let state = signal(QueryState::default());
    provide_context(state);
}

/// Use query state from context.
pub fn use_query_state() -> (ReadSignal<QueryState>, WriteSignal<QueryState>) {
    expect_context::<(ReadSignal<QueryState>, WriteSignal<QueryState>)>()
}
