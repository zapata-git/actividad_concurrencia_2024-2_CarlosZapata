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
README.md
Título del Proyecto
Hilos en Python con Barra de Progreso

Descripción
Este proyecto implementa un programa en Python que utiliza hilos (threading) y una barra de progreso (tqdm) para simular el procesamiento de tareas. El programa incluye una interfaz interactiva en la terminal y personalización visual, como una barra de progreso verde.

Requisitos
Antes de ejecutar el programa, asegúrate de tener Python instalado en tu sistema y la biblioteca tqdm. Si no tienes tqdm, instálalo con el siguiente comando:

pip install tqdm
Cómo Ejecutar
Clona este repositorio en tu máquina local:

git clone https://github.com/tu-usuario/tu-repositorio.git
Accede a la carpeta del proyecto:

cd tu-repositorio
Ejecuta el programa:

python hilos_barra.py
Sigue las instrucciones en pantalla para ingresar los números y ver el progreso del hilo.

Explicación del Código
1. Importar bibliotecas
import threading
import time
from tqdm import tqdm
threading: Maneja la creación y ejecución de hilos.
time: Permite simular tareas mediante pausas (time.sleep).
tqdm: Genera barras de progreso dinámicas en la terminal.
2. Función del hilo
def mythread(args, thread_id):
    a, b = args  # Desempaquetar los argumentos
    print(f"\n[Hilo {thread_id}] Iniciando...")
    
    # Crear una barra de progreso con color verde
    for i in tqdm(
        range(10),
        desc=f"\033[92m[Hilo {thread_id}] Procesando\033[0m",  # Texto en verde
        bar_format="\033[92m{l_bar}{bar}| {n_fmt}/{total_fmt} [{elapsed}<{remaining}]\033[0m",
        unit="tarea",
    ):
        time.sleep(0.5)  # Simula el trabajo con una pausa de 0.5 segundos
    
    print(f"\n[Hilo {thread_id}] Valores recibidos: {a} {b}")
    print(f"[Hilo {thread_id}] Finalizando...\n")
Entrada:
args: Una tupla con dos números ingresados por el usuario.
thread_id: Un identificador para el hilo.
Descripción:
Se imprime un mensaje indicando que el hilo comienza.
La barra de progreso tiene 10 pasos (simulando un trabajo con range(10)).
Cada paso se acompaña de una pausa de 0.5 segundos.
El formato de la barra incluye texto en verde con códigos ANSI.
Al finalizar, se imprimen los valores procesados y un mensaje de conclusión.
3. Bloque principal
if __name__ == "__main__":
    print("=== Programa de Ejemplo con Hilos y Barra de Progreso ===")
    
    # Pedir datos al usuario
    print("Ingrese dos números para el hilo:")
    a = int(input("Número 1: "))
    b = int(input("Número 2: "))
    args = (a, b)  # Crear los argumentos como tupla
    
    # Crear el hilo
    thread_id = 1
    thread = threading.Thread(target=mythread, args=(args, thread_id))
    print(f"\n[Principal] Creando el hilo {thread_id}...")
    
    # Iniciar el hilo
    thread.start()
    print(f"[Principal] Hilo {thread_id} iniciado.\n")
    
    # Esperar a que el hilo termine
    thread.join()
    print("[Principal] Hilo finalizado. ¡Todo listo!\n")
Descripción del flujo:
El programa solicita al usuario dos números.
Se crea un hilo con threading.Thread, indicando:
target=mythread: Función que ejecutará el hilo.
args=(args, thread_id): Argumentos para la función del hilo.
Se inicia el hilo con thread.start().
El hilo principal espera a que el secundario termine usando thread.join().
Al finalizar, imprime un mensaje indicando que el programa ha terminado.
Ejemplo de Ejecución
=== Programa de Ejemplo con Hilos y Barra de Progreso ===
Ingrese dos números para el hilo:
Número 1: 10
Número 2: 20

[Principal] Creando el hilo 1...
[Principal] Hilo 1 iniciado.

[Hilo 1] Iniciando...
[Hilo 1] Procesando: ██████████| 10/10 [00:05<00:00]

[Hilo 1] Valores recibidos: 10 20
[Hilo 1] Finalizando...

[Principal] Hilo finalizado. ¡Todo listo!

## Ejecucion

Coloque las capturas de pantalla donde se evidencia el correcto funcionamiento de su código. 


## Referencias

1. **Hilos en Python**  
   [Cómo usar hilos o threads en Python - Código Pitón](https://www.codigopiton.com/como-usar-hilos-o-threads-en-python/)

2. **Libros sobre Sistemas Operativos y Concurrencia**  
   "Operating Systems: Three Easy Pieces" por Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau.  
   [Sitio oficial](http://pages.cs.wisc.edu/~remzi/OSTEP/)
