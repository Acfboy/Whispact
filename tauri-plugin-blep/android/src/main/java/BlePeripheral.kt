package com.plugin.blep

import android.bluetooth.*
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.BluetoothLeAdvertiser
import android.content.Context
import android.os.ParcelUuid
import android.util.Log
import java.lang.reflect.Method
import java.util.*

class BlePeripheralUtils(private val context: Context) {
    companion object {
        private const val TAG = "BlePeripheralUtils"
    }

    private var mBluetoothAdapter: BluetoothAdapter? = null
    private var bluetoothManager: BluetoothManager? = null
    private var bluetoothLeAdvertiser: BluetoothLeAdvertiser? = null
    private var bluetoothGattServer: BluetoothGattServer? = null

    private val deviceArrayList = ArrayList<BluetoothDevice>()
    var blePeripheralCallback: BlePeripheralCallback? = null

    fun getConnectedDevices(): List<BluetoothDevice> = deviceArrayList.toList()

    private val callback = object : AdvertiseCallback() {
        override fun onStartSuccess(settingsInEffect: AdvertiseSettings) {
            Log.d(TAG, "BLE advertisement added successfully")
        }

        override fun onStartFailure(errorCode: Int) {
            Log.e(TAG, "Failed to add BLE advertisement, reason: $errorCode")
        }
    }


    private val bluetoothGattServerCallback = object : BluetoothGattServerCallback() {
        override fun onConnectionStateChange(device: BluetoothDevice, status: Int, newState: Int) {
            Log.e(TAG, "1.onConnectionStateChange：device name = ${device.name}, address = ${device.address}")
            Log.e(TAG, "1.onConnectionStateChange：status = $status, newState = $newState")

            if (newState == BluetoothProfile.STATE_CONNECTED) {
                deviceArrayList.add(device)
            } else {
                deviceArrayList.removeAll { it.address == device.address }
            }

            blePeripheralCallback?.onConnectionStateChange(device, status, newState)
            super.onConnectionStateChange(device, status, newState)
        }

        override fun onServiceAdded(status: Int, service: BluetoothGattService) {
            super.onServiceAdded(status, service)
            Log.e(TAG, "onServiceAdded：status = $status")
        }

        override fun onCharacteristicReadRequest(
            device: BluetoothDevice,
            requestId: Int,
            offset: Int,
            characteristic: BluetoothGattCharacteristic
        ) {
            Log.e(TAG, "onCharacteristicReadRequest：device name = ${device.name}, address = ${device.address}")
            Log.e(TAG, "onCharacteristicReadRequest：requestId = $requestId, offset = $offset")
            bluetoothGattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, characteristic.value)
        }

