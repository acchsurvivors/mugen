# Mugen

An open-source RASP (Runtime Application Self Protection) for Android

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Implementation](#implementation)
- [License](#license)

## Introduction

***Mugen*** is an Android RASP (Runtime Application Self Protection) created and built as a protection layer for Android Mobile Applications.

> [!NOTE]
> Mugen is still in development and more changes might be introduced in future releases.

## Features

***Mugen*** has the following protections:

- Root Detection
- Debug Detection
- SSL Pinning
- Emulator Detection
- Hook Detection (Machine Learning)
- Anti Repack
- Malicious Packages Detection

> [!NOTE]
> While this library will protect the application from runtime threats, it is important to know that
> no security measure can ever guarantee absolute security. The protections implemented can be bypassed 
> by a motivated and skilled attacker.

## Installation

### Building the Library from Source

If you wish to build ***Mugen*** from source, the provided Python script will download dependencies and build the libraries  make sure to execute the following commands.

```
git clone https://github.com/acchsurvivors/mugen.git
cd mugen
pip install -r requirements.txt
python3 install.py
```

### Project Import

To import ***Mugen*** into your project in Android Studio, you must go to *"File > New > Import Project..."* and select the cloned repository.

### Adding Dependencies

1. Ensure that you have defined mavenCentral in your Gradle configuration.

```gradle
repositories {
    mavenCentral()
}
```

2. Add the Kotlin plugin as a dependency.

```gradle
classpath "org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.24"
```

3. Add the library directory as in the **sourceSets**, inside the **externalNativeBuild** bracket.

```gradle
sourceSets {
    main {
        jniLibs.srcDirs = ['Mugen/src/main/jniLibs']
    }
} 
```

4. Include ***Mugen*** in the **settings.gradle** file

```gradle
include ':Mugen'
```

5. Include ***Mugen*** in the **build.gradle** file

```gredle
implementation project(':Mugen')
```

### ProGuard Rules

The following rules must be included in the **proguard-rules.pro** file in your application project to avoid building error.

```
-dontwarn javax.servlet.ServletContainerInitializer
-dontwarn org.bouncycastle.jsse.BCSSLParameters
-dontwarn com.example.mugen.Mugen$SecurityCheck
-dontwarn org.bouncycastle.jsse.BCSSLSocket
-dontwarn org.bouncycastle.jsse.provider.BouncyCastleJsseProvider
-dontwarn org.conscrypt.Conscrypt$Version
-dontwarn org.conscrypt.Conscrypt
-dontwarn org.conscrypt.ConscryptHostnameVerifier
-dontwarn org.openjsse.javax.net.ssl.SSLParameters
-dontwarn org.openjsse.javax.net.ssl.SSLSocket
-dontwarn org.openjsse.net.ssl.OpenJSSE

```

## Implementation

To initiate ***Mugen*** and the protections in the application, go to the **MainActivity** class and import the RASP into the code and create the instance inside the **onCreate()** function. This will initiate all the detections from ***Mugen*** in the application.

```java
import com.example.mugen.Mugen;
```
```java
Mugen mugen = new Mugen();
```

### SSL Pinning

With the keys generated during the project building process, use the ***cript*** executables present inside the ***Mugen*** project to generate a secure hash.

To generate the initial and required certificate hash, the following command must be executed (works both on Windows and Linux)

```
echo | openssl s_client -connect URL:443 -servername URL 2>NUL | openssl x509 -pubkey -noout | openssl pkey -pubin -outform der | openssl dgst -sha256
```

#### Encrypting Hashes

***Mugen*** allows you to encrypt the APK Signature and Certificate hash for SSL Pinning protection using ***cript***, which comes in with the project for both Linux and Windows.

```
./cript <key> <iv> <hash>
```

After generating and obtaining the hashes, add the following block of code to set up the SSL Pinning protection.

```java
List<Pair<String, String>> domainHashPairs = Arrays.asList(
    new Pair<>("YOURDOMAINHERE", "YOURDOMAINENCRYPTEDHASHTLS")
);
```

### Repacking Protection

To protect the application from tampering, collect the signature from the APK using the following command and implement the output of the command

```
apksigner verify --print-certs .\base.apk
```

### Initializing Mugen

After completing the steps above, initiate ***Mugen*** with `initialize()`.

```java
List<Mugen.SecurityCheck> checksToRun = Arrays.asList(
                Mugen.SecurityCheck.ROOT_DETECTION,
                Mugen.SecurityCheck.EMULATOR_DETECTION,
                Mugen.SecurityCheck.DEBUG_DETECTION,
                Mugen.SecurityCheck.SSL_PINNING,
                Mugen.SecurityCheck.SECURITY_PROBLEMS_DETECTION,
                Mugen.SecurityCheck.HOOK_DETECTION,
                Mugen.SecurityCheck.REPACK_DETECTION,
                Mugen.SecurityCheck.HONEYPOT_CHECK,
                Mugen.SecurityCheck.ANALYSIS,
                Mugen.SecurityCheck.NEURAL
                );

Map<Mugen.SecurityCheck, Boolean> results = mugen.initialize(this, expectedHash, domainHashPairs, checksToRun);
```

## License

This project is protected under the following [LICENSE](https://github.com/acchsurvivors/mugen/blob/main/LICENSE).
