# 🧵 Sincronización de Goroutines con Mutex en Go

[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Go Version](https://img.shields.io/badge/Go-1.16+-blue.svg)](https://golang.org/)
[![Plataformas](https://img.shields.io/badge/Plataformas-Linux%20%7C%20macOS%20%7C%20Windows-orange.svg)]()

## 🎯 Descripción General
Demostración de sincronización concurrente en Go utilizando mutex para prevenir condiciones de carrera al incrementar un contador compartido.

## 💻 Código Fuente Comentado

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

## Resultado de la Ejecución

La imagen muestra la salida del programa en la terminal. Podemos ver que el resultado final del contador es "2000000000" en lugar del valor esperado de "20000000". Esto indica que hay un problema en la implementación que debe ser revisado y corregido.

![Resultado de la ejecución](https://raw.githubusercontent.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata/main/result.png)

## ✉️ Contacto
Carlos Zapata Arango
- GitHub: [@zapata-git](https://github.com/zapata-git)
- Repositorio: [actividad_concurrencia_2024-2_CarlosZapata](https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata)

---
© 2024 Carlos Zapata Arango. Todos los derechos reservados.
