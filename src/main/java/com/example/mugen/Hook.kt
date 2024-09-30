import android.app.Activity
import android.app.ActivityManager
import android.content.Context
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import java.io.File
import java.net.InetSocketAddress
import java.net.Socket
import java.security.MessageDigest
import java.security.cert.CertificateFactory
import java.security.cert.X509Certificate
import android.util.Base64

// ** funções anti hook **
private const val DUAL_APP_ID_999 = "999"

data class HookChecksResult(
    val isHookFiles: Boolean,
    val isStacktest: Boolean,
    val isProcessFridaTest: Boolean,
    val isFridaListen: Boolean,
    val isFridaListenTest: Pair<Boolean, Int>,
    val isInjectedTest: Boolean,
    val isDual:Boolean
) {
    fun isAnyCheckTrue() = isHookFiles || isStacktest || isProcessFridaTest || isFridaListen || isFridaListenTest.first || isInjectedTest
}

fun performHookChecks(context: Context): HookChecksResult {
    val isHookFiles = CheckForHookingApps(context)
    val isStacktest = inspectStackTraceForHookingApps()
    val isProcessFridaTest = checkRunningProcessesFrida(context)
    val isFridaListenTest = isFridaServerListening()
    val isFridaListenTest2 = detectFridaServer()
    val isInjectedTest = checkCodeInjectionFrida()
    val isDual = checkAppCloning(context)

    return HookChecksResult(
        isHookFiles,
        isStacktest,
        isProcessFridaTest,
        isFridaListenTest,
        isFridaListenTest2,
        isInjectedTest,
        isDual
    )
}

// verifica se o android possiu algumas frameworks de hook conhecidas

fun CheckForHookingApps(context: Context): Boolean {
    val packageManager = context.packageManager
    val applicationInfoList = packageManager.getInstalledApplications(PackageManager.GET_META_DATA)
    var hookFound = false

    for (applicationInfo in applicationInfoList) {
        if (applicationInfo.packageName == "de.robv.android.xposed.installer") {
            hookFound = true
            break // Encerra o loop assim que um dos aplicativos é encontrado
        }
        if (applicationInfo.packageName == "com.saurik.substrate") {
            hookFound = true
            break // Encerra o loop assim que um dos aplicativos é encontrado
        }
    }

    return hookFound
}

// verifica a stack atras de frameworks de hook conhecidas
fun inspectStackTraceForHookingApps(): Boolean {
    var isHooked = false
    try {
        throw Exception("blah")
    } catch (e: Exception) {
        var zygoteInitCallCount = 0
        for (stackTraceElement in e.stackTrace) {
            when {
                stackTraceElement.className == "com.android.internal.os.ZygoteInit" -> {
                    zygoteInitCallCount++
                    if (zygoteInitCallCount == 2) {
                        isHooked = true
                    }
                }
                stackTraceElement.className == "com.saurik.substrate.MS$2" && stackTraceElement.methodName == "invoked" -> {
                    isHooked = true
                }
                stackTraceElement.className == "de.robv.android.xposed.XposedBridge" && stackTraceElement.methodName == "main" -> {
                    isHooked = true
                }
                stackTraceElement.className == "de.robv.android.xposed.XposedBridge" && stackTraceElement.methodName == "handleHookedMethod" -> {
                    isHooked = true
                }
            }
        }
    }
    return isHooked
}

// **detectando o frida**

// verificando a existencia de um fridaserver nos processos
fun checkRunningProcessesFrida(context: Context): Boolean {
    var returnValue = false

    // Get currently running application processes
    val manager = context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager
    val list = manager.getRunningServices(300)

    list?.let {
        for (serviceInfo in it) {
            if (serviceInfo.process.contains("fridaserver")) {
                returnValue = true
            }
        }
    }

    return returnValue
}

// tentando se conectar na porta padrão do frida
fun isFridaServerListening(): Boolean {
    val serverAddress = "127.0.0.1"
    val serverPort = 27042

    return try {
        val socket = Socket()
        socket.connect(InetSocketAddress(serverAddress, serverPort), 1000)
        socket.close()
        true
    } catch (e: Exception) {
        false
    }
}

// enviando uma mensagem AUTH para cada porta aberta e verifique se há uma resposta, esperando que o fridaserver se revele.
fun detectFridaServer(): Pair<Boolean, Int> {
    for (i in 0..65535) {
        try {
            val sock = Socket("localhost", i)
            sock.outputStream.write(0)
            sock.outputStream.write("AUTH\r\n".toByteArray())

            Thread.sleep(100)

            val res = ByteArray(7)
            val ret = sock.inputStream.read(res)

            if (ret != -1 && String(res, 0, 6) == "REJECT") {
                sock.close()
                return true to i
            }

            sock.close()
        } catch (e: Exception) {
            // Handle exception
        }
    }
    return false to -1
}

// lista de bibliotecas carregadas e verifica de suspeitas
fun checkCodeInjectionFrida(): Boolean {
    val filePath = "/proc/self/maps"
    val targetString = "frida"
    var found = false

    try {
        File(filePath).forEachLine { line ->
            if (line.contains(targetString)) {
                found = true
                return@forEachLine
            }
        }
    } catch (e: Exception) {
        // Handle exception
    }
    return found
}

//verifica se o aplicativo está sendo executado em um ambiente de "App Cloning" ou
fun checkAppCloning(context: Context): Boolean {

    val path: String = context.filesDir.path
    val packageName = context.packageName
    val pathDotCount = path.split(".").size-1
    val packageDotCount = packageName.split(".").size-1
    if (path.contains(DUAL_APP_ID_999) || pathDotCount > packageDotCount) {
        return false
    }
    return true
}



