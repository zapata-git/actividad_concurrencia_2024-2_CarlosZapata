public class t0 {

    // Clase que implementa la lógica del hilo
    static class Hilos implements Runnable {
        private String mensaje;

        public Hilos(String mensaje) {
            this.mensaje = mensaje;
        }

        @Override
        public void run() {
            System.out.println(mensaje);
        }
    }

    public static void main(String[] args) {
        // Verifica que no haya argumentos
        if (args.length != 0) {
            System.err.println("usage: java t0");
            System.exit(1);
        }

        System.out.println("main: begin");

        // Crear los hilos
        Thread t1 = new Thread(new Hilos("A"));
        Thread t2 = new Thread(new Hilos("B"));

        // Iniciar los hilos
        t1.start();
        t2.start();

        // Esperar que los hilos terminen
        try {
            t1.join();
            t2.join();
        } catch (InterruptedException e) {
            System.err.println("Ejecución interrumpida: " + e.getMessage());
        }

        System.out.println("main: end");
    }
}

