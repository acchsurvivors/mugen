import android.os.Environment
import java.io.File
import java.nio.charset.StandardCharsets
import java.util.regex.Pattern

object BurpDetector {

    @JvmStatic
    fun burpCheck(): Boolean {
        val sdcard = Environment.getExternalStorageDirectory()
        return searchForCerFile(sdcard)
    }

    private fun searchForCerFile(directory: File): Boolean {
        if (!directory.isDirectory) {
            return false
        }

        val files = directory.listFiles()
        if (files != null) {
            for (file in files) {
                if (file.isDirectory) {
                    if (searchForCerFile(file)) {
                        return true
                    }
                } else if (file.name.endsWith(".cer")) {
                    if (checkFileContents(file)) {
                        return true
                    }
                }
            }
        }
        return false
    }

    private fun checkFileContents(file: File): Boolean {
        val pattern = Pattern.compile("(?i)\\bPortSwigger\\b")

        try {
            val contents = file.readText(StandardCharsets.UTF_8)
            if (pattern.matcher(contents).find()) {
                return true
            }
        } catch (e: Exception) {
        }

        return false
    }
}
