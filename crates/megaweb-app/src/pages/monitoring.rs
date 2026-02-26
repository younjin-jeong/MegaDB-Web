use leptos::prelude::*;
use megaweb_types::metrics::*;

/// Server function to get query metrics.
#[server(GetQueryMetrics, "/api")]
pub async fn get_query_metrics() -> Result<QueryMetrics, ServerFnError> {
    Ok(QueryMetrics {
        queries_per_second: 245.0,
        avg_latency_ms: 23.0,
        p50_latency_ms: 12.0,
        p95_latency_ms: 89.0,
        p99_latency_ms: 156.0,
        active_sessions: 12,
        open_connections: 8,
    })
}

/// Server function to get active queries.
#[server(GetActiveQueries, "/api")]
pub async fn get_active_queries() -> Result<Vec<ActiveQuery>, ServerFnError> {
    Ok(vec![
        ActiveQuery {
            session_id: "s-001".into(),
            duration_ms: 12000,
            state: "executing".into(),
            sql_preview: "SELECT cost_by(amount, 'service') FROM cur_data WHERE...".into(),
            database: "megadb".into(),
        },
        ActiveQuery {
            session_id: "s-002".into(),
            duration_ms: 300,
            state: "parsing".into(),
            sql_preview: "INSERT INTO cmdb_resources (resource_id, ...) VALUES ...".into(),
            database: "megadb".into(),
        },
    ])
}

/// Performance Monitoring page.
#[component]
pub fn MonitoringPage() -> impl IntoView {
    let metrics = Resource::new(|| (), |_| get_query_metrics());
    let active_queries = Resource::new(|| (), |_| get_active_queries());

    view! {
        <div class="monitoring-page">
            <h2>"Performance Monitoring"</h2>

            <Suspense fallback=|| view! { <p>"Loading metrics..."</p> }>
                {move || {
                    metrics.get().map(|result| {
                        match result {
                            Ok(m) => view! { <MetricsOverview metrics=m /> }.into_any(),
                            Err(e) => view! { <p class="error">{format!("Error: {e}")}</p> }.into_any(),
                        }
                    })
                }}
            </Suspense>

            <div class="monitoring-section">
                <h3>"Active Queries"</h3>
                <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                    {move || {
                        active_queries.get().map(|result| {
                            match result {
                                Ok(queries) => view! { <ActiveQueriesTable queries=queries /> }.into_any(),
                                Err(e) => view! { <p class="error">{format!("Error: {e}")}</p> }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
fn MetricsOverview(metrics: QueryMetrics) -> impl IntoView {
    view! {
        <div class="metrics-overview">
            <div class="metrics-grid">
                <MetricCard label="Queries/sec" value=format!("{:.0}", metrics.queries_per_second) />
                <MetricCard label="Avg Latency" value=format!("{:.0}ms", metrics.avg_latency_ms) />
                <MetricCard label="P95 Latency" value=format!("{:.0}ms", metrics.p95_latency_ms) />
                <MetricCard label="P99 Latency" value=format!("{:.0}ms", metrics.p99_latency_ms) />
                <MetricCard label="Active Sessions" value=metrics.active_sessions.to_string() />
                <MetricCard label="Connections" value=metrics.open_connections.to_string() />
            </div>
        </div>
    }
}

#[component]
fn MetricCard(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="metric-card">
            <span class="metric-card-value">{value}</span>
            <span class="metric-card-label">{label}</span>
        </div>
    }
}

#[component]
fn ActiveQueriesTable(queries: Vec<ActiveQuery>) -> impl IntoView {
    view! {
        <table class="active-queries-table">
            <thead>
                <tr>
                    <th>"Session"</th>
                    <th>"Duration"</th>
                    <th>"State"</th>
                    <th>"Query"</th>
                </tr>
            </thead>
            <tbody>
                {queries.iter().map(|q| {
                    let duration = if q.duration_ms >= 1000 {
                        format!("{:.1}s", q.duration_ms as f64 / 1000.0)
                    } else {
                        format!("{}ms", q.duration_ms)
                    };
                    view! {
                        <tr>
                            <td class="session-id">{q.session_id.clone()}</td>
                            <td>{duration}</td>
                            <td>
                                <span class=format!("query-state query-state--{}", q.state)>
                                    {q.state.clone()}
                                </span>
                            </td>
                            <td class="query-preview" title=q.sql_preview.clone()>
                                {q.sql_preview.clone()}
                            </td>
                        </tr>
                    }
                }).collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}
