//! MTG AI Suite CLI
//!
//! Command-line interface for managing the MTG AI Suite server.

use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "mtg-ai-suite")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output in JSON format
    #[arg(long)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage API keys for client authentication
    #[command(subcommand)]
    Apikey(ApikeyCommands),

    /// Manage the server
    #[command(subcommand)]
    Server(ServerCommands),

    /// Manage the database
    #[command(subcommand)]
    Db(DbCommands),

    /// Run recognition tests
    #[command(subcommand)]
    Test(TestCommands),
}

#[derive(Subcommand)]
enum ApikeyCommands {
    /// Create a new API key
    Create {
        /// Client name (e.g., "John's iPhone")
        #[arg(short, long)]
        name: String,

        /// Permissions (comma-separated: read,write,ai,push,admin)
        #[arg(short, long, default_value = "read,write,ai,push")]
        permissions: String,

        /// Expiration (e.g., 30d, 1y, never)
        #[arg(long, default_value = "never")]
        expires: String,
    },

    /// List all API keys
    List {
        /// Show only active keys
        #[arg(long)]
        active_only: bool,
    },

    /// Revoke an API key
    Revoke {
        /// Key ID to revoke
        key_id: String,

        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Rotate an API key (generate new key, invalidate old)
    Rotate {
        /// Key ID to rotate
        key_id: String,

        /// Send push notification about new key
        #[arg(long)]
        notify: bool,
    },
}

#[derive(Subcommand)]
enum ServerCommands {
    /// Start the server
    Start {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        /// Number of worker threads
        #[arg(short, long)]
        workers: Option<usize>,

        /// Run as daemon
        #[arg(short, long)]
        daemon: bool,
    },

    /// Stop the server
    Stop,

    /// Show server status
    Status,

    /// View server logs
    Logs {
        /// Follow log output
        #[arg(short, long)]
        follow: bool,

        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
    },

    /// Manage server configuration
    Config {
        /// Action: get, set, list, reset
        action: String,

        /// Config key
        key: Option<String>,

        /// Config value (for set action)
        value: Option<String>,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Run database migrations
    Migrate {
        /// Target migration version
        #[arg(long)]
        target: Option<String>,

        /// Show SQL without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// Seed database with card data
    Seed {
        /// Data source (scryfall, mtgjson)
        #[arg(long, default_value = "scryfall")]
        source: String,

        /// Only fetch new data
        #[arg(long)]
        incremental: bool,
    },

    /// Backup database
    Backup {
        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Compress backup
        #[arg(long)]
        compress: bool,
    },

    /// Restore database from backup
    Restore {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum TestCommands {
    /// Run recognition tests on image catalog
    Recognition {
        /// Path to test image catalog
        #[arg(long, default_value = "./test-data/images")]
        catalog: String,

        /// Output format (text, json, csv)
        #[arg(long, default_value = "text")]
        format: String,
    },

    /// Run performance benchmarks
    Benchmark {
        /// Number of iterations
        #[arg(short, long, default_value = "100")]
        iterations: usize,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load .env if present
    dotenvy::dotenv().ok();

    match cli.command {
        Commands::Apikey(cmd) => match cmd {
            ApikeyCommands::Create {
                name,
                permissions,
                expires,
            } => commands::apikey::create(&name, &permissions, &expires, cli.json).await,
            ApikeyCommands::List { active_only } => {
                commands::apikey::list(active_only, cli.json).await
            }
            ApikeyCommands::Revoke { key_id, force } => {
                commands::apikey::revoke(&key_id, force).await
            }
            ApikeyCommands::Rotate { key_id, notify } => {
                commands::apikey::rotate(&key_id, notify).await
            }
        },
        Commands::Server(cmd) => match cmd {
            ServerCommands::Start {
                port,
                host,
                workers,
                daemon,
            } => commands::server::start(port, &host, workers, daemon).await,
            ServerCommands::Stop => commands::server::stop().await,
            ServerCommands::Status => commands::server::status().await,
            ServerCommands::Logs { follow, lines } => commands::server::logs(follow, lines).await,
            ServerCommands::Config { action, key, value } => {
                commands::server::config(&action, key.as_deref(), value.as_deref()).await
            }
        },
        Commands::Db(cmd) => match cmd {
            DbCommands::Migrate { target, dry_run } => {
                commands::db::migrate(target.as_deref(), dry_run).await
            }
            DbCommands::Seed {
                source,
                incremental,
            } => commands::db::seed(&source, incremental).await,
            DbCommands::Backup { output, compress } => {
                commands::db::backup(&output, compress).await
            }
            DbCommands::Restore { input, force } => commands::db::restore(&input, force).await,
        },
        Commands::Test(cmd) => match cmd {
            TestCommands::Recognition { catalog, format } => {
                commands::test::recognition(&catalog, &format).await
            }
            TestCommands::Benchmark { iterations } => commands::test::benchmark(iterations).await,
        },
    }
}
