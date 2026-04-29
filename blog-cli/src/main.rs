use blog_client::{BlogClient, Transport, error::ClientError};
use clap::{Parser, Subcommand};

const TOKEN_FILE_PATH: &str = ".blog_token";
const DEFAULT_HTTP_HOST: &str = "http://localhost:8080";
const DEFAULT_GRPC_HOST: &str = "http://localhost:50051";

#[derive(Parser)]
#[command(name = "blog-cli", about = "Сlient for the Blog server", version)]
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
    Register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    /// Login
    Login {
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    /// Create post
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    /// Get post
    Get {
        #[arg(long)]
        post_id: i64,
    },
    /// Update post
    Update {
        #[arg(long)]
        post_id: i64,
        #[arg(long)]
        title: String,
        #[arg(long, default_value = "")]
        content: String,
    },
    /// Delete post
    Delete {
        #[arg(long)]
        post_id: i64,
    },
    /// Posts list
    List {
        #[arg(long, default_value = "10")]
        limit: i64,
        #[arg(long, default_value = "0")]
        offset: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let cli = CliParams::parse();

    let cli_param = if cli.grpc {
        (
            cli.server.unwrap_or_else(|| DEFAULT_GRPC_HOST.to_string()),
            Transport::Grpc
        )
    }
    else {
        (
            cli.server.unwrap_or_else(|| DEFAULT_HTTP_HOST.to_string()),
            Transport::Http
        )
    };

    let client = BlogClient::new(cli_param.0, cli_param.1).await?;

    if let Some(token) = read_token() {
        client.set_token(token).await;
    }

    match cli.command {
        Commands::Register { username, email, password } => {
            let res = client.register(&username, &email, &password).await?;
            save_token(&res.token)?;
            if let Some(user) = res.user {
                println!("User has registered.");
                println!("User: {} ({})", user.username, user.email);
            }
            println!("Token is located to: {TOKEN_FILE_PATH}");
        }

        Commands::Login { email, password } => {
            let res = client.login( &email, &password).await?;
            save_token(&res.token)?;
            if let Some(user) = res.user {
                println!("Logged in ({})", user.username);
            }
            println!("Token is located to: {TOKEN_FILE_PATH}");
        }

        Commands::Create { title, content } => {
            let res = client.new_post(&title, &content).await?;
            if let Some(post) = res.post {
                println!("Post # {} has created", post.id);
                println!("Post theme: {}", post.title);
            }
            else {
                println!("{:?}", res)
            }
        }

        Commands::Get { post_id } => {
            let res = client.get_post(post_id).await?;
            if let Some(post) = res.post {
                println!("Post # {} has got:", post.id);
                println!("Post theme: {}", post.title);
            }
        }

        Commands::Update { post_id, title, content } => {
            let res = client.update_post(post_id, &title, &content).await?;
            if let Some(post) = res.post {
                println!("Post # {} has updated:", post.id);
                println!("Post theme: {}", post.title);
            }
        }
        Commands::Delete { post_id } => {
            client.del_post(post_id).await?;
            println!("Post # {post_id} has deleted");
        }

        Commands::List { limit, offset } => {
            let res = client.list_posts( limit, offset).await?;
            println!("{} posts have got", res.posts.len());
            println!("{:<5} {:<20}", "ID", "TITLE");
            println!("{}", "-".repeat(60));
            for p in &res.posts {
                println!("{:<5} {:<20}", p.id, &p.title);
            }
        }
    }

    Ok(())
}

fn read_token() -> Option<String> {
    std::fs::read_to_string(TOKEN_FILE_PATH).ok().map(|s| s.trim().to_string())
}

fn save_token(token: &str) -> Result<(), ClientError> {
    std::fs::write(TOKEN_FILE_PATH, token)?;
    Ok(())
}
