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
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Channel
import java.util.*
import com.plugin.blep.WatchArgs


@InvokeArg
class WatchArgs {
    lateinit var channel: Channel
}

@TauriPlugin
class BlePeripheralPlugin(private val activity: Activity) : Plugin(activity) {

    companion object {
        private const val PERMISSION_REQUEST_CODE = 123
        private val REQUIRED_PERMISSIONS = mutableListOf(
            Manifest.permission.BLUETOOTH,
            Manifest.permission.BLUETOOTH_ADMIN
        ).apply {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                add(Manifest.permission.BLUETOOTH_CONNECT)
                add(Manifest.permission.BLUETOOTH_ADVERTISE)
            }
        }.toTypedArray()
    }

    private lateinit var blePeripheral: BlePeripheralUtils
    private lateinit var notifyCharacteristic: BluetoothGattCharacteristic
    private var recvChannel: Channel? = null
    private var connectChannel: Channel? = null

    private val connectedDevice: BluetoothDevice?
        get() = blePeripheral.getConnectedDevices().firstOrNull()

    init {}

    private fun checkAndRequestPermissions() {
        if (hasAllPermissions()) {
            setupBlePeripheral()
        } else {
            ActivityCompat.requestPermissions(
                activity,
                REQUIRED_PERMISSIONS,
                PERMISSION_REQUEST_CODE
            )
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
        var args = invoke.parseArgs(WatchArgs::class.java);
        recvChannel = args.channel
        connectChannel = args.connectNotifier
        checkAndRequestPermissions()
    }

    private fun setupBlePeripheral() {
        if (!hasAllPermissions()) {
            Log.e("BlePlugin", "Attempted setup without permissions")
            return
        }

        blePeripheral = BlePeripheralUtils(activity).apply {
            init()
            
            val serviceUuid = UUID.fromString("0000ffe0-0000-1000-8000-00805f9b34fb")
            val characteristicUuid = UUID.fromString("0000ffe1-0000-1000-8000-00805f9b34fb")
            
            addServices(
                BlePeripheralUtils.BluetoothGattServiceInfo(
                    serviceUuid,
                    BluetoothGattService.SERVICE_TYPE_PRIMARY,
                    listOf(
                        BlePeripheralUtils.BluetoothGattCharacteristicInfo(
                            characteristicUuid,
                            BluetoothGattCharacteristic.PROPERTY_WRITE or
                                    BluetoothGattCharacteristic.PROPERTY_NOTIFY,
                            BluetoothGattCharacteristic.PERMISSION_WRITE,
                            BlePeripheralUtils.BluetoothGattDescriptorInfo(
                                UUID.fromString("00002902-0000-1000-8000-00805f9b34fb"),
                                BluetoothGattDescriptor.PERMISSION_WRITE
                            )
                        )
                    )
                )
            )
            
            notifyCharacteristic = getCharacteristic(serviceUuid, characteristicUuid)!!
            
            blePeripheralCallback = object : BlePeripheralUtils.BlePeripheralCallback {
                override fun onConnectionStateChange(
                    device: BluetoothDevice, 
                    status: Int, 
                    newState: Int
                ) {
                    if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                        connectChannel.send(JSObject().apply {
                            put("type", "Disconnected")
                        })
                    } else if (newState = BluetoothProfile.STATE_CONNECTED) {
                        connectChannel.send(JSObject().apply {
                            put("type", "Connected")
                        })
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
                            recvChannel?.send(JSObject().apply {
                                put("msg", String(value))
                            }) 
                        } catch (e: IllegalStateException) {
                            recvChannel = null
                        }
                    }
                }
            }
            
            try {
                startBluetoothLeAdvertiser("TauriBleDevice", byteArrayOf(), serviceUuid)
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
        
        val args = invoke.parseArgs(SendArgs::class.java)
        val success = if (connectedDevice != null && ::notifyCharacteristic.isInitialized) {
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
        
        val ret = JSObject().apply {
            put("success", success)
        }
        invoke.resolve(ret)
    }

    class SendArgs {
        lateinit var message: String
    }
}