use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::oneshot;

// HTTP 服务器相关
use axum::{extract::State, response::Redirect, routing::get, routing::post, Json, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

// 全局状态：存储服务器关闭信号
static SERVER_SHUTDOWN: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    println!("{}", name);
    return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SendMessage {
    value: f32,
}

#[tauri::command]
fn send_message(app: AppHandle, value: f32) {
    println!("={}", value);
    app.emit("watch_message", SendMessage { value }).unwrap();
}

/// 服务器信息结构
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfo {
    running: bool,
    port: u16,
    local_url: String,
    network_url: String,
}

/// 获取本机局域网 IP
fn get_local_ip() -> String {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

#[derive(Deserialize)]
struct DeviceData {
    pm2: f32,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

async fn set_device(
    State(app): State<AppHandle>,
    Json(payload): Json<DeviceData>,
) -> impl IntoResponse {
    println!("{}", payload.pm2);
    app.emit("watch_message", SendMessage { value: payload.pm2 })
        .unwrap();
    // TODO: 在这里处理设备设置逻辑
    Json(ApiResponse {
        success: true,
        message: "设备设置成功".to_string(),
    })
}

/// 启动 HTTP 服务器
#[tauri::command]
async fn start_h5_server(app: AppHandle, port: u16) -> Result<ServerInfo, String> {
    // 检查是否已经在运行
    {
        let guard = SERVER_SHUTDOWN.lock().unwrap();
        if guard.is_some() {
            return Err("服务器已经在运行中".to_string());
        }
    }

    // 获取资源目录路径
    let resource_path = app
        .path()
        .resource_dir()
        .map_err(|e| format!("获取资源目录失败: {}", e))?
        .join("web");

    if !resource_path.exists() {
        return Err(format!(
            "H5 资源目录不存在: {:?}，请先运行 pnpm build",
            resource_path
        ));
    }

    println!("H5 资源目录: {:?}", resource_path);

    let local_ip = get_local_ip();
    let local_ip_clone = local_ip.clone();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // 创建关闭信号通道
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // 存储关闭信号发送器
    {
        let mut guard = SERVER_SHUTDOWN.lock().unwrap();
        *guard = Some(shutdown_tx);
    }

    let resource_path_clone = resource_path.clone();
    let app_handle = app.clone();

    // 在后台线程启动服务器
    tokio::spawn(async move {
        // 配置 CORS
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // index.html 路径，用于 SPA fallback
        let index_html = resource_path_clone.join("index.html");

        // 创建静态文件服务，支持 SPA 路由 fallback
        let serve_dir = ServeDir::new(&resource_path_clone)
            .append_index_html_on_directories(true)
            .not_found_service(ServeFile::new(&index_html));

        // 根路径重定向到 /setDevice
        let router = Router::new()
            .route("/", get(|| async { Redirect::permanent("/setDevice") }))
            .route("/api/v1/set_device", post(set_device))
            .fallback_service(serve_dir)
            .layer(cors)
            .with_state(app_handle);

        println!("H5 服务器启动在: http://{}:{}", local_ip_clone, port);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, router)
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
                println!("H5 服务器已关闭");
            })
            .await
            .ok();

        // 服务器关闭后清理状态
        let mut guard = SERVER_SHUTDOWN.lock().unwrap();
        *guard = None;
    });

    Ok(ServerInfo {
        running: true,
        port,
        local_url: format!("http://localhost:{}", port),
        network_url: format!("http://{}:{}", local_ip, port),
    })
}

/// 停止 HTTP 服务器
#[tauri::command]
fn stop_h5_server() -> Result<String, String> {
    let mut guard = SERVER_SHUTDOWN.lock().unwrap();
    if let Some(tx) = guard.take() {
        tx.send(()).map_err(|_| "发送关闭信号失败".to_string())?;
        Ok("服务器已停止".to_string())
    } else {
        Err("服务器未运行".to_string())
    }
}

/// 获取服务器状态
#[tauri::command]
fn get_h5_server_status() -> ServerInfo {
    let guard = SERVER_SHUTDOWN.lock().unwrap();
    let running = guard.is_some();
    let local_ip = get_local_ip();

    ServerInfo {
        running,
        port: 8080,
        local_url: "http://localhost:8080".to_string(),
        network_url: format!("http://{}:8080", local_ip),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            send_message,
            start_h5_server,
            stop_h5_server,
            get_h5_server_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
