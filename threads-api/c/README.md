
# Threads API

> **Original**: Los siguientes codigos son tomados del directiorio [r/threads-api](https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-api) del [repositorio](https://github.com/remzi-arpacidusseau/ostep-code/tree/master) de ejemplos del libro de Remzi.

Para comprender estos ejemplos, se recomienda que consulte el capitulo **Interlude: Thread API** ([link](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-api.pdf)) el cual explica el API POSIX para hilos y los ejemplos que se adjuntan para el caso.

## Ejemplos 

A continuación se muestran algunos ejemplos de como usar el API POSIX para hilos.

**Archivos relvandes**:
- [`thread_create.c`](thread_create.c): Programa simple que crea un hilo con al cual se le pasan unos argumentos (args).
- [`thread_create_with_return_args.c`](thread_create_with_return_args.c): Ejemplo en el cual se retornar valores desde el hilo hijo al hilo padre.
- [`thread_create_simple_args.c`](thread_create_simple_args.c): Ejemplo con paso de argumento y retorno. 

## Compilación

Ejecute el comando `make` para generar cada uno de los ejecutables a partir de cada archivo fuente. 

```sh
prompt> make
```

## Ejecución

En la etapa anterior, cada archivo `foo.c` genera un ejecutable llamado `foo`. En nuestro caso puntual, al ejecutar el comando `make` en el paso anterior, se generaran tres ejecutables `thread_create`, `thread_create_with_return_args` y  `thread_create_simple_args`. Si se desea ejercutar el primero de estos el comando a emplear es:

```sh
prompt> ./thread_create
```

## Salida

La salida del ejecutable anterior (`thread_create`), se muestra a continuación:

```
10 20
done
```


