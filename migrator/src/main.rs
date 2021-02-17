use anyhow::Result;
use clap::Clap;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::fs::{create_dir, read_dir, read_to_string, File, ReadDir};
use std::path::{Path, PathBuf};
use std::time;

async fn run_migration(pool: &PgPool, path: &PathBuf) -> Result<()> {
    let contents = read_to_string(path).expect("Unable to read file");
    println!("Running migration {:?}", path);
    sqlx::query(&contents).execute(pool).await?;
    Ok(())
}

async fn up(pool: &PgPool, paths: &mut ReadDir) -> Result<()> {
    for path in paths {
        let up = path.unwrap().path().join(Path::new("up.sql"));
        run_migration(pool, &up).await?
    }
    Ok(())
}

async fn last_down(pool: &PgPool, paths: &mut ReadDir) -> Result<()> {
    let last = paths.last().unwrap();
    if let Ok(last) = last {
        let down = last.path().join(Path::new("down.sql"));
        run_migration(pool, &down).await?
    }
    Ok(())
}

fn timestamp() -> u64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("Impossible")
        .as_secs()
}

fn generate(migrations_dir: &str, migration_name: &str) {
    let dir_name = format!("{}_{}", timestamp(), migration_name);
    let dir_path = Path::new(migrations_dir).join(Path::new(&dir_name));
    create_dir(&dir_path).expect("Unable to create dir");
    File::create(dir_path.join(Path::new("up.sql"))).expect("Error creating up.sql");
    File::create(dir_path.join(Path::new("down.sql"))).expect("Error creating down.sql");
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Vance Longwill <vancelongwill@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0", author = "Vance Longwill <vancelongwill@gmail.com>")]
    Up(Up),
    Down(Down),
    Generate(Generate),
}

#[derive(Clap)]
struct Up {}

#[derive(Clap)]
struct Down {}

#[derive(Clap)]
struct Generate {
    #[clap(index = 1)]
    migration_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let migrations_dir = String::from("./migrations");
    let pool = PgPool::new(&database_url).await?;
    let mut paths = read_dir(&migrations_dir).unwrap();
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Up(_) => {
            up(&pool, &mut paths).await?;
        }
        SubCommand::Down(_) => {
            last_down(&pool, &mut paths).await?;
        }
        SubCommand::Generate(g) => {
            generate(&migrations_dir, &g.migration_name);
        }
    }

    Ok(())
}
