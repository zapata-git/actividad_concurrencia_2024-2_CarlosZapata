# Implementación en Python

Los ejemplos que se implementaron son los que se encuentran en la sección [threads-api](../../threads-api/)

## Referencias principales
1. **Hilos en Python**  
   [Cómo usar hilos o threads en Python - Código Pitón](https://www.codigopiton.com/como-usar-hilos-o-threads-en-python/)

2. **Libros sobre Sistemas Operativos y Concurrencia**  
   "Operating Systems: Three Easy Pieces" por Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau.  
   [Sitio oficial](http://pages.cs.wisc.edu/~remzi/OSTEP/)


## Ejemplos

Los códigos a reimplementar:
- [x] `thread_create.c`
- [ ] `thread_create_simple_args.c`
- [ ] `thread_create_with_return_args.c`

## Codigos
# Hilos en Python con Barra de Progreso

## Descripción
Este proyecto implementa un programa en Python que utiliza hilos (`threading`) y una barra de progreso (`tqdm`) para simular el procesamiento de tareas. El programa incluye una interfaz interactiva en la terminal y personalización visual.

# Explicación del Código

\# Programa de Ejemplo con Hilos y Barra de Progreso

Este es un programa en Python que demuestra cómo utilizar hilos (`threading`) para ejecutar tareas en paralelo, junto con una barra de progreso visualizada mediante la librería `tqdm`. En el programa, se crean hilos que realizan una tarea simulada y muestran el progreso mediante una barra que indica el avance de cada hilo.

\## Descripción

El programa utiliza la librería `threading` para crear un hilo que ejecuta una función llamada `mythread`. Esta función recibe dos números como argumentos, realiza una tarea simulada (una pausa de medio segundo) 10 veces y muestra el progreso utilizando `tqdm`. Cada hilo es identificado por un `thread\_id`, lo que permite diferenciar las tareas.

\## Requisitos

Para ejecutar este programa, necesitas tener Python instalado y las siguientes librerías:

\- `threading`: Esta librería es parte del módulo estándar de Python, por lo que no requiere instalación adicional.

\- `tqdm`: Se usa para crear la barra de progreso. Puedes instalarla ejecutando el siguiente comando:



`  ````bash

`  `pip install tqdm

**Funcionamiento del Programa**

1. El programa solicita al usuario dos números de entrada.
1. Luego, crea un hilo que ejecuta la función mythread, pasando los dos números como argumentos y un identificador de hilo.
1. La función mythread realiza 10 iteraciones, simulando el procesamiento con una pausa de 0.5 segundos en cada paso, y muestra una barra de progreso.
1. Cuando el hilo termina, imprime los valores recibidos y finaliza el proceso.

**Código**

python

Copiar código

import threading

import time

from tqdm import tqdm

def mythread(args, thread\_id):

`    `a, b = args  

`    `print(f"\n[Hilo {thread\_id}] Iniciando...")



`    `for i in tqdm(range(10), desc=f"[Hilo {thread\_id}] Procesando", unit="tarea"):

`        `time.sleep(0.5)  



`    `print(f"\n[Hilo {thread\_id}] Valores recibidos: {a} {b}")

`    `print(f"[Hilo {thread\_id}] Finalizando...\n")

if \_\_name\_\_ == "\_\_main\_\_":

`    `print("=== Programa de Ejemplo con Hilos y Barra de Progreso ===")



`    `print("Ingrese dos números para el hilo:")

`    `a = int(input("Número 1: "))

`    `b = int(input("Número 2: "))

`    `args = (a, b)  



`    `thread\_id = 1

`    `thread = threading.Thread(target=mythread, args=(args, thread\_id))

`    `print(f"\n[Principal] Creando el hilo {thread\_id}...")



`    `thread.start()

`    `print(f"[Principal] Hilo {thread\_id} iniciado.\n")



`    `thread.join()

`    `print("[Principal] Hilo finalizado. ¡Todo listo!\n")

**Explicación del Código**

1. **Importación de Librerías:**
   1. threading: Se usa para trabajar con hilos en Python.
   1. time: Se utiliza para pausar la ejecución del programa (simulando tareas).
   1. tqdm: Es la librería que genera la barra de progreso para cada hilo.
1. **Función mythread:**
   1. Toma dos números como entrada y simula una tarea que toma tiempo.
   1. Usa tqdm para mostrar una barra de progreso mientras procesa las 10 tareas.
1. **Bloque Principal (\_\_main\_\_):**
   1. Solicita al usuario dos números.
   1. Crea y arranca un hilo para ejecutar mythread.
   1. Usa join() para esperar a que el hilo termine antes de continuar con la ejecución del programa.

**Ejecución**

1. Al ejecutar el programa, se solicita que ingreses dos números.
1. El programa crea un hilo que comienza a ejecutar la función mythread.
1. Verás una barra de progreso que muestra el avance de las tareas dentro del hilo.
1. Una vez que el hilo termine su tarea, el programa imprimirá los valores recibidos y finalizará.

**Salida Esperada**

Al ejecutar el programa, deberías ver algo como esto:

less

Copiar código

=== Programa de Ejemplo con Hilos y Barra de Progreso ===

Ingrese dos números para el hilo:

Número 1: 5

Número 2: 10

[Principal] Creando el hilo 1...

[Principal] Hilo 1 iniciado.

[Hilo 1] Iniciando...

[Hilo 1] Procesando: 100%|██████████| 10/10 [00:05<00:00,  2.00 tarea/s]

[Hilo 1] Valores recibidos: 5 10

[Hilo 1] Finalizando...

[Principal] Hilo finalizado. ¡Todo listo!

**Conclusión**

Este programa es un ejemplo básico de cómo trabajar con hilos en Python y usar una barra de progreso para visualizar el avance de tareas en paralelo. Es útil para entender la gestión de múltiples hilos y la sincronización de tareas dentro de un programa.

r

Copiar código

Este `README.md` está diseñado para ser claro y detallado, proporcionando una descripción general, requisitos, explicación del código y ejemplos de salida.

4o mini



## Referencias

1. **Hilos en Python**  
   [Cómo usar hilos o threads en Python - Código Pitón](https://www.codigopiton.com/como-usar-hilos-o-threads-en-python/)

2. **Libros sobre Sistemas Operativos y Concurrencia**  
   "Operating Systems: Three Easy Pieces" por Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau.  
   [Sitio oficial](http://pages.cs.wisc.edu/~remzi/OSTEP/)
