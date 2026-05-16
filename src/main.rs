use anyhow::Result;
use chrono::Local;
use rtgc::app::App;
use rtgc::audio::audio_manager::RtgcAudioManager;
use rtgc::platform::paths::AppPaths;
use rtgc::platform::window;
use std::sync::{Arc, Mutex};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use winit::event_loop::EventLoop;

static LOG_GUARD: Mutex<Option<tracing_appender::non_blocking::WorkerGuard>> = Mutex::new(None);
static LOG_PATH: Mutex<String> = Mutex::new(String::new());

fn write_panic_to_fallback(msg: &str, location: &str) {
    let fallback_log = std::path::PathBuf::from("rtgc_debug.log");
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&fallback_log)
    {
        use std::io::Write;
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "[{}] PANIC at {}: {}", ts, location, msg).ok();
    }
    eprintln!("[CRITICAL] {} at {}", msg, location);
    eprintln!("[CRITICAL] Log: {}", LOG_PATH.lock().unwrap());
}

fn setup_logging(log_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(log_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create log dir: {}", e))?;

    let abs_path = log_dir.to_string_lossy().to_string();
    *LOG_PATH.lock().unwrap() = abs_path.clone();

    // Fallback: write to project directory so we can always find it
    let fallback_log = std::path::PathBuf::from("rtgc_debug.log");
    {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&fallback_log)
            .map_err(|e| anyhow::anyhow!("Failed to open fallback log: {}", e))?;
        use std::io::Write;
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "[{}] === RTGC STARTUP ===", ts).ok();
        writeln!(file, "[{}] Log dir: {}", ts, abs_path).ok();
    }

    tracing::info!("[RTGC] Log directory: {}", abs_path);

    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "rtgc.log");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let stdout_layer = fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_line_number(true);

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("rtgc=debug,warn"));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(stdout_layer)
        .init();

    *LOG_GUARD.lock().unwrap() = Some(guard);

    tracing::info!("[RTGC] Logging initialized, file: {}/rtgc.log", abs_path);
    eprintln!("[LOG] Log file: {}\\rtgc.log", abs_path);

    Ok(())
}

fn flush_logs() {
    if let Some(guard) = LOG_GUARD.lock().unwrap().take() {
        drop(guard);
    }
}

fn main() -> Result<()> {
    let paths = AppPaths::resolve()?;
    paths.ensure_directories()?;

    setup_logging(&paths.logs_dir)?;

    tracing::info!("RTGC-1.0 запускается...");
    tracing::info!("Версия: 1.0.0-dev");
    tracing::info!("Platform: {}", std::env::consts::OS);

    std::panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown".to_string());

        tracing::error!("PANIC: {} at {}", msg, location);
        write_panic_to_fallback(&msg, &location);
        flush_logs();
    }));

    let event_loop = EventLoop::new()?;

    let (window, gl_surface, gl_context) =
        window::create_window(&event_loop, 1280, 720, "RLG 1.0")?;

    let audio = RtgcAudioManager::new()?;
    let mut app = App::new(Arc::new(gl_context), gl_surface, audio, paths, 1280, 720, Some(Arc::new(std::sync::Mutex::new(window))))?;

    tracing::info!("Запуск главного цикла событий...");
    event_loop.run_app(&mut app)?;

    flush_logs();
    tracing::info!("Приложение завершено корректно");
    Ok(())
}
