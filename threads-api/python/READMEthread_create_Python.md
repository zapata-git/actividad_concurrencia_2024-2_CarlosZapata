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

## 💻 Código Fuente 

### Importaciones y Dependencias
```python
# Threading: Módulo fundamental para la creación y gestión de hilos en Python
import threading

# Time: Módulo para manejo de tiempos y delays
import time

# TQDM: Librería para crear barras de progreso visuales e interactivas
from tqdm import tqdm
```

### Implementación del Hilo Principal
```python
def mythread(args, thread_id):
    """
    Función principal que ejecuta la lógica del hilo y muestra el progreso de ejecución.
    
    Args:
        args (tuple): Tupla conteniendo dos valores numéricos para procesamiento
        thread_id (int): Identificador único del hilo para seguimiento
    
    Comportamiento:
        1. Desempaqueta los argumentos recibidos
        2. Simula procesamiento con barra de progreso
        3. Reporta el estado inicial y final del hilo
        4. Muestra los valores procesados
    """
    # Desempaquetado de argumentos mediante destructuring
    a, b = args
    
    # Notificación de inicio del hilo con formato consistente
    print(f"\n[Hilo {thread_id}] Iniciando...")
    
    # Simulación de procesamiento con barra de progreso
    for i in tqdm(
        range(10),                           # 10 iteraciones de proceso
        desc=f"[Hilo {thread_id}] Procesando", # Descripción en la barra
        unit="tarea"                         # Unidad de medida del progreso
    ):
        # Pausa de 500ms para simular trabajo real
        time.sleep(0.5)
    
    # Reporte de valores procesados
    print(f"\n[Hilo {thread_id}] Valores recibidos: {a} {b}")
    
    # Notificación de finalización del hilo
    print(f"[Hilo {thread_id}] Finalizando...\n")
```

### Punto de Entrada y Control Principal
```python
if __name__ == "__main__":
    """
    Punto de entrada principal del programa.
    Gestiona la creación, ejecución y finalización del hilo.
    """
    # Banner de inicio del programa
    print("=== Programa de Ejemplo con Hilos y Barra de Progreso ===")
    
    # Recolección de datos de entrada
    print("Ingrese dos números para el hilo:")
    a = int(input("Número 1: "))  # Conversión explícita a entero
    b = int(input("Número 2: "))  # Manejo básico de entrada
    
    # Empaquetado de argumentos en tupla para paso al hilo
    args = (a, b)
    thread_id = 1  # Identificador único del hilo
    
    # Creación del objeto Thread
    thread = threading.Thread(
        target=mythread,    # Función que ejecutará el hilo
        args=(args, thread_id)  # Argumentos pasados a la función
    )
    
    # Notificación de creación del hilo
    print(f"\n[Principal] Creando el hilo {thread_id}...")
    
    # Inicio de la ejecución del hilo
    thread.start()
    print(f"[Principal] Hilo {thread_id} iniciado.\n")
    
    # Espera por la finalización del hilo
    thread.join()  # Bloquea hasta que el hilo termine
    
    # Notificación de finalización del programa
    print("[Principal] Hilo finalizado. ¡Todo listo!\n")
```

### 🔍 Detalles Técnicos Importantes

#### 1. Gestión de Hilos
- **Creación**: Utiliza `threading.Thread()` con parámetros nombrados
- **Ejecución**: Implementa `start()` para iniciar el hilo
- **Sincronización**: Usa `join()` para esperar finalización

#### 2. Estructura de Datos
- **Tuplas**: Empaquetado de argumentos inmutable
- **Identificadores**: Sistema de tracking con IDs únicos
- **Estados**: Mensajes formatados con contexto del hilo

#### 3. Visualización de Progreso
- **TQDM**: Barra de progreso interactiva
- **Formato**: Consistente con prefijos `[Hilo X]`
- **Temporización**: Delays controlados con `time.sleep()`

### 🔧 Características Técnicas

1. **Seguridad de Tipos**
   - Conversión explícita de entradas a `int`
   - Tuplas inmutables para argumentos
   - Identificadores numéricos para seguimiento

2. **Manejo de Estados**
   - Mensajes de estado claros y consistentes
   - Seguimiento del ciclo de vida del hilo
   - Notificaciones en puntos clave de ejecución

3. **Visualización**
   - Barra de progreso interactiva con TQDM
   - Formato consistente en mensajes
   - Claridad en la separación de outputs

4. **Sincronización**
   - Control de ejecución con `start()`
   - Espera sincronizada con `join()`
   - Simulación realista con `sleep()`

### 📊 Flujo de Ejecución

1. **Inicialización**
   ```
   Programa → Entrada de datos → Creación de hilo
   ```

2. **Ejecución**
   ```
   Inicio de hilo → Procesamiento con barra → Reporte de valores
   ```

3. **Finalización**
   ```
   Término de proceso → Join del hilo → Mensaje de finalización
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
