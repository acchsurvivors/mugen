import android.content.Context
import android.content.pm.PackageManager
import java.io.File
import java.io.IOException

data class RootChecksResult(
    val isRootFile: Boolean,
    val isRootWrite: Boolean,
    val isRootPath: Boolean,
    val isRootExec: Boolean
) {
    fun isAnyCheckTrue() = isRootFile || isRootWrite || isRootPath || isRootExec
}

fun performRootChecks(context: Context): RootChecksResult {
    val isRootFile = isDeviceRooted_Check_Files(context)
    val isRootWrite = isDirectoryWritable("/system")
    val isRootPath = checkRootPath()
    val isRootExec = isDeviceRooted_Trie_Exec()

    return RootChecksResult(
        isRootFile,
        isRootWrite,
        isRootPath,
        isRootExec
    )
}

fun isDeviceRooted_Check_Files(context: Context): Boolean {
    val suFiles = arrayOf(
        "/sbin/su", "/system/bin/su", "/system/xbin/su", "/data/local/xbin/su",
        "/data/local/bin/su", "/system/sd/xbin/su", "/system/bin/failsafe/su",
        "/data/local/su", "/su/bin/su", "/system/bin/.ext/su", "/system/usr/we-need-root/su",
        "/cache/su", "/dev/su", "/data/su", "/su",
        "/system/app/Superuser.apk", "/system/app/SuperSU.apk", "/system/app/SuperSU",
        "/system/app/SuperSU/SuperSU.apk", "/system/app/Kinguser.apk", "/system/app/KingUser.apk",
        "/system/lib/libsu.so", "/system/lib64/libsu.so",
        "/data/data/com.noshufou.android.su/", "/data/data/eu.chainfire.supersu/",
        "/system/xbin/daemonsu", "/system/xbin/busybox",
        "/data/media/0/TWRP", "/sdcard/TWRP", "/data/TWRP",
        "./frida-server", "/data/local/tmp/frida-server",
        "/system/framework/root-access.jar", "/system/su.d",
        "/system/xbin/ku.sud", "/system/xbin/daemonsu", "/system/xbin/supolicy",
        "/system/xbin/supolicy.so", "/system/xbin/resize2fs_static",
        "/system/xbin/sush", "/system/xbin/busybox", "/system/xbin/busybox_mksh",
        "/system/xbin/busybox_insmod", "/system/xbin/busybox_rmmod", "/system/xbin/toybox",
        "/data/local/tmp/su", "/data/local/tmp/supolicy", "/data/local/tmp/busybox", "/data/local/tmp/magisk",
        "/data/local/tmp/frida-server", "/data/local/tmp/frida64", "/data/local/tmp/magiskhide",
        "/data/local/tmp/magiskcore","/data/adb/ksu","/system/lib/libc_malloc_debug_qemu.so",
        "/system/bin/qemud","/sys/qemu_trace","/system/bin/androVM-prop",
        "/system/bin/microvirt-prop","/dev/vboxguest","/dev/vboxuser",
        "/mnt/prebundledapps/","/system/bluestacks.prop","/system/bin/qemu-props",
        "/sys/devices/virtual/misc/qemu_pipe",
        )

    val filesExist = suFiles.any { File(it).exists() }

    val packageManager = context.packageManager
    val superuserPackages = arrayOf(
        "com.thirdparty.superuser", "eu.chainfire.supersu", "com.noshufou.android.su",
        "com.koushikdutta.superuser", "com.zachspong.temprootremovejb", "com.ramdroid.appquarantine",
        "com.topjohnwu.magisk", "com.devadvance.rootcloak", "com.devadvance.rootcloakplus",
        "de.robv.android.xposed.installer", "com.saurik.substrate", "com.amphoras.hidemyroot",
        "com.amphoras.hidemyrootadfree", "com.formyhm.hiderootPremium", "com.formyhm.hideroot",
        "com.noshufou.android.su.elite", "com.kingo.roo", "com.zhiqupk.root.global",
        "com.smedialink.oneclickroot", "com.alephzain.framaroo", "com.yellowes.su",
        "com.kingroot.kinguser", "stericson.busybox", "com.koushikdutta.rommanager",
        "com.koushikdutta.rommanager.license", "com.dimonvideo.luckypatcher",
        "com.chelpus.lackypatch", "com.ramdroid.appquarantinepro", "com.xmodgame",
        "com.cih.game_cih", "com.charles.lpoqasert", "catch_.me_.if_.you_.can_",
        "org.blackmart.market", "com.allinone.free", "com.repodroid.app",
        "org.creeplays.hack", "com.baseappfull.fwd", "com.zmapp", "com.dv.marketmod.installer",
        "org.mobilism.android", "com.android.wp.net.log", "com.android.camera.update",
        "cc.madkite.freedom", "com.solohsu.android.edxp.manager", "org.meowcat.edxposed.manager",
        "com.android.vending.billing.InAppBillingService.COIN",
        "com.android.vending.billing.InAppBillingService.LUCK", "com.chelpus.luckypatcher",
        "com.blackmartalpha", "com.topjohnwu.magisk", "com.catchingnow.icebox", "com.rootuninstaller.freezer",
        "com.hecorat.freezer", "com.genymotion.superuser", "com.genymotion.superuser",
        "com.genymotion.superuser", "de.robv.android.xposed.installer", "com.bluestacks", "com.bignox.app",
        "com.vphone.launcher", "com.android.emulator", "com.google.android.launcher.layouts.genymotion", "com.bluestacks.home",
    )

    val isSuperuserInstalled = superuserPackages.any { packageName ->
        try {
            packageManager.getPackageInfo(packageName, 0)
            true
        } catch (e: PackageManager.NameNotFoundException) {
            false
        }
    }

    return filesExist || isSuperuserInstalled
}

fun isDirectoryWritable(directoryPath: String): Boolean {
    val directory = File(directoryPath)
    if (!directory.exists() || !directory.isDirectory) {
        return false
    }

    val testFile = File(directory, "test.tmp")
    return try {
        testFile.createNewFile()
        testFile.delete()
        true
    } catch (e: IOException) {
        false
    }
}

fun checkRootPath(): Boolean {
    val pathDirs = System.getenv("PATH")?.split(":") ?: return false
    for (pathDir in pathDirs) {
        val suFile = File(pathDir, "su")
        if (suFile.exists()) {
            return true
        }
    }
    return false
}

fun isDeviceRooted_Trie_Exec(): Boolean {
    var process: Process? = null
    return try {
        process = Runtime.getRuntime().exec("su")
        true
    } catch (e: Exception) {
        false
    } finally {
        process?.destroy()
    }
}
