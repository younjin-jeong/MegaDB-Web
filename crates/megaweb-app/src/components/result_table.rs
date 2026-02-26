use leptos::prelude::*;
use megaweb_types::query::QueryResult;

/// Virtual-scrolling result table for query output.
#[component]
pub fn ResultTable(result: Signal<Option<QueryResult>>) -> impl IntoView {
    view! {
        <div class="result-table-container">
            {move || match result.get() {
                None => view! {
                    <div class="result-table-empty">
                        <p>"Run a query to see results"</p>
                    </div>
                }.into_any(),
                Some(ref r) if r.error.is_some() => view! {
                    <div class="result-table-error">
                        <p class="error-message">{r.error.clone().unwrap_or_default()}</p>
                    </div>
                }.into_any(),
                Some(ref r) => view! {
                    <div class="result-table-wrapper">
                        <div class="result-table-header">
                            <span class="result-count">
                                {format!("{} rows", r.row_count)}
                            </span>
                            <span class="result-time">
                                {format!("{}ms", r.execution_time_ms)}
                            </span>
                            <div class="result-actions">
                                <button class="btn btn-sm" title="Export CSV">"CSV"</button>
                                <button class="btn btn-sm" title="Export JSON">"JSON"</button>
                            </div>
                        </div>
                        <div class="result-table-scroll">
                            <table class="result-table">
                                <thead>
                                    <tr>
                                        {r.columns.iter().map(|col| {
                                            let name = col.name.clone();
                                            let dtype = col.data_type.clone();
                                            let dtype2 = dtype.clone();
                                            view! {
                                                <th title=dtype>
                                                    <span class="col-name">{name}</span>
                                                    <span class="col-type">{dtype2}</span>
                                                </th>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                </thead>
                                <tbody>
                                    {r.rows.iter().take(1000).enumerate().map(|(i, row)| {
                                        view! {
                                            <tr class=if i % 2 == 0 { "row-even" } else { "row-odd" }>
                                                {row.iter().map(|val| {
                                                    let display = format_value(val);
                                                    let display2 = display.clone();
                                                    view! {
                                                        <td title=display>{display2}</td>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}

fn format_value(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::Null => "NULL".to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        other => other.to_string(),
    }
}
