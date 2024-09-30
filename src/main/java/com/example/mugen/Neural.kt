package com.example.mugen

import android.content.Context
import org.tensorflow.lite.Interpreter
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.MappedByteBuffer
import java.nio.channels.FileChannel
import java.io.FileInputStream
import android.os.Process
import java.io.BufferedReader
import java.io.File
import java.io.FileReader
import java.io.IOException
import java.text.SimpleDateFormat
import java.util.*



fun hexToDec(hexStr: String): Int {
    return try {
        hexStr.toInt(16)
    } catch (e: NumberFormatException) {
        0
    }
}

// Função para dividir o process_id no formato '1000-2000'
fun splitProcessId(processIdStr: String?): Pair<Int, Int> {
    return if (processIdStr != null && processIdStr.contains("-")) {
        val parts = processIdStr.split("-")
        val start = parts[0].toIntOrNull() ?: 0
        val end = parts[1].toIntOrNull() ?: 0
        Pair(start, end)
    } else {
        Pair(0, 0)
    }
}

// Função para realizar one-hot encoding de valores categóricos dinamicamente
fun oneHotEncodeDynamic(value: String, categories: MutableMap<String, Int>): FloatArray {
    if (!categories.containsKey(value)) {
        categories[value] = categories.size // Adiciona nova categoria com um índice único
    }
    val encoding = FloatArray(categories.size) { 0f }  // Cria array com tamanho igual ao número total de categorias
    val index = categories[value] ?: 0  // Recupera o índice da categoria
    encoding[index] = 1f  // Define o valor da categoria como 1 no índice correto
    return encoding
}

// Função de normalização usando MinMaxScaler
fun normalize(value: Float, min: Float, max: Float): Float {
    return (value - min) / (max - min)
}

// Defina os valores mínimos e máximos usados durante o treinamento
val minMemoryRegionStart = 0f
val maxMemoryRegionStart = 1234567f
val minMemoryRegionEnd = 0f
val maxMemoryRegionEnd = 1234567f
val minProcessIdStart = 0f
val maxProcessIdStart = 10000f
val minProcessIdEnd = 0f
val maxProcessIdEnd = 20000f

// Mapa global de categorias dinâmicas
val dynamicCategoryMaps = mutableMapOf<String, MutableMap<String, Int>>()

fun preprocessData(newData: Map<String, String>, expectedSize: Int): FloatArray {
    val processedData = mutableListOf<Float>()

    newData.forEach { (key, value) ->
        when (key) {
            "memory_region_start" -> {
                val normalizedValue = normalize(hexToDec(value).toFloat(), minMemoryRegionStart, maxMemoryRegionStart)
                processedData.add(normalizedValue)
            }
            "memory_region_end" -> {
                val normalizedValue = normalize(hexToDec(value).toFloat(), minMemoryRegionEnd, maxMemoryRegionEnd)
                processedData.add(normalizedValue)
            }
            "process_id" -> {
                val (processIdStart, processIdEnd) = splitProcessId(value)
                processedData.add(normalize(processIdStart.toFloat(), minProcessIdStart, maxProcessIdStart))
                processedData.add(normalize(processIdEnd.toFloat(), minProcessIdEnd, maxProcessIdEnd))
            }
            else -> {
                val categoryMap = dynamicCategoryMaps.getOrPut(key) { mutableMapOf() }
                val encoding = oneHotEncodeDynamic(value, categoryMap)
                processedData.addAll(encoding.toList())
            }
        }
    }

    // Ensure the array is exactly the expected size
    while (processedData.size < expectedSize) {
        processedData.add(0f) // Fill with zeros
    }

    // Limit the size to the expected size
    return processedData.take(expectedSize).toFloatArray()
}


// Função auxiliar para redimensionar arrays
fun FloatArray.resize(newSize: Int): FloatArray {
    return this.copyOf(newSize)
}

