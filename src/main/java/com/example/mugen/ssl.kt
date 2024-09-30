package com.example.mugen

import android.app.Activity
import android.content.Context
import okhttp3.OkHttpClient
import okhttp3.Request
import java.io.IOException
import java.net.UnknownHostException
import java.security.MessageDigest
import java.security.cert.X509Certificate
import javax.net.ssl.SSLContext
import javax.net.ssl.TrustManagerFactory
import javax.net.ssl.X509TrustManager


fun calculateSHA256(cert: X509Certificate): String {
    val md = MessageDigest.getInstance("SHA-256")
    val publicKey = cert.publicKey.encoded
    val hash = md.digest(publicKey)
    return hash.joinToString("") { "%02x".format(it) }
}

fun sslPinningWithHash(context: Context, url: String, encryptedExpectedHash: String): Result<Boolean> {
    return try {
        val trustManagerFactory = TrustManagerFactory.getInstance(TrustManagerFactory.getDefaultAlgorithm())
        trustManagerFactory.init(null as java.security.KeyStore?)
        val trustManagers = trustManagerFactory.trustManagers
        val trustManager = trustManagers[0] as X509TrustManager

        val sslContext = SSLContext.getInstance("TLS")
        sslContext.init(null, arrayOf(trustManager), null)

        val client = OkHttpClient.Builder()
            .sslSocketFactory(sslContext.socketFactory, trustManager)
            .build()

        val request = Request.Builder()
            .url(url)
            .build()

        val response = client.newCall(request).execute()

        val handshake = response.handshake ?: return Result.failure(Exception("Handshake falhou"))
        val peerCertificates = handshake.peerCertificates
        val serverCert = peerCertificates[0] as X509Certificate

        val certHash = calculateSHA256(serverCert)
        println(certHash)

        val comparacaoDeHash: Boolean = Mugen.Mugen.decript(context, encryptedExpectedHash, certHash)
        if (comparacaoDeHash) {
            Result.success(true)
        } else {
            Result.success(false)
        }
    } catch (e: UnknownHostException) {
        Result.success(false)
    } catch (e: IOException) {
        Result.success(false)
    } catch (e: Exception) {
        Result.success(false)
    }
}