        override fun onCharacteristicWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            characteristic: BluetoothGattCharacteristic,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            requestBytes: ByteArray
        ) {
            Log.e(TAG, "3.onCharacteristicWriteRequest：device name = ${device.name}, address = ${device.address}")
            Log.e(TAG, "3.onCharacteristicWriteRequest：requestId = $requestId, preparedWrite=$preparedWrite, responseNeeded=$responseNeeded, offset=$offset, value=${String(requestBytes)}")
            bluetoothGattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, characteristic.value)
            blePeripheralCallback?.onCharacteristicWriteRequest(device, requestId, characteristic, preparedWrite, responseNeeded, offset, requestBytes)
        }

        override fun onDescriptorWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            descriptor: BluetoothGattDescriptor,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            value: ByteArray
        ) {
            Log.e(TAG, "2.onDescriptorWriteRequest：device name = ${device.name}, address = ${device.address}")
            Log.e(TAG, "2.onDescriptorWriteRequest：requestId = $requestId, preparedWrite = $preparedWrite, responseNeeded = $responseNeeded, offset = $offset, value = ${String(value)}")
            bluetoothGattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, value)
        }

        override fun onDescriptorReadRequest(
            device: BluetoothDevice,
            requestId: Int,
            offset: Int,
            descriptor: BluetoothGattDescriptor
        ) {
            Log.e(TAG, "onDescriptorReadRequest：device name = ${device.name}, address = ${device.address}")
            Log.e(TAG, "onDescriptorReadRequest：requestId = $requestId")
            bluetoothGattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, null)
        }

        override fun onNotificationSent(device: BluetoothDevice, status: Int) {
            super.onNotificationSent(device, status)
            Log.e(TAG, "5.onNotificationSent：device name = ${device.name}, address = ${device.address}")
            Log.e(TAG, "5.onNotificationSent：status = $status")
        }

        override fun onMtuChanged(device: BluetoothDevice, mtu: Int) {
            super.onMtuChanged(device, mtu)
            Log.e(TAG, "onMtuChanged：mtu = $mtu")
        }

        override fun onExecuteWrite(device: BluetoothDevice, requestId: Int, execute: Boolean) {
            super.onExecuteWrite(device, requestId, execute)
            Log.e(TAG, "onExecuteWrite：requestId = $requestId")
        }
    }

    fun init() {
        bluetoothManager = context.getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        mBluetoothAdapter = bluetoothManager?.adapter
        if (mBluetoothAdapter == null || !mBluetoothAdapter!!.isEnabled) {
            toEnable()
        }
    }

    private fun toEnable(): Boolean {
        mBluetoothAdapter?.let {
            try {
                it.javaClass.getMethod("enableNoAutoConnect").invoke(it)
                return true
            } catch (e: Exception) {
                return it.enable()
            }
        }
        return false
    }

    fun startBluetoothLeAdvertiser(bleName: String, serviceData: ByteArray, parcelUUID: UUID) {
        val settings = AdvertiseSettings.Builder()
            .setConnectable(true)
            .setTimeout(0)
            .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_BALANCED)
            .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_HIGH)
            .build()

        val advertiseData = AdvertiseData.Builder()
            .setIncludeDeviceName(true)
            .setIncludeTxPowerLevel(true)
            .build()

        val scanResponseData = AdvertiseData.Builder()
            .setIncludeTxPowerLevel(true)
            .addServiceData(ParcelUuid(parcelUUID), serviceData)
            .build()

        mBluetoothAdapter?.name = bleName
        bluetoothLeAdvertiser = mBluetoothAdapter?.bluetoothLeAdvertiser
        bluetoothLeAdvertiser?.startAdvertising(settings, advertiseData, scanResponseData, callback)
    }

    fun stopBluetoothLeAdvertiser() {
        bluetoothLeAdvertiser?.stopAdvertising(callback)
    }

    fun addServices(vararg serviceInfo: BluetoothGattServiceInfo) {
        bluetoothGattServer = bluetoothManager?.openGattServer(context, bluetoothGattServerCallback)
        serviceInfo.forEach { info ->
            val service = BluetoothGattService(info.uuid, info.serviceType)
            info.characteristicInfos.forEach { charInfo ->
                val characteristic = BluetoothGattCharacteristic(
                    charInfo.uuid,
                    charInfo.properties,
                    charInfo.permissions
                )
                charInfo.descriptorInfo?.let { descInfo ->
                    characteristic.addDescriptor(
                        BluetoothGattDescriptor(descInfo.uuid, descInfo.permissions)
                    )
                }
                service.addCharacteristic(characteristic)
            }
            // bluetoothGattServer?.addService(service)
            val success = bluetoothGattServer?.addService(service) ?: false
            Log.i("add service", "$success")
        }
    }

    fun notifyDevice(
        device: BluetoothDevice?,
        characteristic: BluetoothGattCharacteristic?,
        data: ByteArray?
    ): Boolean {
        return if (device != null && characteristic != null && data != null) {
            characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT
            characteristic.value = data
            bluetoothGattServer?.notifyCharacteristicChanged(device, characteristic, false) ?: false
        } else {
            false
        }
    }

    fun getCharacteristicList(serviceUuid: UUID): List<BluetoothGattCharacteristic>? {
        return bluetoothGattServer?.getService(serviceUuid)?.characteristics
    }

    fun getCharacteristic(serviceUuid: UUID, characteristicUuid: UUID): BluetoothGattCharacteristic? {
        return bluetoothGattServer?.getService(serviceUuid)?.getCharacteristic(characteristicUuid)
    }

    interface BlePeripheralCallback {
        fun onConnectionStateChange(device: BluetoothDevice, status: Int, newState: Int)
        fun onCharacteristicWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            characteristic: BluetoothGattCharacteristic,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            value: ByteArray
        )
    }

    // Helper data classes (assuming these are defined in original Java code)
    data class BluetoothGattServiceInfo(
        val uuid: UUID,
        val serviceType: Int,
        val characteristicInfos: List<BluetoothGattCharacteristicInfo>
    )

    data class BluetoothGattCharacteristicInfo(
        val uuid: UUID,
        val properties: Int,
        val permissions: Int,
        val descriptorInfo: BluetoothGattDescriptorInfo?
    )

    data class BluetoothGattDescriptorInfo(
        val uuid: UUID,
        val permissions: Int
    )
}