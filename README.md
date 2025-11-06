
# Proyecto `SO`

Este repositorio es un proyecto Rust gestionado con Cargo. Este README explica cómo instalar Rust en Windows (usando PowerShell) y cómo compilar y ejecutar el proyecto localmente.

## Requisitos

- Windows (PowerShell). En este documento se usan comandos para PowerShell 5.1.
- Conexión a Internet para descargar el instalador de Rust.

## 1) Instalar Rust en Windows 

La forma recomendada es usar rustup. Abre PowerShell con permisos de usuario normal (no es necesario administrador en la mayoría de los casos) y ejecuta uno de los siguientes métodos.

Instalación automática:

```powershell
Invoke-WebRequest -Uri https://win.rustup.rs -UseBasicParsing -OutFile rustup-init.exe; 
.\rustup-init.exe -y; 
Remove-Item rustup-init.exe
```

Manual:

1. Visita https://rustup.rs en tu navegador.
2. Sigue las instrucciones y descarga el instalador para Windows.
3. Ejecuta el instalador y acepta las opciones por defecto (instala toolchain 'stable' y agrega la ruta a PATH).

## 2) Verificar la instalación

En PowerShell ejecuta:

```powershell
rustc --version
cargo --version
rustup --version
```

Deberías ver versiones impresas, por ejemplo `rustc 1.XX.0` y `cargo 1.XX.0`.

Si recibes un error "command not found", asegúrate de que `C:\Users\<TU_USUARIO>\.cargo\bin` esté en tu `PATH` y reinicia PowerShell.

## 3) Ejecutar este proyecto

Este repo es un proyecto Cargo. Para compilar y ejecutar en modo de desarrollo (rápido):

1. Abre PowerShell y navega a la carpeta del proyecto (donde está `Cargo.toml`). Ejemplo:

```powershell
cd C:\Users\jcast\Documents\dev\so
```

2. Ejecuta el binario directamente (modo desarrollo):

```powershell
cargo run
```

Esto compilará el proyecto y ejecutará el binario.


## Recursos

- Documentación oficial de Rust: https://www.rust-lang.org
- rustup (instalador): https://rustup.rs
- Guía de Cargo: https://doc.rust-lang.org/cargo/
