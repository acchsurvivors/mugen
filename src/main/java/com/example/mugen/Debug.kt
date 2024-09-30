import android.content.Context
import android.content.pm.ApplicationInfo
import android.os.Debug
import kotlin.math.sqrt

data class DebugChecksResult(
    val isDebuggable: Boolean,
    val isDebuggerConnected: Boolean,
    val isCpuTimeExceeded: Boolean
){
    fun isAnyCheckTrue() = isDebuggable || isDebuggerConnected //|| isCpuTimeExceeded
}

fun performDebugChecks(context: Context): DebugChecksResult {
    val isDebuggable = debugCheck(context)
    val isDebuggerConnected = detectDebugger()
    val isCpuTimeExceeded = detectThreadCpuTimeNanos()

    return DebugChecksResult(
        isDebuggable,
        isDebuggerConnected,
        isCpuTimeExceeded
    )
}

// função para verificar se o app está sendo executado em modo de depuração verificando se a flag FLAG_DEBUGGABLE é diferente de 0
fun debugCheck(context: Context): Boolean {
    val applicationInfo = context.applicationInfo
    return applicationInfo.flags and ApplicationInfo.FLAG_DEBUGGABLE != 0
}

// verifica se está conectado no debug
fun detectDebugger(): Boolean {
    return Debug.isDebuggerConnected()
}

// verifica o tempo de execução porque o debug deixa mais devagar
fun detectThreadCpuTimeNanos(): Boolean {
    val start = Debug.threadCpuTimeNanos()

    for (i in 0 until 1_000_000) {
        // Loop simples para consumir tempo de CPU
        sqrt(i.toDouble())
    }

    val stop = Debug.threadCpuTimeNanos()

    return (stop - start) >= 10_000_000 // 10 ms
}

