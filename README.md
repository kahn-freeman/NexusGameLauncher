# NexusLauncher-Game Launcher
A Game Launcher development with Rust + Slint

# Comparison: Slint vs Electron vs Tauri

| **Feature**       | **Slint**                  | **Electron**                     | **Tauri**                          |
|------------------|---------------------------|----------------------------------|------------------------------------|
| **Size**         | Small (< 10MB)              | Large (50MB+)                    | Medium (< 10MB or > 50MB)                   |
| **Performance**  | High (Rust + native rendering) | Low (Based on Chromium)         | Low (Relies on system WebView)    |
| **Cross-Platform** | Good (Cross-platform) | Good (Cross-platform but resource-heavy) | Good (Supports Windows, macOS, Linux) |
| **Reliability**  | High (Rust-based)          | High (Mature and widely used)    | Medium (Depends on system WebView) |
