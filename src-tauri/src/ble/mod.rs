mod central;
pub mod peripheral;
use tauri::async_runtime::Receiver;


// type IMessage = impl Message;
/// BLE 通信的主从端都会实现的 trait
pub trait BLEComm {
    /// 发送一条信息
    fn send(&self, message: String) -> Result<(), String>;

    /// 取出一个接收器
    fn take_recv<'a>(&mut self) -> Receiver<impl Message + 'a>; 

    /// 阻塞直到连接完成
    /// 用于在触碰后等待连接。
    fn connect(&mut self);
}

pub trait Message {
    fn as_str(&self) -> &str;
}

impl Message for tauri_plugin_blep::RecvMessage {
    fn as_str(&self) -> &str {
        &self.msg
    }
}