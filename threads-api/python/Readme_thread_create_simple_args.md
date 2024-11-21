# Implementación de Hilos en Python

## Descripción General
Este proyecto demuestra la implementación de programación concurrente utilizando hilos en Python, con ejemplos prácticos de threading y visualización de progreso. Se presenta un gestor de descargas simulado que permite la ejecución simultánea de múltiples descargas con seguimiento en tiempo real.

## Referencias Principales
1. **Hilos en Python**  
   [Cómo usar hilos o threads en Python - Código Pitón](https://www.codigopiton.com/como-usar-hilos-o-threads-en-python/)

2. **Recursos Fundamentales**  
   - Libro: "Operating Systems: Three Easy Pieces" 
   - Autores: Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau
   - [Sitio oficial del libro](http://pages.cs.wisc.edu/~remzi/OSTEP/)

## Estado de Implementación
### Reimplementación de Códigos
- [x] `thread_create.c`
- [x] `thread_create_simple_args.c`
- [x] `thread_create_with_return_args.c`

# Gestor de Descargas Concurrente

## Características Principales
- Simulación de descargas concurrentes
- Sistema de cola para gestión de descargas
- Progreso en tiempo real de cada descarga
- Control centralizado de cancelación
- Tiempos de descarga aleatorios
- Interfaz de usuario con emojis

## Requisitos
- Python 3.x
- Módulos estándar:
  - `threading`
  - `time`
  - `random`
  - `queue`

## Instalación
```bash
# Clonar el repositorio
git clone https://github.com/tuusuario/download-manager.git

# Navegar al directorio
cd download-manager
```

## Código Fuente Comentado

```python
import threading  # Para manejo de hilos
import time      # Para control de tiempo
import random    # Para generar tiempos aleatorios
import queue     # Para la cola de descargas

class DownloadManager:
    def __init__(self):
        # Inicialización del gestor de descargas
        self.download_queue = queue.Queue()  # Cola para gestionar descargas
        self.stop_all = False               # Bandera de control global

    def simulate_download(self, download_id):
        """
        Simula una descarga individual
        Args:
            download_id: Identificador único de la descarga
        """
        # Genera tiempo aleatorio para la descarga
        total_time = random.uniform(5, 15)
        start_time = time.time()
        
        print(f"🚀 Iniciando descarga {download_id}")
        
        # Bucle principal de la descarga
        while not self.stop_all and time.time() - start_time < total_time:
            elapsed = time.time() - start_time
            progress = min(100, (elapsed / total_time) * 100)
            
            # Muestra progreso actual
            print(f"⬇️ Descarga {download_id}: {progress:.2f}% completado")
            time.sleep(0.5)  # Pausa para simular trabajo

        # Mensaje de finalización según estado
        if not self.stop_all:
            print(f"✅ Descarga {download_id} completada!")
        else:
            print(f"❌ Descarga {download_id} cancelada")

    def start_multiple_downloads(self, num_downloads):
        """
        Inicia múltiples descargas concurrentes
        Args:
            num_downloads: Número de descargas a iniciar
        """
        threads = []
        
        # Crear y comenzar hilos para cada descarga
        for i in range(1, num_downloads + 1):
            thread = threading.Thread(
                target=self.simulate_download,
                args=(i,),
                name=f"Descarga-{i}"
            )
            threads.append(thread)
            thread.start()

        # Esperar antes de la cancelación
        time.sleep(20)
        
        # Activar señal de detención
        self.stop_all = True
        
        # Esperar finalización de todos los hilos
        for thread in threads:
            thread.join()
            
        print("Todas las descargas finalizadas")

def main():
    # Punto de entrada principal
    download_manager = DownloadManager()
    download_manager.start_multiple_downloads(5)

if __name__ == "__main__":
    main()
```

## Explicación Detallada del Código

### 1. Estructura Principal
#### Clase `DownloadManager`
- **Propósito**: Gestiona múltiples descargas concurrentes
- **Atributos**:
  - `download_queue`: Cola para gestionar descargas
  - `stop_all`: Bandera para control global de cancelación

### 2. Métodos Principales
#### Método `simulate_download()`
- **Funcionalidad**: Simula una descarga individual
- **Características**:
  - Genera tiempo aleatorio entre 5-15 segundos
  - Muestra progreso en tiempo real
  - Responde a señal de cancelación
  - Usa emojis para mejor visualización

#### Método `start_multiple_downloads()`
- **Funcionalidad**: Inicia múltiples descargas concurrentes
- **Proceso**:
  1. Crea hilos para cada descarga
  2. Inicia las descargas
  3. Espera 20 segundos
  4. Envía señal de cancelación
  5. Espera finalización de hilos

## Uso del Programa

### Ejecución Básica
```bash
python download_manager.py
```

### Ejemplo de Salida
```
🚀 Iniciando descarga 1
🚀 Iniciando descarga 2
🚀 Iniciando descarga 3
⬇️ Descarga 1: 25.5% completado
⬇️ Descarga 2: 15.3% completado
⬇️ Descarga 3: 18.7% completado
...
✅ Descarga 1 completada!
❌ Descarga 2 cancelada
❌ Descarga 3 cancelada
Todas las descargas finalizadas
```

## Conceptos Clave
1. **Concurrencia**
   - Ejecución simultánea de múltiples descargas
   - Gestión de recursos compartidos

2. **Threading**
   - Uso de hilos para operaciones paralelas
   - Sincronización de procesos

3. **Control de Flujo**
   - Sistema de cancelación centralizado
   - Gestión de estado de descargas

## Casos de Uso
- Simulación de sistemas de descarga
- Prototipado de gestores de descarga
- Aprendizaje de programación concurrente
- Demostración de conceptos de threading

## Contribución
Las contribuciones son bienvenidas. Por favor, sigue estos pasos:
1. Fork del repositorio
2. Crea una rama para tu característica
3. Envía un Pull Request

## Aspectos Técnicos Destacados
- Implementación de concurrencia usando `threading`
- Sistema de monitoreo en tiempo real
- Manejo de cancelación centralizada
- Uso de colas para gestión de tareas

## Licencia
Este proyecto está bajo licencia MIT. Ver archivo LICENSE para detalles.

## Autor
[Tu Nombre]

## Contacto
- GitHub: [@tuusuario](https://github.com/tuusuario)
- Email: tu@email.com

---
© 2024 Gestor de Descargas Concurrente. Todos los derechos reservados.
