import subprocess
import os
import platform
import shutil
import re
import random
import string
import threading

def verificar_rust_instalado():
    try:
        resultado = subprocess.run(["rustc", "--version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if resultado.returncode == 0:
            return True
        else:
            return False
    except FileNotFoundError:
        return False

def instalar_rust():
    try:
        sistema_operacional = platform.system()

        if sistema_operacional == "Linux" or sistema_operacional == "Darwin":  
            comando = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
        elif sistema_operacional == "Windows":
            comando = "powershell -Command \"& {Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe; Start-Process .\\rustup-init.exe -ArgumentList '/quiet /install' -Wait}\""
        else:
            return f"Sistema operacional {sistema_operacional} não suportado para instalação automática."

        print("Instalando o Rust...")
        resultado = subprocess.run(comando, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

        if resultado.returncode == 0:
            return "Rust foi instalado com sucesso!"
        else:
            return f"Erro ao instalar Rust: {resultado.stderr}"

    except Exception as e:
        return f"Ocorreu um erro: {str(e)}"

def verificar_e_instalar_android_ndk():
    try:
        android_ndk_home = os.environ.get("ANDROID_NDK_HOME")
        
        if android_ndk_home and os.path.exists(android_ndk_home):
            return f"Android NDK já está instalado e configurado em: {android_ndk_home}"
        
        print("Android NDK não encontrado. Iniciando instalação...")

        sistema_operacional = platform.system()
        ndk_folder_name = "android-ndk-r25b"

        if sistema_operacional == "Linux" or sistema_operacional == "Darwin":  
            ndk_url = "https://dl.google.com/android/repository/android-ndk-r25b-linux.zip" if sistema_operacional == "Linux" else "https://dl.google.com/android/repository/android-ndk-r25b-darwin.zip"
            comandos = [
                f"curl -o android-ndk.zip {ndk_url}",  
                "unzip android-ndk.zip",  
                "rm android-ndk.zip"  
            ]
            configurar_variavel = f"echo 'export ANDROID_NDK_HOME=$(pwd)/{ndk_folder_name}' >> ~/.bashrc && echo 'export PATH=$ANDROID_NDK_HOME:$PATH' >> ~/.bashrc && source ~/.bashrc"

        elif sistema_operacional == "Windows":
            ndk_url = "https://dl.google.com/android/repository/android-ndk-r25b-windows.zip"
            comandos = [
                f"powershell -Command \"& {{Invoke-WebRequest -Uri {ndk_url} -OutFile android-ndk.zip}}\"", 
                "powershell -Command \"& {Expand-Archive -Path android-ndk.zip -DestinationPath .}\"",  
                "powershell -Command \"& {Remove-Item android-ndk.zip}\""  
            ]
            configurar_variavel = r'[System.Environment]::SetEnvironmentVariable("ANDROID_NDK_HOME", "$(pwd)\{ndk_folder_name}", "User"); ' \
                                  r'[System.Environment]::SetEnvironmentVariable("Path", "$Env:ANDROID_NDK_HOME;$Env:Path", "User");'

        else:
            return f"Sistema operacional {sistema_operacional} não suportado para instalação automática."

        for comando in comandos:
            resultado = subprocess.run(comando, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            if resultado.returncode != 0:
                return f"Erro ao executar: {comando}\n{resultado.stderr}"

        # Verifique se o NDK foi descompactado corretamente
        if not os.path.exists(ndk_folder_name):
            return f"Erro: O NDK não foi descompactado corretamente. Pasta {ndk_folder_name} não encontrada."

        print(f"NDK baixado em: {os.getcwd()}/{ndk_folder_name}")

        resultado_config = subprocess.run(configurar_variavel, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if resultado_config.returncode != 0:
            return f"Erro ao configurar a variável de ambiente: {resultado_config.stderr}"

        return "Android NDK instalado e variável de ambiente configurada com sucesso!"

    except Exception as e:
        return f"Ocorreu um erro: {str(e)}"


def verificar_e_instalar_cargo_ndk():
    try:
        resultado = subprocess.run(["cargo", "ndk", "--version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        
        if resultado.returncode == 0:
            return "cargo-ndk já está instalado."
        
        print("cargo-ndk não encontrado. Iniciando instalação...")

        sistema_operacional = platform.system()

        resultado_cargo = subprocess.run(["cargo", "--version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if resultado_cargo.returncode != 0:
            return "Erro: Cargo não está instalado. Por favor, instale o Rust e Cargo antes de prosseguir."

        comando_instalar = "cargo install cargo-ndk"

        resultado_instalacao = subprocess.run(comando_instalar, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

        if resultado_instalacao.returncode != 0:
            return f"Erro ao instalar cargo-ndk: {resultado_instalacao.stderr}"

        return "cargo-ndk instalado com sucesso!"

    except Exception as e:
        return f"Ocorreu um erro: {str(e)}"


def compilar_e_copiar_bibliotecas_rust():
    try:
        mugen_dir = os.path.join(os.getcwd(), "mugen")

        if not os.path.exists(mugen_dir):
            return "Diretório mugen não encontrado."

        # Mapear arquiteturas para triplets do cargo-ndk
        arquiteturas = {
            "armeabi-v7a": "armv7-linux-androideabi",
            "arm64-v8a": "aarch64-linux-android",
            "x86": "i686-linux-android",
            "x86_64": "x86_64-linux-android"
        }

        # Caminho destino para os .so
        destino_base = os.path.join(os.getcwd(), "src/main/jniLibs")

        # Entra no diretório do projeto Mugen
        os.chdir(mugen_dir)

        # Compila o projeto para cada arquitetura
        for arch, triplet in arquiteturas.items():
            print(f"Compilando Mugen para {arch} ({triplet})...")

            comando = f"cargo ndk -t {triplet} -- build --release"
            resultado = subprocess.run(comando, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

            if resultado.returncode != 0:
                return f"Erro ao compilar Mugen para {arch}:\n{resultado.stderr}"

            # Caminho onde o .so é gerado
            so_dir = os.path.join(mugen_dir, "target", triplet, "release")
            so_files = [f for f in os.listdir(so_dir) if f.endswith(".so")]

            if not so_files:
                return f"Nenhum arquivo .so encontrado para Mugen em {arch}."

            # Cria o diretório destino se não existir
            destino_arquitetura = os.path.join(destino_base, arch)
            os.makedirs(destino_arquitetura, exist_ok=True)

            # Move o .so gerado para o diretório correspondente
            for so_file in so_files:
                src_so_path = os.path.join(so_dir, so_file)
                dest_so_path = os.path.join(destino_arquitetura, so_file)

                print(f"Copiando {so_file} para {destino_arquitetura}...")
                shutil.copy(src_so_path, dest_so_path)

        return "Compilação e cópia dos arquivos .so concluída com sucesso!"
    
    except Exception as e:
        return f"Ocorreu um erro: {str(e)}"


    except Exception as e:
        return f"Ocorreu um erro: {str(e)}"


def generate_random_key_iv():
    key = os.urandom(16).hex()  
    iv = os.urandom(8).hex()   
    return key, iv

def save_keys_to_file(key, iv, file_name="chaves.txt"):
    with open(file_name, 'w', encoding='utf-8') as file:
        file.write(f"Chave: {key}\n")
        file.write(f"IV: {iv}\n")
    print(f"Chaves salvas em {file_name}")

def update_key_and_iv_in_lib():
    file_path = "./mugen/src/utilit.rs"

    # Lê o conteúdo do arquivo
    if not os.path.exists(file_path):
        print(f"Erro: Arquivo {file_path} não encontrado.")
        return

    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()

    def set_random_keys():
        print("Tempo esgotado! Gerando chaves aleatórias...")
        key, iv = generate_random_key_iv()
        update_file_with_keys(content, key, iv)

    def update_file_with_keys(content, key, iv):
        # Usa regex para substituir as constantes KEY e NONCE no arquivo
        content = re.sub(r'const KEY: &\[u8\] = b".*";', f'const KEY: &[u8] = b"{key}";', content)
        content = re.sub(r'const NONCE: &\[u8\] = b".*";', f'const NONCE: &[u8] = b"{iv}";', content)

        # Grava o novo conteúdo no arquivo lib.rs
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write(content)

        # Salva as chaves em um arquivo separado
        save_keys_to_file(key, iv)

        # Imprime as novas chaves e IV
        print(f"Chave: {key}")
        print(f"IV: {iv}")

    # Pergunta ao usuário se ele quer fornecer a chave e o IV
    print("Você deseja fornecer uma chave e IV personalizados? (s/n)")
    resposta = input().strip().lower()

    # Cria um evento de threading para controlar o timeout
    timer = threading.Timer(200.0, set_random_keys)
    timer.start()

    try:
        if resposta == 's':
            key = input("Digite a chave (em hexadecimal de 32 caracteres): ").strip()
            iv = input("Digite o IV (em hexadecimal de 16 caracteres): ").strip()

            # Cancela o temporizador 
            timer.cancel()

            # Valida o comprimento da chave e do IV
            if len(key) != 32 or len(iv) != 16:
                print("Erro: A chave deve ter 32 caracteres e o IV deve ter 24 caracteres.")
                return
        else:
            # Cancela o temporizador se o usuário optar por gerar chaves aleatórias
            timer.cancel()
            print("Gerando chave e IV aleatórios...")
            key, iv = generate_random_key_iv()

        # Atualiza o arquivo com as chaves fornecidas ou geradas
        update_file_with_keys(content, key, iv)

    except KeyboardInterrupt:
        # Cancela o temporizador se o usuário interromper o input
        timer.cancel()
        print("\nOperação cancelada pelo usuário.")

def instalar_targets_rust():
    # Targets a serem adicionados
    targets = [
        "armv7-linux-androideabi",
        "aarch64-linux-android",
        "i686-linux-android",
        "x86_64-linux-android"
    ]

    # Verifica o sistema operacional
    sistema = platform.system()

    # Função para verificar se um target já foi adicionado
    def target_instalado(target):
        comando_verificar = ["rustup", "target", "list", "--installed"]
        resultado = subprocess.run(comando_verificar, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if resultado.returncode == 0:
            return target in resultado.stdout.splitlines()
        else:
            raise Exception(f"Erro ao verificar targets instalados: {resultado.stderr}")

    # Lista de targets que precisam ser instalados
    targets_para_instalar = [target for target in targets if not target_instalado(target)]

    if not targets_para_instalar:
        return "Todos os targets já estão instalados."

    # Comando base para adicionar os targets que faltam
    comando_base = ["rustup", "target", "add"] + targets_para_instalar

    try:
        if sistema == "Windows":
            # Executa o comando no Windows
            resultado = subprocess.run(comando_base, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        elif sistema == "Linux":
            # Executa o comando no Linux
            resultado = subprocess.run(comando_base, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        else:
            return f"Sistema operacional não suportado: {sistema}"

        # Verifica se o comando foi bem-sucedido
        if resultado.returncode == 0:
            return "Targets Rust instalados com sucesso!"
        else:
            return f"Erro ao instalar targets: {resultado.stderr}"

    except Exception as e:
        return f"Ocorreu um erro: {str(e)}"

def main():
    print("Preparando para a configuração do RASP!!!")
    rust_instalado = verificar_rust_instalado()
    if(rust_instalado != True):
        print("Rust Não encontrado no sistema instalando")
        instalar_rust()
    verificar_e_instalar_android_ndk()
    verificar_e_instalar_cargo_ndk()
    instalar_targets_rust()
    update_key_and_iv_in_lib()
    compilar_e_copiar_bibliotecas_rust()


if __name__ == "__main__":
    main()