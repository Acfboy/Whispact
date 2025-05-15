use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    BleCentralDiscoverError(String),
    BleCentralConnectError(String),
    BleCentralSubscribeError(String),
    BleCentralDeviceNotFound,
    BleCenteralSendDataFailed(String),
    LastMessageNotSend,
    SendBeforeConnect,
    ReceiveBeforeConnect,
    ConnectBeforeSetup,
    BlePeripheralSendFail(String),
    SetHceError(String),
    RequestBlueToothError(String),
    InitNfcError(String),
}