// MegaDB HTTP API proxy.
//
// In Phase 1, the server functions in megaweb-app return mock data directly.
// In Phase 4+, this module will proxy requests to the MegaDB HTTP API using reqwest.
//
// Planned proxy routes:
//   POST /api/query       -> POST megadb:8080/query
//   GET  /api/tables      -> GET  megadb:8080/tables
//   GET  /api/health      -> GET  megadb:8080/health
//   GET  /api/metrics     -> GET  megadb:8080/metrics
