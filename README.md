# Grow

Grow Seeder CLI is a command line tool written in Rust to manage database seeders. It allows to generate, list and run seeders defined in RON format for LibSQL, PostgreSQL, MySQL and SQLite compatible databases. Automatically detects the database type through the `DATABASE_URL` environment variable.

## Requirements

- **Rust**: Make sure you have Rust installed on your system.
- **Environment variables**:
  - `DATABASE_URL`: Database connection URL.
  - `TURSO_AUTH_TOKEN` (only for LibSQL/Turso databases).

## Installation

```bash
cargo install --git https://github.com/Wilovy09/grow 
```

## Commands

### grow init

Creates a `seeders/` folder in the current directory to store seeders.

```bash
grow init
```

### grow new <NAME>

Creates a new `.ron` file inside the `seeders/` folder. The file name will be `<NAME>.ron`.

```bash
grow new UserSeeder
```

### grow list

Displays a list of all available seeders in the `seeders/` folder.

```bash
grow list
```

### grow run [<NAME.ron>]

Run the seeders. If no file is specified, it will run all seeders in alphabetical order.

```bash
grow run User.ron
```

## Seeder Example

A seeder file in `.ron` format could have the following content:

```ron
(
  // Table name
    User: [
        {
          // Column name
            "role": "Administrator",
            // Column value
            "email": "admin@example.com",
            "password": "hashed_password_admin",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        },
        {
            "role": "Cliente",
            "email": "cliente1@example.com",
            "password": "hashed_password_cliente1",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        },
        {
            "role": "Cliente",
            "email": "cliente2@example.com",
            "password": "hashed_password_cliente2",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        }
     ]
)
```

## Database Compatibility

Grow Seeder CLI is compatible with:

> [!NOTE]
> Required on all `DATABASE_URL` in your `.env`

- [x] **LibSQL**: Requires the `TURSO_AUTH_TOKEN` variable.
- [x] **PostgreSQL**
- [x] **MySQL**
- [x] **SQLite**

The CLI automatically detects the database type via `DATABASE_URL` and handles the connection appropriately.

## Contributions

1. Make a fork of the repository.
2. Create a new branch for your functionality or bug fixes.
3. Send a pull request with the changes.

## License

This project is licensed under the [MIT License](LICENSE).

---

# Grow

Grow Seeder CLI es una herramienta de línea de comandos escrita en Rust para gestionar seeders en bases de datos. Permite generar, listar y ejecutar seeders definidos en formato RON para bases de datos compatibles con LibSQL, PostgreSQL, MySQL y SQLite. Detecta automáticamente el tipo de base de datos a través de la variable de entorno `DATABASE_URL`.

## Requisitos

- **Rust**: Asegúrate de tener Rust instalado en tu sistema.
- **Variables de entorno**:
  - `DATABASE_URL`: URL de conexión a la base de datos.
  - `TURSO_AUTH_TOKEN` (solo para bases de datos LibSQL/Turso).

## Instalación

```bash
cargo install --git https://github.com/Wilovy09/grow 
```

## Comandos

### grow init

Crea una carpeta `seeders/` en el directorio actual para almacenar los seeders.

```bash
grow init
```

### grow new <NOMBRE>

Crea un nuevo archivo `.ron` dentro de la carpeta `seeders/`. El nombre del archivo será `<NOMBRE>.ron`.

```bash
grow new UserSeeder
```

### grow list

Muestra una lista de todos los seeders disponibles en la carpeta `seeders/`.

```bash
grow list
```

### grow run [<NAME.ron>]

Ejecuta los seeders. Si no se especifica un archivo, ejecutará todos los seeders en orden alfabético.

```bash
grow run User.ron
```

## Ejemplo de Seeder

Un archivo de seeder en formato `.ron` podría tener el siguiente contenido:

```ron
(
  // Nombre de la tabla
    User: [
        {
          // Nombre de la columna
            "role": "Administrador",
            //       Valor de la columna
            "email": "admin@example.com",
            "password": "hashed_password_admin",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        },
        {
            "role": "Cliente",
            "email": "cliente1@example.com",
            "password": "hashed_password_cliente1",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        },
        {
            "role": "Cliente",
            "email": "cliente2@example.com",
            "password": "hashed_password_cliente2",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        }
    ]
)
```

## Compatibilidad con Bases de Datos

Grow Seeder CLI es compatible con:

> [!NOTE]
> Es necesario en todas `DATABASE_URL` en tu `.env`

- [x] **LibSQL**: Requiere la variable `TURSO_AUTH_TOKEN`.
- [x] **PostgreSQL**
- [x] **MySQL**
- [x] **SQLite**

El CLI detecta automáticamente el tipo de base de datos a través de `DATABASE_URL` y maneja la conexión de manera adecuada.

## Contribuciones

1. Haz un fork del repositorio.
2. Crea una nueva rama para tu funcionalidad o corrección de errores.
3. Envía un pull request con los cambios.

## Licencia

Este proyecto está licenciado bajo la [MIT License](LICENSE).
