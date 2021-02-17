extern crate csv;
extern crate reqwest;

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::marker::Unpin;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs::{create_dir_all, File};
use tokio::io;
use tokio::io::AsyncRead;

struct DownloadProgress<R: AsyncRead + Unpin> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: AsyncRead + Unpin> AsyncRead for DownloadProgress<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let s = self.get_mut();
        let p = AsyncRead::poll_read(Pin::new(&mut s.inner), ctx, buf);
        if let Poll::Ready(Ok(res)) = p {
            s.progress_bar.inc(res as u64);
        }
        p
    }
}

fn get_sentences_url(lang: &str) -> String {
    format!(
        "https://downloads.tatoeba.org/exports/per_language/{}/{}_sentences.tsv.bz2",
        lang, lang
    )
}

// fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//     self.inner.read(buf).map(|n| {
//         self.progress_bar.inc(n as u64);
//         n
//     })
// }

async fn fetch_all_csv(languages: Vec<String>) -> Result<()> {
    let dir = Path::new("./data/tatoeba");
    let m = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");
    let client = reqwest::Client::new();
    for lang in languages {
        let lang_dir_path = dir.join(Path::new(&lang));
        create_dir_all(&lang_dir_path).await?;
        let mut file = File::create(lang_dir_path.join(Path::new("sentences.tsv.bz2"))).await?;
        let url = "http://ipv4.download.thinkbroadband.com/20MB.zip".to_string(); // get_sentences_url(&lang);
        // let mut res = client.head(&url).send().await?;
        // println!("Got initial response");
        let res = reqwest::get(&url).await?;
        let content_length = res.content_length().unwrap();
        let pb = ProgressBar::new(content_length);
        pb.set_style(sty.clone());
        let bytes = res.bytes().await?;
        let mut source = DownloadProgress {
            progress_bar: pb,
            inner: &mut &*bytes,
        };
        tokio::io::copy(&mut source, &mut file).await?;
        // m.add(pb);
        // m.join_and_clear().unwrap();
        source.progress_bar.finish_with_message("done");
    }
    Ok(())
}

fn sanitize_csv(reader: impl std::io::Read, writer: impl std::io::Write) -> Result<()> {
    let mut rdr = csv::Reader::from_reader(reader);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Supported languages
    // let languages: Vec<String> = vec!["en".to_string(), "tr".to_string()];
    //let in_file_path = "./data/tatoeba_sentences.csv";
    //let out_file_path = "./data/tatoeba_sentences_sanitized.csv";
    //let in_file = File::open(in_file_path).await?;
    //let out_file = File::open(out_file_path).await?;
    // sanitize_csv(in_file, out_file).unwrap();
    fetch_all_csv(vec!["tur".to_string()]).await?;
    Ok(())
}
