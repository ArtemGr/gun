#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub mod tungstenite;
#[cfg(not(target_arch = "wasm32"))]
pub mod websockets;
#[cfg(target_arch = "wasm32")]
pub mod websockets_wasm;
