# 🧵 Sincronización de Goroutines con Mutex en Go

[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Go Version](https://img.shields.io/badge/Go-1.16+-blue.svg)](https://golang.org/)
[![Plataformas](https://img.shields.io/badge/Plataformas-Linux%20%7C%20macOS%20%7C%20Windows-orange.svg)]()

## 🎯 Descripción General
🎯 Descripción General
Este proyecto demuestra el uso de semáforos para sincronizar el acceso a una variable compartida entre múltiples hilos en un programa concurrente escrito en Go. El objetivo es incremental un contador de manera segura, evitando condiciones de carrera que podrían llevar a resultados erróneos.
En un entorno concurrente, donde múltiples hilos de ejecución (goroutines en Go) acceden a recursos compartidos, es fundamental utilizar mecanismos de sincronización adecuados para prevenir problemas como las condiciones de carrera. En este ejemplo, se emplea un semáforo (mutex) para controlar el acceso al contador compartido.
Los semáforos son una primitiva de sincronización que permiten regular el acceso a secciones críticas del código, donde solo una unidad de ejecución (hilo) puede modificar los datos en un momento dado. Al bloquear y desbloquear el semáforo, se garantiza que el incremento del contador se realice de manera atómica y segura, evitando que los hilos interfieran entre sí.
En el programa, se crean dos goroutines que incrementan el mismo contador 10 millones de veces. Cada goroutine adquiere el bloqueo del semáforo antes de modificar el contador, y lo libera una vez finalizada la operación. Esto asegura que solo una goroutine pueda acceder al contador en un momento determinado, previniendo condiciones de carrera.
Sin embargo, el resultado observado muestra que el valor final del contador es incorrecto, lo que indica que hay un problema en la implementación del mecanismo de sincronización que debe ser identificado y corregido.
Este ejemplo ilustra los conceptos fundamentales de la concurrencia y la sincronización de hilos utilizando semáforos en Go, y cómo prevenir condiciones de carrera al acceder a recursos compartidos.


## 💻 Código Fuente 

```go
package main

import (
    "fmt"      // Paquete para formatear e imprimir texto
    "sync"     // Proporciona primitivas de sincronización
    "time"     // Manejo de operaciones temporales
)

// Variables globales compartidas
var (
    counter int           // Contador que será incrementado concurrentemente
    mutex   sync.Mutex    // Mutex para sincronizar el acceso al contador
)

// Función que será ejecutada por cada goroutine
func child(wg *sync.WaitGroup, id int) {
    // Asegura que se llame a wg.Done() cuando la función termine
    defer wg.Done()

    // Cada goroutine incrementa el contador 10 millones de veces
    for i := 0; i < 10000000; i++ {
        // Bloquea el mutex antes de modificar el contador
        // Esto previene condiciones de carrera
        mutex.Lock()
        
        // Incrementa el contador de manera segura
        counter++
        
        // Imprime el estado cada millón de incrementos
        if counter%1000000 == 0 {
            fmt.Printf("Goroutine %d: counter = %d\n", id, counter)
        }
        
        // Desbloquea el mutex para permitir que otras goroutines accedan
        mutex.Unlock()
        
        // Simula un pequeño retraso para mostrar comportamiento concurrente
        // Hace que cada goroutine "ceda" brevemente
        time.Sleep(1 * time.Millisecond)
    }
}

func main() {
    // Crea un WaitGroup para sincronizar la finalización de goroutines
    var wg sync.WaitGroup
    
    // Indica que esperaremos 2 goroutines
    wg.Add(2)
    
    // Lanza dos goroutines que ejecutarán la función child
    // Cada una con un ID único
    go child(&wg, 1)
    go child(&wg, 2)
    
    // Espera hasta que ambas goroutines completen su trabajo
    wg.Wait()
    
    // Imprime el resultado final del contador
    fmt.Printf("Final result: %d (should be 20000000)\n", counter)
}
```

## 🔍 Explicación Detallada

### Conceptos Clave
1. **Mutex (Exclusión Mutua)**
   - Previene que múltiples goroutines accedan simultáneamente a la sección crítica
   - `mutex.Lock()` bloquea el acceso
   - `mutex.Unlock()` libera el acceso

2. **WaitGroup**
   - Sincroniza la finalización de múltiples goroutines
   - `wg.Add(n)` establece el contador de goroutines
   - `wg.Done()` marca una goroutine como completada
   - `wg.Wait()` espera hasta que todas las goroutines terminen

3. **Concurrencia**
   - Dos goroutines incrementan el mismo contador
   - `time.Sleep()` simula trabajo y muestra interleaving

## 🧪 Análisis de Comportamiento

- **Objetivo**: Incrementar un contador compartido de manera segura
- **Resultado Esperado**: 20,000,000 (10,000,000 * 2 goroutines)
- **Problema Resuelto**: Condiciones de carrera
  ## 🚀 Instalación y Ejecución

### Requisitos del Sistema
- Go 1.16 o superior instalado en tu computadora

### Pasos de Instalación

1. **Instalación de Go**:
   - Descarga el paquete de Go desde el [sitio web oficial](https://golang.org/dl/) para tu sistema operativo. Por ejemplo, el instalador MSI para Windows.
   - Sigue las instrucciones de instalación proporcionadas para tu plataforma.

2. **Configuración del Entorno de Desarrollo**:
   - Utiliza un editor de código como Visual Studio Code.
   - Instala la extensión de Go desde la sección de extensiones de Visual Studio Code para obtener funcionalidades de desarrollo mejoradas.

### Pasos de Ejecución

1. **Clonación del Repositorio**:
   - Clona el repositorio del proyecto desde GitHub: `git clone https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata.git`
   - Navega hasta la carpeta del proyecto: `cd actividad_concurrencia_2024-2_CarlosZapata`

2. **Ejecución del Programa**:
   - En la terminal, ejecuta el programa con el comando `go run main.go`.
   - Verifica que se muestre el resultado en la consola.

## Resultado de la Ejecución

La imagen muestra la salida del programa en la terminal. Podemos ver que el resultado final del contador es "2000000000" en lugar del valor esperado de "20000000". Esto indica que hay un problema en la implementación que debe ser revisado y corregido.

![image](https://github.com/user-attachments/assets/8981773b-3fba-4325-af2e-4ab2527482ba)


## ✉️ Contacto
Carlos Zapata Arango
- GitHub: [@zapata-git](https://github.com/zapata-git)
- Repositorio: [actividad_concurrencia_2024-2_CarlosZapata](https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata)

---
© 2024 Carlos Zapata Arango. Todos los derechos reservados.
