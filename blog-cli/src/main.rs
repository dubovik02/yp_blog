use clap::Parser;

const TOKEN_FILE_PATH: &str = ".blog_token";
const DEFAULT_HTTP_HOST: &str = "http://localhost:8080";
const DEFAULT_GRPC_HOST: &str = "http://localhost:50051";

#[derive(Parser)]
#[command(name = "blog-cli", about = "Сlient for thr Blog server", version)]
struct CliParams {
    #[arg(long, global = true)]
    grpc: bool,
    #[arg(long, global = true)]
    server: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Register
    register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    /// Login
    login {
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    /// Create post
    create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
        #[arg(long)]
        author_id: i64,
    },
    /// Get post
    get {
        #[arg(long)]
        post_id: i64,
    },
    /// Update post
    update {
        #[arg(long)]
        post_id: i64,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        content: String,
    },
    /// Delete post
    delete {
        #[arg(long)]
        post_id: i64,
    },
    /// Posts list
    list {
        #[arg(long, default_value = "10")]
        limit: i64,
        #[arg(long, default_value = "0")]
        offset: i64,
    },
}

fn load_token() -> Option<String> {
    std::fs::read_to_string(TOKEN_FILE_PATH).ok().map(|s| s.trim().to_string())
}

fn save_token(token: &str) -> Result<()> {
    std::fs::write(TOKEN_FILE_PATH, token)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliParams::parse();

    let transport = if cli.grpc {
        let addr = cli.server.unwrap_or_else(|| DEFAULT_GRPC.to_string());
        Transport::Grpc(addr)
    } else {
        let addr = cli.server.unwrap_or_else(|| DEFAULT_HTTP.to_string());
        Transport::Http(addr)
    };

    let client = BlogClient::new(transport).await?;

    if let Some(token) = load_token() {
        client.set_token(token).await;
    }

    match cli.command {
        Commands::register { username, email, password } => {
            let res = client.register(&username, &email, &password).await?;
            save_token(&res.token)?;
            println!("Registered successfully!");
            println!("User: {} ({})", res.user.username, res.user.email);
            println!("Token saved to {TOKEN_FILE_PATH}");
        }
        Commands::Login { username, password } => {
            let resp = client.login(&username, &password).await?;
            save_token(&resp.token)?;
            println!("Logged in as {}", resp.user.username);
            println!("Token saved to {TOKEN_FILE_PATH}");
        }
        Commands::Create { title, content } => {
            let post = client.create_post(&title, &content).await?;
            println!("Post created (id={}):", post.id);
            println!("  Title:   {}", post.title);
            println!("  Author:  {}", post.author_username);
            println!("  Created: {}", post.created_at);
        }
        Commands::Get { id } => {
            let post = client.get_post(id).await?;
            println!("Post #{}:", post.id);
            println!("  Title:   {}", post.title);
            println!("  Author:  {}", post.author_username);
            println!("  Content: {}", post.content);
            println!("  Created: {}", post.created_at);
        }
        Commands::Update { id, title, content } => {
            let post = client.update_post(id, &title, &content).await?;
            println!("Post #{} updated:", post.id);
            println!("  Title:   {}", post.title);
            println!("  Updated: {}", post.updated_at);
        }
        Commands::Delete { id } => {
            client.delete_post(id).await?;
            println!("Post #{id} deleted");
        }
        Commands::List { limit, offset } => {
            let result = client.list_posts(limit, offset).await?;
            println!("Posts ({}/{}):", result.posts.len(), result.total);
            println!("{:<5} {:<30} {:<20}", "ID", "TITLE", "AUTHOR");
            println!("{}", "-".repeat(60));
            for p in &result.posts {
                println!("{:<5} {:<30} {:<20}", p.id, &p.title, &p.author_username);
            }
        }
    }

    Ok(())
}
