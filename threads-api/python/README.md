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
