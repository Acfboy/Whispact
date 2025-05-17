use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    BleCentralDiscover(String),
    BleCentralConnect(String),
    BleCentralSubscribe(String),
    BleCentralDeviceNotFound,
    BleCenteralSendDataFailed(String),
    LastMessageNotSend,
    SendBeforeConnect,
    ReceiveBeforeConnect,
    ConnectBeforeSetup,
    BlePeripheralSendFail(String),
    RequestBlueTooth(String),
    InitNfc(String),
}
