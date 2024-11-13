# Producer/Consumer Problem

## Ejemplos

La siguiente lista de ejemplos esta relacionada con este problema:
- [`pc_single_cv.c`](pc_single_cv.c): Que pasa cuando se usa una sola variable de condición.
- [`pc.c`](pc.c): Solución que trabaja

## Compilación

Ejecute el comando `make` para generar cada uno de los ejecutables y corralos para ver como trabajan. nserte llamadas `sleep()` de distintas duraciones para controlar el tiempo y mire que pasa.

```sh
prompt> make
```

## Ejecución

### Caso con una sola variable de condicion (`pc_single_cv`)

A continuación se muestra como correr el ejecutable `pc_single_cv`

```sh
prompt> ./pc_single_cv
```

La salida del ejecutable anterior (`join_no_state_var`) se muestra para diferentes escenarios:

* **Caso 1**: Paso sin argumentos.
  
  ```
  ./pc_single_cv
  usage: ./pc_single_cv <buffersize> <loops> <consumers>
  ```

* **Caso 2**: Pasando como argumentos un tamaño de buffer de 3, 1000 ciclos de inserción y dos consumidores.
  
  ```
  ./pc_single_cv 3 1000 2
  ```

  Al parecer, el codigo funciona bien en este caso pues no se bloquea.

* **Caso 3**: Aumentando el numero de consumidores (6 para este caso) hasta un caso de falla.
  
  ```
  ./pc_single_cv 3 1000 6
  ^C
  ```

  Para este ultimo caso, fue necesario usar `CTRL + C`, pues el ejecutable se bloqueo lo cual implica que hubo un problema.


### Caso que funciona (`pc`)

A continuación se muestra como correr el ejecutable `pc`

```sh
prompt> ./pc
```

La salida del ejecutable anterior (`join_no_state_var`) se muestra para diferentes escenarios:

* **Caso 1**: Paso sin argumentos.
  
  ```
  ./pc
  usage: ./pc <buffersize> <loops> <consumers>
  ```

* **Caso 2**: Pasando como argumentos un tamaño de buffer de 3, 1000 ciclos de inserción y dos consumidores.
  
  ```
  ./pc 3 1000 2
  ```

  Al parecer, el codigo funciona bien en este caso pues no se bloquea.

* **Caso 3**: Aumentando el numero de consumidores (6 para este caso) hasta un caso de falla.
  
  ```
  ./pc 3 1000 6
  ```

  Tambien funciona bien pues no hubo bloqueo.


