# Semaphores

> **Original**: Los siguientes codigos son tomados del directiorio [threads-sema](https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-sema) del [repositorio](https://github.com/remzi-arpacidusseau/ostep-code/tree/master) de ejemplos del libro de Remzi.

Los ejemplos de codigo del capitulo **Semaphores** ([threads-sema.pdf](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-sema.pdf)) son mostrados a continuación. Compile los códigos mediante el comando `make`; luego, corra el ejecutable resultante para ver como funciona.

## Fork/Join

El ejemplo `join.c` muestra la implementación de un problema **`fork/join`** (i.e., esperar por un hijo) udando semaforos 

Ejecute el comando `make` para compilar el codigo; luego llame al ejecutable `join` para ver como funciona:

```sh
prompt> make
prompt> ./join
```

## Binary Semaphores (Locks)

El ejemplo `binary.c` como se implementa **`locks`** (semaforos binarios) mediante el uso de semaforos.  

Ejecute el comando `make` para compilar el codigo; luego llame al ejecutable `binary` para ver como funciona:

```sh
prompt> make
prompt> ./binary
```

## Producer/Consumer

El código con la solución correcta del problema del **`producer/consumer`** se implementa en el archivo `producer_consumer.c`.

Ejecute el comando `make` para compilarlo y luego llame el ejecutable `producer_consumer` para probarlo. El programa la entrada de diferentes argumentos:
- Tamaño del buffer entre productor y consumidor.
- Cantidad de veces que el productor deberia poner algo en el buffer (produce).
- Numero de hilos consumidores.

```sh
prompt> make
prompt> ./producer_consumer 1 1000 1
```

La salida deberia imprimir cada item producido una vez mostrando cual consumidor consume el item (de los previamente producidos) fue el obtenido.

# Reader/Writer Locks

El codido se implementa en `rwlock.c`. Use el comando `make` para compilar y ejecute el archivo ejecutable resultante por medio de `rwlock`.

## Dining Philosophers

El problema de la cena de filosofos se encuentra implementado de diferentes maneras:
- `dining_philosophers_deadlock.c`: Codigo que se bloquea.
- `dining_philosophers_deadlock_print.c`: Codigo que se bloquea, pero que muestra la salida en pantalla para para propositos didacticos y de depuración.
- `dining_philosophers_no_deadlock.c`: Codigo que no se bloquea.
- `dining_philosophers_no_deadlock_print.c`: Codigo que no se bloquea, pero que muestra la salida en pantalla para para propositos didacticos y de depuración.

Ejecute el comando  `make` obtener todos los ejecutables a partir del `Makefile`.

## Zemaphores

Código en `zemaphore.c`. Esto es sólo una pequeña prueba del Zemaphore con el problema de **`fork/join`**.

# Throttle

Código **bonus** que muestra cómo los semáforos se pueden estrangular al ejecutar  a la vez en un determinado fragmento de código. Código en `throttle.c`. 