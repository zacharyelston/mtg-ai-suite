//! Admin dashboard for monitoring server stats

use axum::{
    response::Html,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
static SERVER_START: OnceLock<DateTime<Utc>> = OnceLock::new();

pub fn init_stats() {
    SERVER_START.get_or_init(Utc::now);
}

pub fn increment_requests() {
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[derive(Serialize)]
pub struct ServerStats {
    uptime_seconds: i64,
    total_requests: u64,
    server_version: &'static str,
    active_clients: u32,
}

#[derive(Serialize)]
pub struct ClientInfo {
    id: String,
    name: String,
    last_seen: String,
    request_count: u32,
    permissions: Vec<String>,
}

#[derive(Serialize)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

pub async fn dashboard_html() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

pub async fn get_stats() -> Json<ServerStats> {
    let uptime = SERVER_START
        .get()
        .map(|start| Utc::now().signed_duration_since(*start).num_seconds())
        .unwrap_or(0);

    Json(ServerStats {
        uptime_seconds: uptime,
        total_requests: REQUEST_COUNT.load(Ordering::Relaxed),
        server_version: env!("CARGO_PKG_VERSION"),
        active_clients: 0,
    })
}

pub async fn get_clients() -> Json<Vec<ClientInfo>> {
    Json(vec![
        ClientInfo {
            id: "9b4d325d-f1b9-4b79-8e2b-67d5c37f7f72".to_string(),
            name: "TestClient".to_string(),
            last_seen: Utc::now().to_rfc3339(),
            request_count: 0,
            permissions: vec!["read".to_string(), "write".to_string(), "ai".to_string(), "push".to_string()],
        },
    ])
}

pub async fn get_logs() -> Json<Vec<LogEntry>> {
    Json(vec![
        LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: "Server started".to_string(),
        },
    ])
}

const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MTG AI Suite - Dashboard</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            min-height: 100vh;
            color: #e0e0e0;
        }
        .header {
            background: rgba(0,0,0,0.3);
            padding: 20px 40px;
            border-bottom: 1px solid #333;
        }
        .header h1 { color: #fff; font-size: 24px; }
        .header span { color: #888; font-size: 14px; margin-left: 10px; }
        .tabs {
            display: flex;
            background: rgba(0,0,0,0.2);
            padding: 0 40px;
        }
        .tab {
            padding: 15px 30px;
            cursor: pointer;
            border-bottom: 3px solid transparent;
            transition: all 0.2s;
            color: #888;
        }
        .tab:hover { color: #fff; background: rgba(255,255,255,0.05); }
        .tab.active { border-bottom-color: #4a90d9; color: #fff; }
        .content { padding: 40px; }
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }
        .stat-card {
            background: rgba(255,255,255,0.05);
            border-radius: 12px;
            padding: 25px;
            border: 1px solid #333;
        }
        .stat-card h3 { color: #888; font-size: 12px; text-transform: uppercase; margin-bottom: 10px; }
        .stat-card .value { font-size: 36px; font-weight: bold; color: #4a90d9; }
        .stat-card .unit { font-size: 14px; color: #666; margin-left: 5px; }
        table {
            width: 100%;
            border-collapse: collapse;
            background: rgba(255,255,255,0.03);
            border-radius: 12px;
            overflow: hidden;
        }
        th, td { padding: 15px 20px; text-align: left; border-bottom: 1px solid #333; }
        th { background: rgba(0,0,0,0.3); color: #888; font-size: 12px; text-transform: uppercase; }
        tr:hover { background: rgba(255,255,255,0.05); }
        .badge {
            display: inline-block;
            padding: 4px 10px;
            border-radius: 20px;
            font-size: 11px;
            font-weight: 500;
        }
        .badge-info { background: #1e3a5f; color: #4a90d9; }
        .badge-success { background: #1e4620; color: #4caf50; }
        .badge-warn { background: #4a3f00; color: #ffc107; }
        .badge-error { background: #4a1e1e; color: #f44336; }
        .log-entry { font-family: 'Monaco', 'Menlo', monospace; font-size: 13px; }
        .refresh-btn {
            background: #4a90d9;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            margin-bottom: 20px;
        }
        .refresh-btn:hover { background: #3a7bc8; }
    </style>
</head>
<body>
    <div class="header">
        <h1>MTG AI Suite <span>Dashboard</span></h1>
    </div>
    <div class="tabs">
        <div class="tab active" data-tab="overview">Overview</div>
        <div class="tab" data-tab="clients">Clients</div>
        <div class="tab" data-tab="logs">Logs</div>
    </div>
    <div class="content">
        <div id="overview" class="tab-content active">
            <div class="stats-grid">
                <div class="stat-card">
                    <h3>Uptime</h3>
                    <div><span class="value" id="uptime">0</span><span class="unit">sec</span></div>
                </div>
                <div class="stat-card">
                    <h3>Total Requests</h3>
                    <div><span class="value" id="requests">0</span></div>
                </div>
                <div class="stat-card">
                    <h3>Active Clients</h3>
                    <div><span class="value" id="clients">0</span></div>
                </div>
                <div class="stat-card">
                    <h3>Server Version</h3>
                    <div><span class="value" id="version" style="font-size:24px">-</span></div>
                </div>
            </div>
        </div>
        <div id="clients" class="tab-content">
            <button class="refresh-btn" onclick="loadClients()">Refresh</button>
            <table>
                <thead>
                    <tr>
                        <th>Client ID</th>
                        <th>Name</th>
                        <th>Last Seen</th>
                        <th>Requests</th>
                        <th>Permissions</th>
                    </tr>
                </thead>
                <tbody id="clients-table"></tbody>
            </table>
        </div>
        <div id="logs" class="tab-content">
            <button class="refresh-btn" onclick="loadLogs()">Refresh</button>
            <table>
                <thead>
                    <tr>
                        <th>Timestamp</th>
                        <th>Level</th>
                        <th>Message</th>
                    </tr>
                </thead>
                <tbody id="logs-table"></tbody>
            </table>
        </div>
    </div>
    <script>
        document.querySelectorAll('.tab').forEach(tab => {
            tab.addEventListener('click', () => {
                document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
                document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
                tab.classList.add('active');
                document.getElementById(tab.dataset.tab).classList.add('active');
            });
        });

        async function loadStats() {
            try {
                const res = await fetch('/dashboard/api/stats');
                const data = await res.json();
                document.getElementById('uptime').textContent = data.uptime_seconds;
                document.getElementById('requests').textContent = data.total_requests;
                document.getElementById('clients').textContent = data.active_clients;
                document.getElementById('version').textContent = data.server_version;
            } catch(e) { console.error('Failed to load stats', e); }
        }

        async function loadClients() {
            try {
                const res = await fetch('/dashboard/api/clients');
                const data = await res.json();
                const tbody = document.getElementById('clients-table');
                tbody.innerHTML = data.map(c => `
                    <tr>
                        <td><code>${c.id.substring(0,8)}...</code></td>
                        <td>${c.name}</td>
                        <td>${new Date(c.last_seen).toLocaleString()}</td>
                        <td>${c.request_count}</td>
                        <td>${c.permissions.map(p => `<span class="badge badge-info">${p}</span>`).join(' ')}</td>
                    </tr>
                `).join('');
            } catch(e) { console.error('Failed to load clients', e); }
        }

        async function loadLogs() {
            try {
                const res = await fetch('/dashboard/api/logs');
                const data = await res.json();
                const tbody = document.getElementById('logs-table');
                tbody.innerHTML = data.map(l => `
                    <tr class="log-entry">
                        <td>${new Date(l.timestamp).toLocaleString()}</td>
                        <td><span class="badge badge-${l.level === 'ERROR' ? 'error' : l.level === 'WARN' ? 'warn' : 'success'}">${l.level}</span></td>
                        <td>${l.message}</td>
                    </tr>
                `).join('');
            } catch(e) { console.error('Failed to load logs', e); }
        }

        loadStats();
        loadClients();
        loadLogs();
        setInterval(loadStats, 5000);
    </script>
</body>
</html>
"#;
