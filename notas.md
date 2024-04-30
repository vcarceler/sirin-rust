# Parámetros

Se puede compilar y ejecutar la aplicación indicando los parámetros después de `--`

~~~
cargo run -- 0.0.0.0 10000 hola
~~~

# Mensage de actix-web indicando que se deben activar los logs

Tener activado el log en la aplicación resulta muy útil. Además, si no lo tenemos
activo nos podemos encontrar el siguiente mensaje de actix-web en el navegador.

~~~
Requested application data is not configured correctly. View/enable debug logs for more details.
~~~

Añadir env_logger

~~~
cargo add env_logger
~~~

Añadir a la función main

~~~
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
~~~