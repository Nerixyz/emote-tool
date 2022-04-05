mod avif;
mod cli;
mod ffmpeg;
mod task;
mod webp;

use crate::{
    avif::task::AvifEncoderTask,
    cli::{AvifCommand, Cli, CliCommand, IoOptions, WebpCommand},
    ffmpeg::{formats::AcceptedFormats, frames::extract_frames},
    task::EncoderTask,
    webp::task::WebpEncoderTask,
};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;

fn main() {
    ffmpeg_next::init().unwrap();
    ffmpeg_next::log::set_level(ffmpeg_next::log::Level::Warning);

    match Cli::parse().command {
        CliCommand::Avif(AvifCommand { io, opts }) => run_task::<AvifEncoderTask, _>(io, opts),
        CliCommand::Webp(WebpCommand { io, opts }) => run_task::<WebpEncoderTask, _>(io, opts),
    }
}

fn run_task<T, A>(io_options: IoOptions, task_options: A)
where
    T: EncoderTask<CliArgs = A>,
{
    let (input_ctx, istream_idx) = ffmpeg::read_initial_stream(&io_options.input).unwrap();
    let istream = input_ctx.stream(istream_idx).unwrap();
    let frames = extract_frames(&istream, &input_ctx);
    assert!(frames > 0);
    let frames = frames as u64;

    let config = T::configure(task_options, &istream, &input_ctx).unwrap();
    let accepted_formats = AcceptedFormats::for_task::<T>();

    let out_file = T::make_output_path(&io_options.output);
    let writer = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(&out_file)
        .unwrap();

    let is_single_frame = istream.frames() == 1;
    let (frame_tx, frame_rx) = crossbeam::channel::bounded(if is_single_frame { 1 } else { 20 });

    let progress_manager = MultiProgress::new();
    let ffmpeg_progress = progress_manager.add(ProgressBar::new(frames).with_style(
        ProgressStyle::default_bar().template("[Ffmpeg {spinner}] Frame {pos}/{len} ({percent}%) {bar:20} {elapsed}/{eta} {msg}").unwrap()));
    let task_progress = progress_manager.add(ProgressBar::new(frames).with_style(
        ProgressStyle::default_bar().template("[Encoder {spinner}] Frame {pos}/{len} ({percent}%) {bar:20} {elapsed}/{eta} {msg}").unwrap()));

    let ffmpeg_thread = thread::spawn(move || {
        ffmpeg::emit_frames(
            input_ctx,
            istream_idx,
            accepted_formats,
            frame_tx,
            ffmpeg_progress,
        )
    });

    let task_thread = thread::spawn(move || {
        if is_single_frame {
            T::run_still(writer, config, frame_rx, task_progress)
        } else {
            T::run_animation(writer, config, frame_rx, task_progress)
        }
    });
    let ff_result = ffmpeg_thread.join();
    let task_result = task_thread.join();

    if matches!((&ff_result, &task_result), (Ok(Ok(_)), Ok(Ok(_)))) {
        let stats = task_result.unwrap().unwrap();
        println!("Finished: {}", stats);
        println!(
            "Written to {}",
            out_file.as_os_str().to_str().unwrap_or("<invalid UTF8>")
        );
    } else {
        match ff_result {
            Ok(Ok(_)) => eprintln!("[Ffmpeg] Finished without errors."),
            Ok(Err(e)) => eprintln!("[Ffmpeg] Errored: {}", e),
            Err(_) => eprintln!("[Ffmpeg] Thread panicked."),
        }
        match task_result {
            Ok(Ok(_)) => eprintln!("[Encoder] Finished without errors."),
            Ok(Err(e)) => eprintln!("[Encoder] Errored: {}", e),
            Err(_) => eprintln!("[Encoder] Thread panicked."),
        }
        eprintln!("Some thread panicked or returned an error!");
        std::process::exit(-1);
    }
}
