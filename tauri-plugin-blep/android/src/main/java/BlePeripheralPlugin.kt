package com.plugin.blep

import android.Manifest
import android.app.Activity
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.bluetooth.BluetoothGattService
import android.bluetooth.BluetoothProfile
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Channel
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.util.*

@InvokeArg
class WatchArgs {
    lateinit var channel: Channel
    lateinit var connectNotifier: Channel
    lateinit var uuid: String
}

@TauriPlugin(
  permissions = [
    Permission(strings = [Manifest.permission.BLUETOOTH_CONNECT, Manifest.permission.BLUETOOTH_ADVERTISE, Manifest.permission.BLUETOOTH_SCAN], alias = "bluetooth")
  ]
)
class BlePeripheralPlugin(private val activity: Activity) : Plugin(activity) {

    companion object {
        private const val PERMISSION_REQUEST_CODE = 123
        private val REQUIRED_PERMISSIONS =
                mutableListOf(Manifest.permission.BLUETOOTH_CONNECT, Manifest.permission.BLUETOOTH_ADVERTISE, Manifest.permission.BLUETOOTH_SCAN)
    }

    private lateinit var blePeripheral: BlePeripheralUtils
    private lateinit var notifyCharacteristic: BluetoothGattCharacteristic
    private var recvChannel: Channel? = null
    private var connectChannel: Channel? = null
    private var customUuid: String? = null

    private val connectedDevice: BluetoothDevice?
        get() = blePeripheral.getConnectedDevices().firstOrNull()

    init {}

    private fun checkAndRequestPermissions() {
        Log.i("ble peri", "checking")
        if (hasAllPermissions()) {
            Log.i("ble peri", "has all permission")
            setupBlePeripheral()
        } else {
            Log.i("ble peri", "no permission")
        }
    }

    fun onActivityResult(requestCode: Int, resultCode: Int) {
        when (requestCode) {
            PERMISSION_REQUEST_CODE -> {
                if (hasAllPermissions()) {
                    setupBlePeripheral()
                } else {
                    Log.e("BlePlugin", "Required permissions denied")
                }
            }
        }
    }
    private fun hasAllPermissions(): Boolean {
        return REQUIRED_PERMISSIONS.all {
            ContextCompat.checkSelfPermission(activity, it) == PackageManager.PERMISSION_GRANTED
        }
    }

    @Command
    fun setup(invoke: Invoke) {
        var args = invoke.parseArgs(WatchArgs::class.java)
        recvChannel = args.channel
        connectChannel = args.connectNotifier
        customUuid = args.uuid
        checkAndRequestPermissions()
    }

    private fun setupBlePeripheral() {
        if (!hasAllPermissions()) {
            Log.e("BlePlugin", "Attempted setup without permissions")
            return
        }

        blePeripheral =
                BlePeripheralUtils(activity).apply {
                    init()

                    val serviceUuid = UUID.fromString(customUuid)
                    val characteristicUuid = UUID.fromString(customUuid)

                    addServices(
                            BlePeripheralUtils.BluetoothGattServiceInfo(
                                    serviceUuid,
                                    BluetoothGattService.SERVICE_TYPE_PRIMARY,
                                    listOf(
                                            BlePeripheralUtils.BluetoothGattCharacteristicInfo(
                                                    characteristicUuid,
                                                    BluetoothGattCharacteristic.PROPERTY_WRITE or
                                                            BluetoothGattCharacteristic
                                                                    .PROPERTY_NOTIFY,
                                                    BluetoothGattCharacteristic.PERMISSION_WRITE,
                                                    BlePeripheralUtils.BluetoothGattDescriptorInfo(
                                                            UUID.fromString(
                                                                    "00002902-0000-1000-8000-00805f9b34fb"
                                                            ),
                                                            BluetoothGattDescriptor.PERMISSION_WRITE
                                                    )
                                            )
                                    )
                            )
                    )

                    // Log.i("ble peri", "service added")


                    // Log.i("ble peri", "notify chara")
                    blePeripheralCallback =
                            object : BlePeripheralUtils.BlePeripheralCallback {
                                override fun onConnectionStateChange(
                                        device: BluetoothDevice,
                                        status: Int,
                                        newState: Int
                                ) {
                                    if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                                        connectChannel?.send(
                                                JSObject().apply { put("type", "Disconnected") }
                                        )
                                    } else if (newState == BluetoothProfile.STATE_CONNECTED) {
                                        connectChannel?.send(
                                                JSObject().apply { put("type", "Connected") }
                                        )
                                    }
                                }

                                override fun onCharacteristicWriteRequest(
                                        device: BluetoothDevice,
                                        requestId: Int,
                                        characteristic: BluetoothGattCharacteristic,
                                        preparedWrite: Boolean,
                                        responseNeeded: Boolean,
                                        offset: Int,
                                        value: ByteArray
                                ) {
                                    activity.runOnUiThread {
                                        try {
                                            recvChannel?.send(
                                                    JSObject().apply { put("msg", String(value)) }
                                            )
                                        } catch (e: IllegalStateException) {
                                            recvChannel = null
                                        }
                                    }
                                }
                            }

                    try {
                        Log.i("ble per", "start")
                        startBluetoothLeAdvertiser("TauriBleDevice", byteArrayOf(), serviceUuid)
                        Log.i("ble per", "started")
                    } catch (e: SecurityException) {
                        Log.e("BlePlugin", "Bluetooth operation failed: ${e.message}")
                    }
                }
    }

    @Command
    fun send(invoke: Invoke) {
        if (!hasAllPermissions()) {
            invoke.reject("Missing required permissions")
            return
        }

        if (!::notifyCharacteristic.isInitialized) {
            var serviceUuid = UUID.fromString(customUuid)
            var characteristicUuid = UUID.fromString(customUuid)
            notifyCharacteristic = blePeripheral?.getCharacteristic(serviceUuid, characteristicUuid)!!
        }

        val args = invoke.parseArgs(SendArgs::class.java)
        val success =
                if (connectedDevice != null && ::notifyCharacteristic.isInitialized) {
                    try {
                        blePeripheral.notifyDevice(
                                connectedDevice!!,
                                notifyCharacteristic,
                                args.message.toByteArray()
                        )
                    } catch (e: SecurityException) {
                        false
                    }
                } else {
                    false
                }

        val ret = JSObject().apply { put("success", success) }
        invoke.resolve(ret)
    }

    class SendArgs {
        lateinit var message: String
    }
}
