package com.example.mugen

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import performDebugChecks
import performEmulatorChecks
import performHookChecks
import performRootChecks
import java.io.IOException
import java.net.UnknownHostException


private const val PREFS_NAME = "M"
private const val KEY_FUNCTION_CALLED = "Go"
val honey = Honey()
class Mugen {

    object Mugen {
        init {
            System.loadLibrary("mugen")
        }
        @JvmStatic
        external fun performDebugChecks(context: Context): Boolean
        @JvmStatic
        external fun isEmulator(): Boolean
        @JvmStatic
        external fun detect_root(): Boolean
        @JvmStatic
        external fun Checks_repack(context: Context, encryptedHash: String): Boolean
        @JvmStatic
        external fun checkSecurityIssues(context: Context): Boolean
        @JvmStatic
        external fun decript(context: Context,encryptedHash: String,certHash: String): Boolean
        @JvmStatic
        external fun predict(context: Context): Boolean
        }

    enum class SecurityCheck {
        SSL_PINNING,
        ANALYSIS,
        ROOT_DETECTION,
        EMULATOR_DETECTION,
        DEBUG_DETECTION,
        SECURITY_PROBLEMS_DETECTION,
        HOOK_DETECTION,
        REPACK_DETECTION,
        HONEYPOT_CHECK,
        NEURAL
    }

    fun initialize(
        context: Context,
        expectedHash: String,
        domainHashPairs: List<Pair<String, String>>,
        checksToRun: List<SecurityCheck>
    ): Map<SecurityCheck, Boolean> {
        val sharedPreferences: SharedPreferences = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

        if (!sharedPreferences.getBoolean(KEY_FUNCTION_CALLED, false)) {
            honey.createBlacklistFile(context)
            with(sharedPreferences.edit()) {
                putBoolean(KEY_FUNCTION_CALLED, true)
                apply()
            }
        }
        return startSecurityChecks(context, expectedHash, domainHashPairs, checksToRun)
    }

    private fun startSecurityChecks(context: Context, expectedHash: String, domainHashPairs: List<Pair<String, String>>, checksToRun: List<SecurityCheck>): Map<SecurityCheck, Boolean> {
        val results = mutableMapOf<SecurityCheck, Boolean>()

        try {
            for (check in checksToRun) {
                val result = when (check) {
                    SecurityCheck.SSL_PINNING -> sslPinning(context, domainHashPairs)
                    SecurityCheck.ANALYSIS -> analise(context)
                    SecurityCheck.ROOT_DETECTION -> checkRootDetection(context)
                    SecurityCheck.EMULATOR_DETECTION -> checkEmulatorDetection()
                    SecurityCheck.DEBUG_DETECTION -> checkDebugDetection(context)
                    SecurityCheck.SECURITY_PROBLEMS_DETECTION -> checkSecurityProblemsDetection(context)
                    SecurityCheck.HOOK_DETECTION -> checkHookDetection(context)
                    SecurityCheck.REPACK_DETECTION -> checkRepackDetection(context, expectedHash)
                    SecurityCheck.HONEYPOT_CHECK -> honeypotCheck(context)
                    SecurityCheck.NEURAL -> runModelPredictionAndSave(context)
                }
                results[check] = result
            }
        } catch (e: Exception) {
            Log.e("MugenCheck", "Error during security checks: ${e.message}")
        }

        return results
    }

    private fun checkSecurityProblemsDetection(context: Context): Boolean {
        val checks = Mugen.checkSecurityIssues(context)
        val burp = BurpDetector.burpCheck()
        return checks || burp
    }

    private fun checkRootDetection(context: Context): Boolean {
        val isRooted = Mugen.detect_root()
        val rootCheckResult = performRootChecks(context)
        return isRooted || rootCheckResult.isAnyCheckTrue()
    }

    private fun checkDebugDetection(context: Context): Boolean {
        val isDebugOn = Mugen.performDebugChecks(context)
        val debugCheckResult = performDebugChecks(context)
        return isDebugOn || debugCheckResult.isAnyCheckTrue()
    }

    private fun checkEmulatorDetection(): Boolean {
        val isEmulator = Mugen.isEmulator()
        val emulatorCheckResult = performEmulatorChecks()
        return isEmulator || emulatorCheckResult.isAnyCheckTrue()
    }

    private fun checkHookDetection(context: Context): Boolean {
        val hookChecksResult = performHookChecks(context)
        return hookChecksResult.isAnyCheckTrue()
    }

    private fun checkRepackDetection(context: Context, expectedHash: String): Boolean {
        return Mugen.Checks_repack(context, expectedHash)
    }

    private fun honeypotCheck(context: Context): Boolean {
        return !honey.isBlacklistFileExists(context)
    }

    private fun analise(context: Context): Boolean {
        return Mugen.predict(context)
    }

    private fun sslPinning(context: Context, domainHashPairs: List<Pair<String, String>>): Boolean {
        var allCertificatesValid = true

        for ((url, encryptedHash) in domainHashPairs) {
            val result = sslPinningWithHash(context, url, encryptedHash)
            result.fold(
                onSuccess = { isTrusted ->
                    if (!isTrusted) {
                        allCertificatesValid = false
                    }
                },
                onFailure = { error ->
                    allCertificatesValid = false
                    when (error) {
                        is IOException -> {
                            if (error.cause is UnknownHostException) {
                                Log.d("MugenCheck", "Sem conexão com a internet para $url")
                            } else {
                                Log.d("MugenCheck", "Erro de rede para $url: ${error.message}")
                            }
                        }
                        else -> {
                            Log.d("MugenCheck", "Erro inesperado para $url: ${error.message}")
                        }
                    }
                }
            )
            if (!allCertificatesValid) break
        }

        return allCertificatesValid
    }

    fun runModelPredictionAndSave(context: Context): Boolean {
        val printer = MemoryInfoPrinter()
        val memoryData = printer.captureMemoryInfo(context)
        val threshold = 0.0061
        val tfliteModel = TFLiteModel(context)
        val expectedSize = 7336 / 4
        var suspiciousCount = 0

        memoryData.drop(230).take(10).forEachIndexed { index, entry ->
            try {
                val inputData = preprocessData(entry, expectedSize)
                val reconstructed = tfliteModel.predictData(inputData)
                val reconstructionError = tfliteModel.calculateReconstructionError(reconstructed, inputData)
                val isSuspicious = tfliteModel.isSuspicious(reconstructionError, threshold)
                val enrichedEntry = entry.toMutableMap()
                enrichedEntry["reconstruction_error"] = reconstructionError.toString()
                enrichedEntry["is_suspicious"] = isSuspicious.toString()
                // Log.d("ModelPrediction2", "Dado ${index + 1}: Suspeito? $isSuspicious (Erro: $reconstructionError)")
                if (isSuspicious) suspiciousCount++
                /* if (entry.toString().contains("frida", ignoreCase = true)) {
                     Log.w("ModelPrediction2", "Alerta! A palavra 'Frida' foi encontrada no dado ${index + 1}: $entry")
                 }*/
            } catch (e: Exception) {
                Log.e("ModelPrediction2", "Error processing entry $index", e)
            }
        }
        return suspiciousCount >= 3
    }

}