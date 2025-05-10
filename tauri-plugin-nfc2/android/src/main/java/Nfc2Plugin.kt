package com.plugin.nfc2

import android.app.Activity
import android.app.PendingIntent
import android.content.Intent
import android.content.SharedPreferences
import android.content.IntentFilter
import android.nfc.*
import android.nfc.NfcAdapter
import android.nfc.cardemulation.*
import android.nfc.tech.IsoDep
import android.os.Bundle
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.*
import java.io.IOException

@InvokeArg
class WatchArgs {
    lateinit var dataChannel: Channel
    lateinit var errorChannel: Channel
}

@InvokeArg
class HceConfigArgs {
    var aid: String = "F00000000A0101"
    var uuid: String = "12345678-1234-5678-1234-567812345678"
}

@TauriPlugin
class Nfc2Plugin(private val activity: Activity) : Plugin(activity) {
    // 状态变量声明
    private var isHceEnabled: Boolean = false
    private var currentAid: String = "F00000000A0101"
    private var currentUuid: String = "12345678-1234-5678-1234-567812345678"

    private val nfcAdapter: NfcAdapter? = NfcAdapter.getDefaultAdapter(activity)
    private var dataChannel: Channel? = null
    private var errorChannel: Channel? = null
    private lateinit var prefs: SharedPreferences

    private val pendingIntent: PendingIntent by lazy {
        PendingIntent.getActivity(
                activity,
                0,
                Intent(activity, activity.javaClass).addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP),
                PendingIntent.FLAG_MUTABLE
        )
    }

    @Command
    fun init(invoke: Invoke) {
        val args = invoke.parseArgs(WatchArgs::class.java)
        dataChannel = args.dataChannel
        errorChannel = args.errorChannel
        prefs = activity.getSharedPreferences("nfc_plugin", Activity.MODE_PRIVATE)
        loadHceConfig()
        checkNfcStatus()
    }

    @Command
    fun startHce(invoke: Invoke) {
        val args = invoke.parseArgs(HceConfigArgs::class.java)
        currentAid = args.aid
        currentUuid = args.uuid
        isHceEnabled = true
        saveHceConfig()
        invoke.resolve()
    }

    @Command
    fun stopHce(invoke: Invoke) {
        isHceEnabled = false
        invoke.resolve()
    }

    private fun loadHceConfig() {
        currentAid = prefs.getString("aid", "F00000000A0101") ?: "F00000000A0101"
        currentUuid = prefs.getString("uuid", "") ?: ""
    }

    private fun saveHceConfig() {
        prefs.edit().putString("aid", currentAid).putString("uuid", currentUuid).apply()
    }

    private fun checkNfcStatus() {
        when {
            nfcAdapter == null -> sendError("NFC_NOT_SUPPORTED", "设备不支持 NFC")
            !nfcAdapter.isEnabled -> sendError("NFC_DISABLED", "请先启用 NFC")
            else -> enableForegroundDispatch()
        }
    }

    private fun enableForegroundDispatch() {
        try {
            val intentFilter = IntentFilter(NfcAdapter.ACTION_TECH_DISCOVERED).apply {
                addCategory(Intent.CATEGORY_DEFAULT)
            }
            val filters = arrayOf(intentFilter)
            val techList = arrayOf(arrayOf(IsoDep::class.java.name)) // 仅处理 ISO-DEP 标签

            nfcAdapter?.enableForegroundDispatch(
                activity,
                pendingIntent,
                filters, 
                techList
            )
        } catch (e: SecurityException) {
            sendError("SECURITY_ERROR", "NFC权限被拒绝: ${e.message}")
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        handleNfcIntent(intent)
    }

    private fun handleNfcIntent(intent: Intent?) {
        intent?.let {
            processTag(intent.getParcelableExtra(NfcAdapter.EXTRA_TAG)!!)
        }
    }

    private fun processTag(tag: Tag) {
        IsoDep.get(tag)?.use { isoDep ->
            try {
                Log.i("handle intent", "start")
                isoDep.connect()
                Log.i("handle intent", "connected")
                val selectApdu =
                        "00A40400${String.format("%02X", currentAid.length / 2)}$currentAid".hexToBytes()
                val selectResponse = isoDep.transceive(selectApdu)
                Log.i("handle intent", "select aid")
                if (!isSuccess(selectResponse)) return@use

                val uuidResponse = isoDep.transceive("00CA0000".hexToBytes())
                if (!isSuccess(uuidResponse)) return@use

                val uuid =
                        uuidResponse
                                .copyOfRange(0, uuidResponse.size - 2)
                                .toHexString()
                                .insertHyphens()
                sendData(uuid)
            } catch (e: IOException) {
                sendError("IO_ERROR", "通信失败: ${e.message}")
            }
        }
                ?: sendError("TAG_ERROR", "不支持的标签类型")
    }

    private fun isSuccess(response: ByteArray): Boolean {
        Log.i("nfc response", response.toHexString())
        return response.size >= 2 &&
                response[response.size - 2].toInt() == 0x90 &&
                response[response.size - 1].toInt() == 0x00
    }

    private fun sendData(data: String) {
        dataChannel?.send(JSObject().apply { put("value", data) })
    }

    private fun sendError(code: String, message: String) {
        errorChannel?.send(
                JSObject().apply {
                    put("code", code)
                    put("data", message)
                }
        )
    }
}

class HceService : HostApduService() {
    private lateinit var prefs: SharedPreferences

    override fun onCreate() {
        super.onCreate()
        prefs = getSharedPreferences("nfc_plugin", MODE_PRIVATE)
    }

    override fun processCommandApdu(commandApdu: ByteArray?, extras: Bundle?): ByteArray {
        if (commandApdu == null) return "6F00".hexToBytes()
        return handleValidCommand(commandApdu)
    }

    private fun handleValidCommand(apdu: ByteArray): ByteArray {
        return when (apdu[1].toInt()) {
            0xA4 -> {
                if (isSelectAidCommand(apdu)) {
                    "9000".hexToBytes()
                } else {
                    "6F00".hexToBytes()
                }
            }
            0xCA -> {
                if (apdu.size >= 4 && apdu[2] == 0x00.toByte() && apdu[3] == 0x00.toByte()) {
                    (prefs.getString("uuid", "")!!.replace("-", "").hexToBytes() +
                            "9000".hexToBytes())
                } else {
                    "6A86".hexToBytes()
                }
            }
            else -> "6D00".hexToBytes()
        }
    }

    private fun isSelectAidCommand(apdu: ByteArray): Boolean {
        return apdu.size >= 5 &&
                apdu[0] == 0x00.toByte() &&
                apdu[1] == 0xA4.toByte() &&
                apdu[2] == 0x04.toByte() &&
                apdu[3] == 0x00.toByte()
    }

    override fun onDeactivated(reason: Int) {}
}

private fun String.hexToBytes() = chunked(2).map { it.toInt(16).toByte() }.toByteArray()

private fun ByteArray.toHexString() = joinToString("") { "%02X".format(it) }

private fun String.insertHyphens() =
        replace(Regex("(.{8})(.{4})(.{4})(.{4})(.{12})"), "$1-$2-$3-$4-$5")