// Classe TFLiteModel para carregar e executar inferências no modelo TFLite
class TFLiteModel(context: Context) {
    private var interpreter: Interpreter

    init {
        interpreter = Interpreter(loadModelFile(context))
    }

    private fun loadModelFile(context: Context): MappedByteBuffer {
        val fileDescriptor = context.assets.openFd("Mahoraga.tflite")
        val inputStream = FileInputStream(fileDescriptor.fileDescriptor)
        val fileChannel = inputStream.channel
        val startOffset = fileDescriptor.startOffset
        val declaredLength = fileDescriptor.declaredLength
        return fileChannel.map(FileChannel.MapMode.READ_ONLY, startOffset, declaredLength)
    }

    fun predictData(inputData: FloatArray): FloatArray {
        val inputBuffer = ByteBuffer.allocateDirect(inputData.size * 4).order(ByteOrder.nativeOrder())
        inputBuffer.asFloatBuffer().put(inputData)

        val outputBuffer = ByteBuffer.allocateDirect(inputData.size * 4).order(ByteOrder.nativeOrder())
        val outputFloatBuffer = outputBuffer.asFloatBuffer()

        interpreter.run(inputBuffer, outputBuffer)

        val outputArray = FloatArray(inputData.size)
        outputFloatBuffer.get(outputArray)

        return outputArray
    }

    fun calculateReconstructionError(reconstructed: FloatArray, originalInput: FloatArray): Double {
        var error = 0.0
        for (i in originalInput.indices) {
            error += (originalInput[i] - reconstructed[i]).let { it * it }
        }
        return error / originalInput.size
    }

    fun isSuspicious(reconstructionError: Double, threshold: Double): Boolean {
        return reconstructionError > threshold
    }
}

// Função para capturar e processar as informações de memória e rodar o modelo
class MemoryInfoPrinter {
    private val MAX_LINES = 250

    fun captureMemoryInfo(context: Context): List<Map<String, String>> {
        val formattedMemoryInfo: MutableList<Map<String, String>> = mutableListOf()
        try {
            val maps = readProcMaps()
            formattedMemoryInfo.addAll(formatMemoryInfo(maps))
        } catch (e: Exception) {
            println("Error during memory info capture: ${e.message}")
        }
        return formattedMemoryInfo
    }

    @Throws(IOException::class)
    private fun readProcMaps(): List<String> {
        val file = File("/proc/${Process.myPid()}/maps")
        val lines: MutableList<String> = mutableListOf()
        BufferedReader(FileReader(file)).use { reader ->
            var line: String?
            while (reader.readLine().also { line = it } != null && lines.size < MAX_LINES) {
                lines.add(line!!)
            }
        }
        return lines
    }

    private fun formatMemoryInfo(maps: List<String>): List<Map<String, String>> {
        val formattedLines: MutableList<Map<String, String>> = mutableListOf()
        val processId = Process.myPid().toString()

        for (line in maps) {
            val parts = line.split("\\s+".toRegex())
            if (parts.size >= 6) {
                val memoryRegionStart = "0x" + parts[0].split("-")[0]
                val memoryRegionEnd = "0x" + parts[0].split("-")[1]
                val permissions = parts[1]
                val path = if (parts.size > 5) parts[5] else ""
                val libName = if (path.contains("/")) path.substring(path.lastIndexOf("/") + 1) else "Unknown"
                val deletedFlag = if (libName.contains(Regex("\\(deleted\\)"))) "deleted" else "notdeleted"

                val memoryInfoMap = mapOf(
                    "memory_region_start" to memoryRegionStart,
                    "memory_region_end" to memoryRegionEnd,
                    "process_id" to processId,
                    "permission" to permissions,
                    "Nome_da_lib" to libName,
                    "path_para_lib" to path,
                    "deleted_flag" to deletedFlag
                )
                formattedLines.add(memoryInfoMap)
            }
        }
        return formattedLines
    }
}
