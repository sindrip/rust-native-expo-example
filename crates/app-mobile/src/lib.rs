uniffi::setup_scaffolding!();

static RUNTIME: std::sync::LazyLock<tokio::runtime::Runtime> = std::sync::LazyLock::new(|| {
    tokio::runtime::Runtime::new().unwrap_or_else(|e| panic!("failed to create Tokio runtime: {e}"))
});

static TASK_HANDLE: std::sync::LazyLock<parking_lot::Mutex<Option<tokio::task::JoinHandle<()>>>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(None));

#[uniffi::export(callback_interface)]
pub trait BackgroundTaskCallback: Send + Sync {
    fn on_tick(&self, count: u64);
}

#[uniffi::export]
fn add(left: u64, right: u64) -> u64 {
    app_core::add(left, right)
}

#[uniffi::export]
fn subtract(left: u64, right: u64) -> u64 {
    app_core::subtract(left, right)
}

#[uniffi::export]
fn multiply(left: u64, right: u64) -> u64 {
    app_core::multiply(left, right)
}

#[uniffi::export]
fn divide(left: u64, right: u64) -> Option<u64> {
    app_core::divide(left, right)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn delayed_add(left: u64, right: u64, delay_ms: u64) -> u64 {
    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    app_core::add(left, right)
}

#[uniffi::export]
pub fn start_background_task(callback: Box<dyn BackgroundTaskCallback>) {
    let handle = RUNTIME.spawn(async move {
        let mut count = 0u64;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            count += 1;
            callback.on_tick(count);
        }
    });
    if let Some(old) = TASK_HANDLE.lock().replace(handle) {
        old.abort();
    }
}

#[uniffi::export]
pub fn stop_background_task() {
    if let Some(handle) = TASK_HANDLE.lock().take() {
        handle.abort();
    }
}
