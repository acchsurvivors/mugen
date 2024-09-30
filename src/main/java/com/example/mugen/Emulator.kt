import android.os.Build
import java.io.File

data class EmulatorChecksResult(
    val isEmulatorFiles: Boolean,
    val isemulatorTest: Boolean,
    val isCustomTest: Boolean,
    val isRunningOnEmulator: Boolean
) {
    fun isAnyCheckTrue() = isEmulatorFiles || isemulatorTest || isCustomTest || isRunningOnEmulator
}

fun performEmulatorChecks(): EmulatorChecksResult {
    val isEmulatorFiles = checkEmulatorFiles()
    val isemulatorTest = emulatorTest()
    val isCustomTest = isCustomRomTestKeyBuild()
    val isRunningOnEmulator = checkRunningOnEmulator()

    return EmulatorChecksResult(
        isEmulatorFiles,
        isemulatorTest,
        isCustomTest,
        isRunningOnEmulator
    )
}

private val GENY_FILES = arrayOf(
    "/dev/socket/genyd",
    "/dev/socket/baseband_genyd"
)
private val PIPES = arrayOf(
    "/dev/socket/qemud",
    "/dev/qemu_pipe"
)
private val X86_FILES = arrayOf(
    "ueventd.android_x86.rc",
    "x86.prop",
    "ueventd.ttVM_x86.rc",
    "init.ttVM_x86.rc",
    "fstab.ttVM_x86",
    "fstab.vbox86",
    "init.vbox86.rc",
    "ueventd.vbox86.rc"
)
val ANDY_FILES = arrayOf(
    "fstab.andy",
    "ueventd.andy.rc"
)
private val NOX_FILES = arrayOf(
    "fstab.nox",
    "init.nox.rc",
    "ueventd.nox.rc"
)

fun checkFiles(targets: Array<String>): Boolean {
    for (pipe in targets) {
        val file = File(pipe)
        if (file.exists()) {
            return true
        }
    }
    return false
}

fun checkEmulatorFiles(): Boolean {
    return (checkFiles(GENY_FILES)
            || checkFiles(ANDY_FILES)
            || checkFiles(NOX_FILES)
            || checkFiles(X86_FILES)
            || checkFiles(PIPES))
}

fun emulatorTest(): Boolean {
    return (Build.BRAND.startsWith("generic") && Build.DEVICE.startsWith("generic"))
            || Build.FINGERPRINT.startsWith("generic")
            || Build.FINGERPRINT.startsWith("unknown")
            || Build.HARDWARE.contains("goldfish")
            || Build.HARDWARE.contains("ranchu")
            || Build.MODEL.contains("google_sdk")
            || Build.MODEL.contains("Emulator")
            || Build.MODEL.contains("Android SDK built for x86")
            || Build.MANUFACTURER.contains("Genymotion")
            || Build.PRODUCT.contains("sdk_google")
            || Build.PRODUCT.contains("google_sdk")
            || Build.PRODUCT.contains("sdk")
            || Build.PRODUCT.contains("sdk_x86")
            || Build.PRODUCT.contains("sdk_gphone64_arm64")
            || Build.PRODUCT.contains("vbox86p")
            || Build.PRODUCT.contains("emulator")
            || Build.PRODUCT.contains("simulator")
            || checkEmulatorFiles()
}

fun isCustomRomTestKeyBuild(): Boolean {
    val str = Build.TAGS
    return str != null && str.contains("test-keys")
}

fun checkRunningOnEmulator(): Boolean {
    return ((Build.FINGERPRINT.startsWith("google/sdk_gphone_")
            && Build.FINGERPRINT.endsWith(":user/release-keys")
            && Build.MANUFACTURER == "Google" && Build.PRODUCT.startsWith("sdk_gphone_") && Build.BRAND == "google"
            && Build.MODEL.startsWith("sdk_gphone_"))
            || Build.HARDWARE.contains("goldfish")
            || Build.HARDWARE.contains("ranchu")
            || Build.PRODUCT.contains("sdk_gphone64_arm64")
            || Build.PRODUCT.contains("vbox86p")
            || Build.PRODUCT.contains("emulator")
            || Build.PRODUCT.contains("simulator")
            || Build.FINGERPRINT.startsWith("generic")
            || Build.FINGERPRINT.startsWith("unknown")
            || Build.PRODUCT.contains("sdk_google")
            || Build.MODEL.contains("google_sdk")
            || Build.PRODUCT.contains("sdk_x86")
            || Build.MODEL.contains("Emulator")
            || Build.MODEL.contains("Android SDK built for x86")
            || "QC_Reference_Phone" == Build.BOARD && !"Xiaomi".equals(
        Build.MANUFACTURER,
        ignoreCase = true
    )
            || Build.MANUFACTURER.contains("Genymotion")
            || Build.HOST.startsWith("Build")
            || Build.BRAND.startsWith("generic") && Build.DEVICE.startsWith("generic")
            || Build.PRODUCT == "google_sdk")
}
