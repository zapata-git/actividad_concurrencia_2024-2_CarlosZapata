# Implementación de Concurrencia en Python

[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Python Version](https://img.shields.io/badge/python-3.x-blue.svg)](https://www.python.org/downloads/)
[![GitHub](https://img.shields.io/badge/GitHub-zapata--git-darkgreen.svg)](https://github.com/zapata-git)

Un proyecto educativo que demuestra la implementación de programación concurrente utilizando hilos en Python, desarrollado como parte de la actividad de concurrencia 2024-2.

## 🎯 Objetivo del Proyecto

Este proyecto implementa un gestor de descargas concurrente para demostrar conceptos prácticos de threading en Python, permitiendo ejecutar y monitorear múltiples descargas simultáneas con seguimiento en tiempo real.

## 🚀 Características Principales

- ✨ Simulación de descargas concurrentes
- 📊 Monitoreo de progreso en tiempo real
- 🎮 Sistema de control centralizado
- 🔄 Implementación de cola de descargas
- ⏱️ Simulación con tiempos aleatorios
- 🎯 Interfaz con indicadores visuales

## 📋 Prerrequisitos

- Python 3.x
- Módulos estándar de Python:
  ```python
  import threading
  import time
  import random
  import queue
  ```

## 🛠️ Instalación

```bash
# Clonar el repositorio
git clone https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata.git

# Navegar al directorio del proyecto
cd actividad_concurrencia_2024-2_CarlosZapata/threads-api/python
```

## 💻 Uso

Ejecutar el gestor de descargas:

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

## 🏗️ Arquitectura del Proyecto

### Clase DownloadManager

La clase principal que gestiona las operaciones de descarga:

```python
class DownloadManager:
    def __init__(self):
        self.download_queue = queue.Queue()  # Cola de descargas
        self.stop_all = False               # Control de cancelación
```

### Componentes Principales

#### 1. Simulación de Descarga
```python
def simulate_download(self, download_id):
    """
    Simula una descarga individual con progreso en tiempo real
    """
```

#### 2. Gestor de Descargas Múltiples
```python
def start_multiple_downloads(self, num_downloads):
    """
    Inicia y coordina múltiples descargas concurrentes
    """
```

## 📚 Conceptos Implementados

### 1. Concurrencia
- Ejecución simultánea de tareas
- Gestión de recursos compartidos
- Manejo de estados concurrentes

### 2. Threading en Python
- Implementación de hilos
- Sincronización entre procesos
- Control de ciclo de vida

### 3. Patrones de Diseño
- Sistema de cola para gestión de tareas
- Control centralizado de cancelación
- Monitoreo en tiempo real

## 📖 Referencias

### Literatura Técnica
- "Operating Systems: Three Easy Pieces" - Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau
  - [Sitio oficial](https://pages.cs.wisc.edu/~remzi/OSTEP/)

### Documentación
- [Threading en Python](https://docs.python.org/3/library/threading.html)
- [Queue en Python](https://docs.python.org/3/library/queue.html)

## 🤝 Contribución

Las contribuciones son bienvenidas. Para contribuir:

1. Fork del repositorio
2. Crea una rama para tu feature (`git checkout -b feature/NuevaCaracteristica`)
3. Commit de tus cambios (`git commit -m 'Add: Nueva Caracteristica'`)
4. Push a la rama (`git push origin feature/NuevaCaracteristica`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## ✉️ Contacto

Carlos Zapata Arango
- GitHub: [@zapata-git](https://github.com/zapata-git)
- Repositorio: [actividad_concurrencia_2024-2_CarlosZapata](https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata)

---
© 2024 Carlos Zapata Arango. Todos los derechos reservados.
