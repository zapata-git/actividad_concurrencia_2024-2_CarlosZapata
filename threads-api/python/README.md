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

# Ejemplo de Threading en Python con Barra de Progreso

## Descripción

Este script de Python demuestra un ejemplo simple de multithreading utilizando el módulo `threading` y la barra de progreso `tqdm`. El programa permite a los usuarios ingresar dos números y ejecutar un hilo que procesa estos valores con un indicador de progreso visual.

## Características

- Crea un único hilo con entrada definida por el usuario
- Usa `tqdm` para mostrar una barra de progreso durante la ejecución del hilo
- Simula trabajo con intervalos de suspensión
- Imprime información específica del hilo

## Requisitos Previos

- Python 3.x
- Biblioteca `tqdm`

## Instalación

1. Clonar el repositorio:
   ```bash
   git clone https://github.com/tuusuario/ejemplo-threading.git
   ```

2. Instalar dependencias:
   ```bash
   pip install tqdm
   ```

## Uso

Ejecuta el script y sigue las instrucciones:
```bash
python ejemplo_threading.py
```

El programa:
- Pedirá ingresar dos números
- Creará un hilo
- Mostrará una barra de progreso
- Presentará detalles del procesamiento del hilo

## Ejemplo de Salida
```
=== Programa de Ejemplo con Hilos y Barra de Progreso ===
Ingrese dos números para el hilo:
Número 1: 10
Número 2: 20

[Principal] Creando el hilo 1...
[Principal] Hilo 1 iniciado.

[Hilo 1] Iniciando...
[Hilo 1] Procesando: 100%|██████████| 10/10 [00:05<00:00,  0.50s/tarea]

[Hilo 1] Valores recibidos: 10 20
[Hilo 1] Finalizando...

[Principal] Hilo finalizado. ¡Todo listo!
```

## Contribuciones

¡Las contribuciones son bienvenidas! No dudes en enviar un Pull Request.

## Licencia

Este proyecto es de código abierto. Considera agregar un archivo de licencia específico.


**Funcionamiento del Programa**

1. El programa solicita al usuario dos números de entrada.
1. Luego, crea un hilo que ejecuta la función mythread, pasando los dos números como argumentos y un identificador de hilo.
1. La función mythread realiza 10 iteraciones, simulando el procesamiento con una pausa de 0.5 segundos en cada paso, y muestra una barra de progreso.
1. Cuando el hilo termina, imprime los valores recibidos y finaliza el proceso.



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



**Conclusión**

Este programa es un ejemplo básico de cómo trabajar con hilos en Python y usar una barra de progreso para visualizar el avance de tareas en paralelo. Es útil para entender la gestión de múltiples hilos y la sincronización de tareas dentro de un programa.




## Referencias

1. **Hilos en Python**  
   [Cómo usar hilos o threads en Python - Código Pitón](https://www.codigopiton.com/como-usar-hilos-o-threads-en-python/)

2. **Libros sobre Sistemas Operativos y Concurrencia**  
   "Operating Systems: Three Easy Pieces" por Remzi H. Arpaci-Dusseau y Andrea C. Arpaci-Dusseau.  
   [Sitio oficial](http://pages.cs.wisc.edu/~remzi/OSTEP/)
