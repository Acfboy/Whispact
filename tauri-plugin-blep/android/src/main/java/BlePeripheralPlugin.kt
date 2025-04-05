package com.plugin.blep

import android.Manifest
import android.app.Activity
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.bluetooth.BluetoothGattService
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log  
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.util.*
import java.util.concurrent.ConcurrentLinkedQueue

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

    private val receiveQueue = ConcurrentLinkedQueue<String>()
    private lateinit var blePeripheral: BlePeripheralUtils
    private lateinit var notifyCharacteristic: BluetoothGattCharacteristic

    private val connectedDevice: BluetoothDevice?
        get() = blePeripheral.getConnectedDevices().firstOrNull()

    init {
        checkAndRequestPermissions()
    }

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

    // 添加Activity结果回调处理
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
                ) {}

                override fun onCharacteristicWriteRequest(
                    device: BluetoothDevice,
                    requestId: Int,
                    characteristic: BluetoothGattCharacteristic,
                    preparedWrite: Boolean,
                    responseNeeded: Boolean,
                    offset: Int,
                    value: ByteArray
                ) {
                    receiveQueue.add(String(value))
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
    fun tryRecv(invoke: Invoke) {
        val ret = JSObject().apply {
            put("message", receiveQueue.poll() ?: "")
        }
        invoke.resolve(ret)
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