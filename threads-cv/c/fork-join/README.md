# Fork/Join Problem

## Ejemplos

La siguiente lista de ejemplos esta relacionada con este problema

- [`join_spin.c`](join_spin.c): Solución que funciona pero que gasta CPU.
- [`join_no_lock.c`](join_no_lock.c): Que sucede cuano no se coloca un `lock` de la variable que indica el cambio de estado (`done`).
- [`join_no_state_var.c`](join_no_state_var.c): Que pasa cuando no se usa una variable que indica el evento de cambio de estado (`done`)
- [`join.c`](join.c): Solución que trabaja sin problema.
- [`join_modular.c`](join_modular.c): Versión modularizada de la solución que trabaja sin problema.

## Compilación

Ejecute el comando `make` para generar cada uno de los ejecutables y corralos para ver como trabajan. nserte llamadas `sleep()` de distintas duraciones para controlar el tiempo y mire que pasa.

```sh
prompt> make
```

## Ejecución

A continuación se muestra como correr el ejecutable `join_no_state_var`

```sh
prompt> ./join_no_state_var
```

## Salida

La salida del ejecutable anterior (`join_no_state_var`), se muestra a continuación:

```
parent: begin
child: begin
child: signal
parent: wait to be signalled...
^C
```

Notese que de acuerdo a la salida anterior, que hay un caracter `^C` lo cual implica que para este caso fue necesario terminar el programa usando `CTRL + C` por que se bloqueo. ¿A que cree que se deba esto?