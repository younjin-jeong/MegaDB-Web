// Prometheus metrics proxy.
//
// In Phase 1, the server functions return mock data.
// In Phase 4, this module will proxy PromQL queries to Prometheus:
//   - Query throughput: rate(megadb_queries_total[5m])
//   - Latency percentiles: histogram_quantile(0.99, ...)
//   - Resource usage: container_cpu_usage_seconds_total, container_memory_usage_bytes
