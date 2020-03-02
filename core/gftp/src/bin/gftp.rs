use anyhow::Result;
use log::info;
use std::path::PathBuf;
use structopt::StructOpt;
use ya_core_model::ethaddr::NodeId;

#[derive(StructOpt)]
pub enum CmdLine {
    Publish {
        #[structopt(short = "f", long = "file", help = "File to publish")]
        path: PathBuf,
    },
    Download {
        node_id: NodeId,
        hash: String,
        output_file: PathBuf,
    },
}

#[actix_rt::main]
async fn main() -> Result<()> {
    //std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
    env_logger::init();

    let cmd_args = CmdLine::from_args();

    let config = gftp::Config {
        chunk_size: 40 * 1024,
    };

    match cmd_args {
        CmdLine::Publish { path } => {
            let url = config.publish(&path).await?;

            info!(
                "Published file [{}] as {}.",
                &path.display(),
                url,
            );

            actix_rt::signal::ctrl_c().await?;
            info!("Received ctrl-c signal. Shutting down.")
        }
        CmdLine::Download {
            node_id,
            hash,
            output_file,
        } => {
            info!(
                "Downloading file [{}] from [{:?}], target path [{}].",
                &hash,
                node_id,
                output_file.display()
            );

            gftp::download_file(node_id, &hash, &output_file).await?;
            info!("File downloaded.")
        }
    }
    Ok(())
}
