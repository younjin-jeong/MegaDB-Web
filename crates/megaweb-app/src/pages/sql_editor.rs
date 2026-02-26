use leptos::prelude::*;
use megaweb_types::query::QueryResult;

use crate::components::codemirror::CodeMirrorEditor;
use crate::components::result_table::ResultTable;
use crate::components::tab_bar::{Tab, TabBar};

/// Server function to execute SQL against MegaDB.
/// In Phase 1, returns mock data. Will proxy to MegaDB HTTP API later.
#[cfg(feature = "ssr")]
async fn mock_execute_query(sql: &str) -> QueryResult {
    use megaweb_types::query::QueryColumn;

    // Simulate execution delay
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

    // Return mock CUR-like data
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
    // Phase 1: return mock data
    // Phase 4+: proxy to MegaDB HTTP API at {megadb_url}/query
    Ok(mock_execute_query(&sql).await)
}

/// SQL Editor page with multi-tab support.
#[component]
pub fn SqlEditorPage() -> impl IntoView {
    // Tab state
    let (tabs, set_tabs) = signal(vec![Tab {
        id: "tab-1".to_string(),
        title: "Query 1".to_string(),
        closeable: true,
    }]);
    let (active_tab, set_active_tab) = signal(0usize);
    let tab_count = StoredValue::new(1usize);

    // SQL content per tab (keyed by index for simplicity)
    let (sql_content, set_sql_content) = signal(String::new());

    // Query result
    let (result, set_result) = signal(Option::<QueryResult>::None);
    let (is_running, set_is_running) = signal(false);

    // Execute query action
    let execute_action = Action::new(move |sql: &String| {
        let sql = sql.clone();
        async move { execute_query(sql, "megadb".to_string()).await }
    });

    // Update result when action completes
    Effect::new(move || {
        if let Some(result_value) = execute_action.value().get() {
            set_is_running.set(false);
            match result_value {
                Ok(r) => set_result.set(Some(r)),
                Err(e) => set_result.set(Some(QueryResult {
                    columns: vec![],
                    rows: vec![],
                    row_count: 0,
                    execution_time_ms: 0,
                    error: Some(e.to_string()),
                })),
            }
        }
    });

    let on_execute = Callback::new(move |sql: String| {
        set_is_running.set(true);
        execute_action.dispatch(sql);
    });

    let on_tab_select = Callback::new(move |i: usize| {
        set_active_tab.set(i);
    });

    let on_tab_close = Callback::new(move |i: usize| {
        set_tabs.update(|t| {
            if t.len() > 1 {
                t.remove(i);
            }
        });
        set_active_tab.update(|a| {
            let len = tabs.get_untracked().len();
            if *a >= len {
                *a = len.saturating_sub(1);
            }
        });
    });

    let on_tab_add = Callback::new(move |_: ()| {
        let count = tab_count.get_value() + 1;
        tab_count.set_value(count);
        set_tabs.update(|t| {
            t.push(Tab {
                id: format!("tab-{count}"),
                title: format!("Query {count}"),
                closeable: true,
            });
        });
        set_active_tab.set(tabs.get_untracked().len() - 1);
    });

    let tabs_signal = Signal::from(tabs);
    let active_signal = Signal::from(active_tab);
    let result_signal = Signal::from(result);

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
                        on_change=Callback::new(move |s: String| set_sql_content.set(s))
                    />
                    <div class="editor-toolbar">
                        <button
                            class="btn btn-primary"
                            disabled=move || is_running.get()
                            on:click=move |_| on_execute.run(sql_content.get())
                        >
                            {move || if is_running.get() { "Running..." } else { "Run (Ctrl+Enter)" }}
                        </button>
                    </div>
                </div>

                <div class="results-pane">
                    <ResultTable result=result_signal />
                </div>
            </div>
        </div>
    }
}
