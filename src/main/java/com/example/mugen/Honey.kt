package com.example.mugen

import android.content.Context
import android.os.Build
import android.util.Base64
import java.io.IOException

class Honey {

    fun createBlacklistFile(context: Context) {
        val filename = "blacklist.tmp"
        val fingerprint = Build.FINGERPRINT
        val encodedFingerprint = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            // Para API 26 e superior, usa-se java.util.Base64
            android.util.Base64.encodeToString(fingerprint.toByteArray(), Base64.NO_WRAP)
        } else {
            // Para versões abaixo da API 26, usa-se a Base64 de android.util
            android.util.Base64.encodeToString(fingerprint.toByteArray(), Base64.NO_WRAP)
        }

        try {
            context.openFileOutput(filename, Context.MODE_PRIVATE).use { outputStream ->
                outputStream.write(encodedFingerprint.toByteArray())
            }
        } catch (e: IOException) {
            e.printStackTrace()
        }
    }


    fun isBlacklistFileExists(context: Context): Boolean {
        val filename = "blacklist.tmp"
        val file = context.getFileStreamPath(filename)
        return file.exists()
    }

}