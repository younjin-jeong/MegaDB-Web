use leptos::prelude::*;
use megaweb_types::query::{QueryHistoryEntry, QueryResult};

use crate::components::codemirror::CodeMirrorEditor;
use crate::components::query_history::QueryHistoryPanel;
use crate::components::result_table::ResultTable;
use crate::components::tab_bar::{Tab, TabBar};
use crate::state::query::use_query_state;

/// Server function to execute SQL against MegaDB.
/// In Phase 1, returns mock data. Will proxy to MegaDB HTTP API later.
#[cfg(feature = "ssr")]
async fn mock_execute_query(sql: &str) -> QueryResult {
    use megaweb_types::query::QueryColumn;

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    if sql.trim().is_empty() {
        return QueryResult {
            columns: vec![],
            rows: vec![],
            row_count: 0,
            execution_time_ms: 0,
            error: Some("Empty query".to_string()),
        };
    }

    QueryResult {
        columns: vec![
            QueryColumn {
                name: "line_item_id".into(),
                data_type: "VARCHAR".into(),
                nullable: false,
            },
            QueryColumn {
                name: "account_id".into(),
                data_type: "VARCHAR".into(),
                nullable: false,
            },
            QueryColumn {
                name: "service_name".into(),
                data_type: "VARCHAR".into(),
                nullable: false,
            },
            QueryColumn {
                name: "usage_type".into(),
                data_type: "VARCHAR".into(),
                nullable: true,
            },
            QueryColumn {
                name: "cost".into(),
                data_type: "DECIMAL(18,6)".into(),
                nullable: false,
            },
            QueryColumn {
                name: "region".into(),
                data_type: "VARCHAR".into(),
                nullable: true,
            },
            QueryColumn {
                name: "billing_period".into(),
                data_type: "DATE".into(),
                nullable: false,
            },
        ],
        rows: (0..25)
            .map(|i| {
                vec![
                    serde_json::json!(format!("li-{:06}", i)),
                    serde_json::json!("123456789012"),
                    serde_json::json!(["EC2", "S3", "RDS", "Lambda", "CloudFront"][i % 5]),
                    serde_json::json!(["BoxUsage", "DataTransfer", "Requests", "Storage"][i % 4]),
                    serde_json::json!(format!("{:.2}", (i as f64 * 12.34) + 5.67)),
                    serde_json::json!(["us-east-1", "eu-west-1", "ap-northeast-1"][i % 3]),
                    serde_json::json!("2026-02-01"),
                ]
            })
            .collect(),
        row_count: 25,
        execution_time_ms: 23,
        error: None,
    }
}

#[server(ExecuteQuery, "/api")]
pub async fn execute_query(sql: String, _database: String) -> Result<QueryResult, ServerFnError> {
    Ok(mock_execute_query(&sql).await)
}

/// SQL Editor page with multi-tab support and query history.
#[component]
pub fn SqlEditorPage() -> impl IntoView {
    let (query_state, set_query_state) = use_query_state();

    let (show_history, set_show_history) = signal(false);

    // Derive tab list from global state
    let tabs_signal = Signal::derive(move || {
        let state = query_state.get();
        state
            .tabs
            .iter()
            .map(|t| Tab {
                id: t.id.to_string(),
                title: t.title.clone(),
                closeable: state.tabs.len() > 1,
            })
            .collect::<Vec<_>>()
    });

    let active_signal = Signal::derive(move || query_state.get().active_tab_index);
    let result_signal = Signal::derive(move || query_state.get().active_tab().result.clone());
    let is_running = Signal::derive(move || query_state.get().active_tab().is_running);
    let sql_content = Signal::derive(move || query_state.get().active_tab().sql.clone());

    // Execute query action
    let execute_action = Action::new(move |sql: &String| {
        let sql = sql.clone();
        async move { execute_query(sql, "megadb".to_string()).await }
    });

    // Update result and history when action completes
    Effect::new(move || {
        if let Some(result_value) = execute_action.value().get() {
            let sql = query_state.get_untracked().active_tab().sql.clone();
            match result_value {
                Ok(ref r) => {
                    let result = r.clone();
                    let entry = QueryHistoryEntry {
                        id: uuid::Uuid::new_v4(),
                        sql,
                        database: "megadb".to_string(),
                        execution_time_ms: result.execution_time_ms,
                        row_count: result.row_count,
                        executed_at: chrono::Utc::now(),
                        success: result.is_ok(),
                    };
                    set_query_state.update(|s| {
                        s.active_tab_mut().result = Some(result);
                        s.active_tab_mut().is_running = false;
                        s.push_history(entry);
                    });
                }
                Err(ref e) => {
                    let err_msg = e.to_string();
                    let entry = QueryHistoryEntry {
                        id: uuid::Uuid::new_v4(),
                        sql,
                        database: "megadb".to_string(),
                        execution_time_ms: 0,
                        row_count: 0,
                        executed_at: chrono::Utc::now(),
                        success: false,
                    };
                    set_query_state.update(|s| {
                        s.active_tab_mut().result = Some(QueryResult {
                            columns: vec![],
                            rows: vec![],
                            row_count: 0,
                            execution_time_ms: 0,
                            error: Some(err_msg),
                        });
                        s.active_tab_mut().is_running = false;
                        s.push_history(entry);
                    });
                }
            }
        }
    });

    let on_execute = Callback::new(move |sql: String| {
        set_query_state.update(|s| {
            s.active_tab_mut().sql = sql.clone();
            s.active_tab_mut().is_running = true;
        });
        execute_action.dispatch(sql);
    });

    let on_tab_select = Callback::new(move |i: usize| {
        set_query_state.update(|s| s.active_tab_index = i);
    });

    let on_tab_close = Callback::new(move |i: usize| {
        set_query_state.update(|s| s.close_tab(i));
    });

    let on_tab_add = Callback::new(move |_: ()| {
        set_query_state.update(|s| s.add_tab());
    });

    let on_sql_change = Callback::new(move |s: String| {
        set_query_state.update(|state| state.active_tab_mut().sql = s);
    });

    let on_history_restore = Callback::new(move |sql: String| {
        set_query_state.update(|s| s.active_tab_mut().sql = sql);
        set_show_history.set(false);
    });

    view! {
        <div class="sql-editor-page">
            <TabBar
                tabs=tabs_signal
                active_index=active_signal
                on_select=on_tab_select
                on_close=on_tab_close
                on_add=on_tab_add
            />

            <div class="sql-editor-content">
                <div class="editor-pane">
                    <CodeMirrorEditor
                        initial_content=String::new()
                        on_execute=on_execute
                        on_change=on_sql_change
                    />
                    <div class="editor-toolbar">
                        <button
                            class="btn btn-primary"
                            disabled=move || is_running.get()
                            on:click=move |_| on_execute.run(sql_content.get())
                        >
                            {move || if is_running.get() { "Running..." } else { "Run (Ctrl+Enter)" }}
                        </button>
                        <button
                            class="btn btn-secondary"
                            on:click=move |_| set_show_history.update(|v| *v = !*v)
                        >
                            {move || if show_history.get() { "Hide History" } else { "History" }}
                        </button>
                    </div>
                </div>

                <QueryHistoryPanel
                    show=Signal::from(show_history)
                    on_restore=on_history_restore
                    on_close=Callback::new(move |_| set_show_history.set(false))
                />

                <div class="results-pane">
                    <ResultTable result=result_signal />
                </div>
            </div>
        </div>
    }
}
