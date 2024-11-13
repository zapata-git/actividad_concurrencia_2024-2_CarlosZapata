# Condition Variables

> **Original**: Los siguientes codigos son tomados del directiorio [threads-cv](https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-cv) del [repositorio](https://github.com/remzi-arpacidusseau/ostep-code/tree/master) de ejemplos del libro de Remzi.

Los ejemplos de codigo del capitulo **Condition Variables** ([threads-cv.pdf](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-cv.pdf)) son mostrados a continuación. Compile los códigos mediante el comando `make`; luego, corra el ejecutable resultante para ver como funciona.

## Casos de uso de la CV

Mediante el uso de variables de condición, se abordan dos tipos de problemas:
* Problema Fork/Join 
* Problema Producer/Consumer 

A continuación se agrupan los diferentes ejemplos asociados a cada problema.

### Problema Fork/Join 

Los ejemplos relacionados con este problema se encuentran en el directorio [fork-join](./fork-join/)

### Problema Producer/Consumer 

Los ejemplos relacionados con este problema se encuentran en el directorio [producer-consumer](./producer-consumer/)

## Referencias

* https://ycpcs.github.io/cs365-spring2017/lectures/lecture09.html
* https://www.ibm.com/docs/en/aix/7.2?topic=programming-using-condition-variables
* https://docs.oracle.com/cd/E19455-01/806-5257/6je9h032r/index.html
* https://www.sysnet.ucsd.edu/~voelker/class/cse120/condvars.html
* https://cseweb.ucsd.edu/classes/sp16/cse120-a/index.html







