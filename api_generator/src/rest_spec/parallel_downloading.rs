use super::RestApiSpec;

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use rayon::prelude::*;
use std::error::Error as StdError;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, copy, Read};
use std::path::PathBuf;
use std::thread;

#[derive(Debug)]
pub(super) enum DownloadSpecsErrors {
    IoErr(io::Error),
    HttpError(reqwest::Error),
    Cumulative(Vec<DownloadSpecsErrors>),
}

/// Downloads the given specs to the provided director in parallel, displaying progress bars for
/// each file.
pub(super) fn download_specs_to_dir(
    client: reqwest::blocking::Client,
    specs: &[RestApiSpec],
    download_dir: &PathBuf,
) -> Result<(), DownloadSpecsErrors> {
    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} {msg} [{elapsed_precise} (ETA: {eta})] [{bar:40.cyan/blue}] {bytes}/{total_bytes}")
        .progress_chars("#>-");

    let max_name = specs
        .iter()
        .max_by(|a, b| a.name.len().cmp(&b.name.len()))
        .map(|r| r.name.len())
        .unwrap_or(0)
        + 1;

    // We need to chunk it because none of the progress bar libs offer good support for simultaneously
    // showing more progress bars than there are terminal rows.
    let results = specs
        .chunks(rayon::current_num_threads())
        .map(|specs_group| {
            let multibar = MultiProgress::with_draw_target(ProgressDrawTarget::stderr_with_hz(60));
            let download_packs = specs_group
                .iter()
                .map(|spec| {
                    let progress_bar = multibar.add(ProgressBar::new(spec.size as u64));
                    progress_bar.set_style(sty.clone());
                    progress_bar.set_message(&right_pad(&spec.name, max_name));
                    (spec.name.clone(), spec.download_url.clone(), progress_bar)
                })
                .collect::<Vec<_>>();

            let finish = thread::spawn(move || multibar.join());

            let results = download_packs
                .into_par_iter()
                .map(|(name, download_url, progress_bar)| {
                    let mut download_path = download_dir.clone();
                    download_path.push(&name);
                    let request = client.get(&download_url);
                    let mut file = File::create(download_path.to_string_lossy().into_owned())?;
                    let mut source = ReadWithProgress {
                        progress_bar: &progress_bar,
                        inner: request.send()?,
                    };

                    copy(&mut source, &mut file)?;
                    progress_bar.finish();
                    Ok(())
                })
                .collect::<Vec<Result<(), DownloadSpecsErrors>>>();

            finish
                .join()
                .expect("Failed to join a thread. Unrecoverable, so bailing.")?;
            ok_or_accumulate(results)
        })
        .collect::<Vec<Result<(), DownloadSpecsErrors>>>();

    ok_or_accumulate(results)
}

fn ok_or_accumulate(
    results: Vec<Result<(), DownloadSpecsErrors>>,
) -> Result<(), DownloadSpecsErrors> {
    let errs = results
        .into_iter()
        .filter_map(|r| r.err())
        .collect::<Vec<_>>();
    if errs.is_empty() {
        Ok(())
    } else {
        Err(DownloadSpecsErrors::Cumulative(errs))
    }
}

fn right_pad(s: &str, pad: usize) -> String {
    let mut out = String::from(s);
    let len = s.len();
    if pad > len {
        for _ in 0..pad - len {
            out.push(' ');
        }
    }
    out
}

/// A thin wrapper around another `Reader` so that we can report on progress
struct ReadWithProgress<'bar, R> {
    inner: R,
    progress_bar: &'bar ProgressBar,
}

impl<'bar, R> Read for ReadWithProgress<'bar, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

impl std::fmt::Display for DownloadSpecsErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DownloadSpecsErrors::IoErr(err) => write!(f, "IoErr {}", err),
            DownloadSpecsErrors::HttpError(err) => write!(f, "HttpError {}", err),
            DownloadSpecsErrors::Cumulative(errs) => write!(f, "Cumulative {:?}", errs),
        }
    }
}

impl StdError for DownloadSpecsErrors {
    #[allow(warnings)]
    fn description(&self) -> &str {
        match self {
            DownloadSpecsErrors::IoErr(err) => err.description(),
            DownloadSpecsErrors::HttpError(err) => err.description(),
            DownloadSpecsErrors::Cumulative(_) => "Cumulative errors",
        }
    }
}

impl From<io::Error> for DownloadSpecsErrors {
    fn from(e: io::Error) -> Self {
        DownloadSpecsErrors::IoErr(e)
    }
}

impl From<reqwest::Error> for DownloadSpecsErrors {
    fn from(e: reqwest::Error) -> Self {
        DownloadSpecsErrors::HttpError(e)
    }
}
