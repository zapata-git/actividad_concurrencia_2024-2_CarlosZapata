# 🧵 Sincronización de Hilos con Semáforos en C

[![Licencia MIT](https://img.shields.io/badge/Licencia-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![C Version](https://img.shields.io/badge/C-11-blue.svg)](https://en.cppreference.com/w/c/11)
[![Plataformas](https://img.shields.io/badge/Plataformas-Linux%20%7C%20macOS-orange.svg)]()

## 🎯 Descripción General
Este proyecto demuestra la sincronización de hilos utilizando semáforos en C, con un ejemplo práctico de acceso seguro a una variable compartida.

## 📋 Estado de Implementación
- ✅ Sincronización con semáforos
- ✅ Incremento concurrente de contador
- ✅ Soporte multiplataforma (Linux/macOS)

Los códigos a reimplementar:

- [ ] `join.c`
- [x] `binary.c`
  - Este código demuestra la creación de un solo hilo binario que imprime el número binario de 0 a 31.
- [ ] `producer_consumer.c`
- [ ] `rwlock.c`
- [ ] `dining_philosophers_deadlock.c`
- [ ] `dining_philosophers_deadlock_print.c`
- [ ] `dining_philosophers_no_deadlock.c`
- [ ] `dining_philosophers_no_deadlock_print.c`
- [ ] `zemaphore.c`
- [ ] `throttle.c`

## 🛠️ Prerrequisitos

### Dependencias del Sistema
- Compilador GCC o Clang
- Biblioteca pthread
- Biblioteca semaphore.h (en Linux)

### Instalación de Dependencias

#### En Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install build-essential libc6-dev
```

#### En macOS
```bash
xcode-select --install
```

## 💻 Código Fuente

### Componentes Principales
- **Bibliotecas Utilizadas**
  - `<stdio.h>`: Entrada/salida estándar
  - `<stdlib.h>`: Funciones estándar de sistema
  - `<pthread.h>`: Gestión de hilos
  - `<unistd.h>`: Funciones del sistema Unix
  - `<semaphore.h>`: Semáforos (plataforma dependiente)

### Conceptos Clave
1. **Semáforos**
   - Mecanismo de sincronización
   - Control de acceso a sección crítica
   - Prevención de condiciones de carrera

2. **Variables Compartidas**
   - Uso de `volatile` para prevenir optimizaciones
   - Acceso sincronizado mediante semáforos

## 🚀 Compilación y Ejecución

### Compilación
```bash
# Para Linux
gcc -pthread thread_sync.c -o thread_sync

# Para macOS
gcc -pthread thread_sync.c -o thread_sync
```

### Ejecución
```bash
./thread_sync
```

## 🔍 Detalles Técnicos

### Flujo de Ejecución
1. Inicialización de semáforo
2. Creación de dos hilos
3. Cada hilo incrementa contador 10,000,000 veces
4. Sincronización mediante semáforo
5. Impresión de resultado final

### Consideraciones de Sincronización
- Semáforo binario (`mutex`)
- Sección crítica protegida
- Incremento seguro de variable compartida

## 📊 Características de Implementación

### 1. Multiplataforma
- Soporte para Linux
- Soporte para macOS con implementación personalizada
- Uso condicional de bibliotecas

### 2. Gestión de Concurrencia
- Control de acceso a recursos
- Prevención de condiciones de carrera
- Sincronización precisa

## 🧪 Análisis de Resultados

### Comportamiento Esperado
- Valor final del contador: 20,000,000
- Sin datos corruptos o condiciones de carrera

### Posibles Variaciones
- Número de hilos
- Cantidad de incrementos
- Estrategias de sincronización

## 📚 Conceptos Fundamentales

### Concurrencia
- Ejecución simultánea de múltiples hilos
- Gestión de recursos compartidos
- Sincronización de procesos

### Semáforos
- Mecanismo de señalización
- Control de acceso
- Prevención de conflictos

## 📖 Referencias Técnicas

### Bibliografía Recomendada
- 📘 "Modern Operating Systems" - Andrew S. Tanenbaum
- 📘 "Advanced Programming in the UNIX Environment" - W. Richard Stevens

### Documentación
- [POSIX Threads Programming](https://computing.llnl.gov/tutorials/pthreads/)
- [Semaphore Reference](https://linux.die.net/man/3/sem_init)

## 🤝 Contribución

1. Fork del repositorio
2. Crea rama para tu feature (`git checkout -b feature/mejora-concurrencia`)
3. Commit de cambios (`git commit -m 'Mejora: Sincronización de Hilos'`)
4. Push a la rama (`git push origin feature/mejora-concurrencia`)
5. Abre un Pull Request

## ✉️ Contacto

Carlos Zapata Arango
- GitHub: [@zapata-git](https://github.com/zapata-git)
- Repositorio: [actividad_concurrencia_2024-2_CarlosZapata](https://github.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata)

![Resultado final](https://raw.githubusercontent.com/zapata-git/actividad_concurrencia_2024-2_CarlosZapata/main/result.png)

## Ejemplos


---
© 2024 Carlos Zapata Arango. Todos los derechos reservados.
