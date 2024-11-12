# Threads Intro

> **Original**: Los siguientes codigos son tomados del directiorio [threads-intro](https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-intro) del [repositorio](https://github.com/remzi-arpacidusseau/ostep-code/tree/master) de ejemplos del libro de Remzi.

Para recordar los conceptos del API de POSIX para hilos puede consultar el **Interlude: Thread API** ([link](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-api.pdf)).

## Ejemplos 

Un par de programas sencillos que usan hilos.

**Archivos relvandes**:
- [`t0.c`](t0.c): Programa simple que ilustra el uso de hilos.
- [`t0.c`](t1.c): Programa que usa hilos para la implementación de un contador.

## Compilación

Ejecute el comando `make` para generar cada uno de los ejecutables a partir de cada archivo fuente. 

```sh
prompt> make
```

## Ejecución

A continuación se muestra un caso de ejecución del program `t0`:

```sh
prompt> ./t0
```

## Salida

La salida del ejecutable anterior (`t0`), se muestra a continuación:

```
main: begin
A
B
main: end
```


