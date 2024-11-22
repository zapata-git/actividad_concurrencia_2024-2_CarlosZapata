# 🧵 Implementación de Hilos en Python

[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Python Version](https://img.shields.io/badge/python-3.x-blue.svg)](https://www.python.org/downloads/)
[![GitHub](https://img.shields.io/badge/GitHub-zapata--git-darkgreen.svg)](https://github.com/zapata-git)
[![TQDM](https://img.shields.io/badge/TQDM-latest-orange.svg)](https://github.com/tqdm/tqdm)

## 🎯 Descripción General
Este proyecto demuestra la implementación práctica de programación concurrente utilizando hilos (threads) en Python, con ejemplos detallados de threading y visualización de progreso en tiempo real.

## 📋 Estado de Implementación

### Reimplementación de Códigos
- ✅ `thread_create.c`
- ⏳ `thread_create_simple_args.c`
- ⏳ `thread_create_with_return_args.c`

## 🛠️ Prerrequisitos

- Python 3.x
- Módulos requeridos:
  ```python
  threading  # Módulo estándar para hilos
  time      # Módulo estándar para tiempo
  tqdm      # Librería para barras de progreso
  ```

## 📥 Instalación

```bash
# Clonar el repositorio
git clone https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata.git

# Navegar al directorio
cd actividad_concurrencia_2024-2_CarlosZapata

# Instalar dependencias
pip install tqdm
```

## 💻 Código Fuente Comentado

```python
import threading
import time
from tqdm import tqdm

def mythread(args, thread_id):
    """
    Función principal del hilo que procesa argumentos y muestra progreso
    
    Args:
        args (tuple): Tupla con dos valores numéricos
        thread_id (int): Identificador único del hilo
    """
    a, b = args
    print(f"\n[Hilo {thread_id}] Iniciando...")
    
    for i in tqdm(range(10), desc=f"[Hilo {thread_id}] Procesando", unit="tarea"):
        time.sleep(0.5)
    
    print(f"\n[Hilo {thread_id}] Valores recibidos: {a} {b}")
    print(f"[Hilo {thread_id}] Finalizando...\n")

if __name__ == "__main__":
    print("=== Programa de Ejemplo con Hilos y Barra de Progreso ===")
    
    print("Ingrese dos números para el hilo:")
    a = int(input("Número 1: "))
    b = int(input("Número 2: "))
    
    args = (a, b)
    thread_id = 1
    
    thread = threading.Thread(target=mythread, args=(args, thread_id))
    print(f"\n[Principal] Creando el hilo {thread_id}...")
    
    thread.start()
    print(f"[Principal] Hilo {thread_id} iniciado.\n")
    
    thread.join()
    print("[Principal] Hilo finalizado. ¡Todo listo!\n")
```

## 🚀 Uso y Ejecución

```bash
python ejemplo_threading.py
```

### 📝 Ejemplo de Salida
```
=== Programa de Ejemplo con Hilos y Barra de Progreso ===
Ingrese dos números para el hilo:
Número 1: 10
Número 2: 20

[Hilo 1] Iniciando...
[Hilo 1] Procesando: 100%|██████████| 10/10 [00:05<00:00, 0.50s/tarea]
[Hilo 1] Valores recibidos: 10 20
[Hilo 1] Finalizando...
```

## 🏗️ Arquitectura del Código

### 1. Componentes Principales
- **Función mythread**
  - Maneja argumentos de entrada
  - Implementa barra de progreso
  - Simula procesamiento
  - Reporta estado

### 2. Flujo de Ejecución
1. Solicitud de entrada de datos
2. Creación del hilo
3. Ejecución del procesamiento
4. Visualización del progreso
5. Finalización y reporte

## 📚 Conceptos Clave Implementados

### 1. Concurrencia
- Ejecución paralela de tareas
- Gestión de recursos
- Sincronización de procesos

### 2. Threading en Python
- Creación de hilos
- Paso de argumentos
- Control de ejecución

### 3. Visualización
- Barras de progreso interactivas
- Reportes de estado
- Mensajes informativos

## 📖 Referencias y Recursos

### Literatura Técnica
- 📘 "Operating Systems: Three Easy Pieces"
  - Autores: Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau
  - [Sitio oficial del libro](http://pages.cs.wisc.edu/~remzi/OSTEP/)

### Documentación
- [Threading en Python](https://docs.python.org/3/library/threading.html)
- [TQDM Documentation](https://tqdm.github.io/)
- [Cómo usar hilos en Python - Código Pitón](https://www.codigopiton.com/como-usar-hilos-o-threads-en-python/)

## 🤝 Contribución

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
