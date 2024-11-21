# Implementación de Hilos en Python

## Descripción General
Este proyecto demuestra la implementación de programación concurrente utilizando hilos en Python, con ejemplos prácticos de threading y visualización de progreso.

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
- [ ] `thread_create_simple_args.c`
- [ ] `thread_create_with_return_args.c`

## Código Fuente

```python
# Importación de librerías necesarias
import threading  # Módulo para manejar hilos
import time       # Módulo para control de tiempo
from tqdm import tqdm  # Librería para barras de progreso

# Función principal del hilo con argumentos y ID de hilo
def mythread(args, thread_id):
    # Desempaqueta los argumentos recibidos
    a, b = args
    
    # Mensaje de inicio del hilo
    print(f"\n[Hilo {thread_id}] Iniciando...")
    
    # Barra de progreso con 10 iteraciones
    # Simula procesamiento con tiempo de espera
    for i in tqdm(range(10), desc=f"[Hilo {thread_id}] Procesando", unit="tarea"):
        time.sleep(0.5)  # Pausa de medio segundo en cada iteración
    
    # Impresión de valores recibidos
    print(f"\n[Hilo {thread_id}] Valores recibidos: {a} {b}")
    
    # Mensaje de finalización del hilo
    print(f"[Hilo {thread_id}] Finalizando...\n")

# Punto de entrada principal del programa
if __name__ == "__main__":
    # Mensaje de bienvenida
    print("=== Programa de Ejemplo con Hilos y Barra de Progreso ===")
    
    # Solicitud de entrada de dos números
    print("Ingrese dos números para el hilo:")
    a = int(input("Número 1: "))
    b = int(input("Número 2: "))
    
    # Empaquetado de argumentos
    args = (a, b)
    thread_id = 1
    
    # Creación del hilo
    thread = threading.Thread(target=mythread, args=(args, thread_id))
    
    # Mensaje de creación de hilo
    print(f"\n[Principal] Creando el hilo {thread_id}...")
    
    # Inicio del hilo
    thread.start()
    print(f"[Principal] Hilo {thread_id} iniciado.\n")
    
    # Espera a que el hilo termine
    thread.join()
    
    # Mensaje de finalización
    print("[Principal] Hilo finalizado. ¡Todo listo!\n")
```

### Explicación Detallada del Código

#### Importaciones
- `threading`: Módulo fundamental para crear y gestionar hilos en Python
- `time`: Permite controlar pausas y tiempo de ejecución
- `tqdm`: Biblioteca para crear barras de progreso interactivas

#### Función `mythread(args, thread_id)`
- **Parámetros**:
  - `args`: Tupla con dos valores de entrada
  - `thread_id`: Identificador del hilo
- **Funcionalidades**:
  - Imprime mensaje de inicio
  - Simula procesamiento con barra de progreso
  - Usa `time.sleep()` para simular trabajo
  - Imprime valores recibidos
  - Muestra mensaje de finalización

#### Bloque Principal `__main__`
- Punto de entrada del programa
- Solicita dos números al usuario
- Crea un hilo con `threading.Thread()`
- Inicia el hilo con `thread.start()`
- Espera finalización con `thread.join()`

## Detalles Técnicos

### Funcionamiento del Programa
1. **Entrada de Datos**
   - El programa solicita dos números al usuario
   - Crea un hilo pasando los números como argumentos

2. **Procesamiento del Hilo**
   - Ejecuta la función `mythread`
   - Simula procesamiento con 10 iteraciones
   - Cada iteración tiene una pausa de 0.5 segundos
   - Muestra barra de progreso usando `tqdm`

3. **Características del Código**
   - Utiliza el módulo `threading`
   - Implementa barra de progreso con `tqdm`
   - Gestiona la ejecución de tareas en paralelo

### Requisitos
- Python 3.x
- Biblioteca `tqdm`

### Instalación
```bash
# Clonar repositorio
git clone https://github.com/tuusuario/threading-python.git

# Instalar dependencias
pip install tqdm
```

### Ejecución
```bash
python ejemplo_threading.py
```

## Ejemplo de Salida
```
=== Programa de Ejemplo con Hilos y Barra de Progreso ===
Ingrese dos números para el hilo:
Número 1: 10
Número 2: 20

[Hilo 1] Iniciando...
[Hilo 1] Procesando: 100%|██████████| 10/10 [00:05<00:00,  0.50s/tarea]
[Hilo 1] Valores recibidos: 10 20
[Hilo 1] Finalizando...
```

## Conceptos Clave
- **Concurrencia**: Ejecución de múltiples tareas simultáneamente
- **Threads**: Unidades de ejecución ligeras dentro de un proceso
- **Barra de Progreso**: Visualización del avance de tareas

## Conclusión
Este ejemplo ilustra conceptos básicos de threading en Python, mostrando cómo crear hilos, simular procesamiento y visualizar progreso.

## Licencia
Proyecto de código abierto. Consultar archivo de licencia para detalles.
