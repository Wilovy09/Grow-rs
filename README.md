<div align="center">
<img src="https://github.com/user-attachments/assets/a4f0872c-794d-4a9b-a6e1-497addc59a7d" />
<a target="_blank" href="https://crates.io/crates/grow-rs">
<img alt="crates.io" src="https://img.shields.io/crates/v/grow-rs.svg?style=for-the-badge&color=28153f&logo=rust" height="20">
</a>
</div>

Grow Seeder CLI is a command line tool written in Rust to manage database seeders. It allows to generate, list and run seeders defined in RON format for LibSQL, PostgreSQL, MySQL and SQLite compatible databases. Automatically detects the database type through the `DATABASE_URL` environment variable.

## Requirements

- **Environment variables**:
  - `DATABASE_URL`: Database connection URL.
  - `TURSO_AUTH_TOKEN` (only for LibSQL/Turso databases).

## Installation

```bash
cargo install grow-rs
# or
cargo install --git https://github.com/Wilovy09/Grow-rs
```

## Commands

| Commands             | Functions                                                                                   |
| -------------------- | ------------------------------------------------------------------------------------------- |
| grow init            | Creates a `seeders/` folder in the current directory to store seeders.                      |
| grow new \<NAME>     | Creates a new `.ron` file inside the `seeders/` folder. The file name will be `<NAME>.ron`. |
| grow list            | Displays a list of all available seeders in the `seeders/` folder.                          |
| grow run             | Interactive mode: shows a multi-select list of all available seeders to run.                |
| grow run \<NAME>     | Run a specific seeder (`.ron` extension is optional). Example: `grow run roles`             |

## Cargo features

| Feature   | Description                                            |
| --------- | ------------------------------------------------------ |
| `default` | Install `libsql`, `sqlx databases` and `fake` support. |
| `fake`    | Enable `fake` support                                  |
| `libsql`  | Install only `libsql` support.                         |
| `sqlx`    | Install only `sqlx databases` support.                 |

## Seeder Example

A seeder file in `.ron` format could have the following content:

> [!NOTE]
> If the ID is generated automatically by the db, it is not necessary to enter it in the seeder.

```ron
{
	// Static data
	// products: DATA[(OBJECT)],
	User: [
	    (
		    column_name: "value",
	    )
	],

	// Schema qualified tables
	"schema.table": [
	    (
			column_name: "value",
		)
	],

	// Repeated data
	// products(REPEATED_TIMES): {DATA},
	User(4): {
		"column_name": "hashed_password_admin{i}",
	},

	// Repeated data with schemas
	// ("schema.table", REPEATED_TIMES): {DATA},
	("catalogs.products", 5): {
		"name": "{fake(WORD)}",
		"description": "{fake(WORD)}",
		"price_cents": 10000,
		// "price_cents": "{fake(DIGIT)}", // THIS FAKE FEATURE IS NOT SUPPORTED
		"currency": "mxn",
	},
}
```

## `.env` file

### Configuration of `DATABASE_URL` for Different Databases

1. **PostgresSQL, MySQL, SQLite (SQLx Support)**:
   ```env
   DATABASE_URL=sqlite://DB_NAME
   ```

   - This line configures the database URL for SQLite. The format sqlite://DB_NAME indicates that SQLite is being used, and DB_NAME is the name of the database.

2. **libsql (TURSO)**
    ```env
    DATABASE_URL=libsql://DB_NAME.turso.io
    TURSO_AUTH_TOKEN=eyJhbGciWJhNGRjIn0.igV6yDaKTuDKM7_5J-UWGOftULBg
    ```

    - Here, `DATABASE_URL` is configured to use libsql, which is a SQLite-compatible database with additional features like cloud synchronization. DB_NAME.turso.io is the URL of the database on Turso.
    - `TURSO_AUTH_TOKEN` is an authentication token required to access the database on Turso.

3. **Dry-run (Simulation)**
    ```env
    DATABASE_URL=mock://DB_NAME
    ```

    - This configuration is used to perform a simulation or "dry-run." Instead of connecting to a real database, a mock database is used to test commands without affecting real data.

### Considerations for Windows: Using `/` Instead of `\`

On operating systems like Windows, paths (file directories) are traditionally written using the backslash (\). However, in the context of software development, especially when working with cross-platform tools and languages, it is common to use the forward slash (/) instead of the backslash.

- **Reason**: The forward slash (/) is the standard on Unix systems (like Linux and macOS) and is widely supported on Windows. Using / ensures that the code is compatible across different operating systems.
- **Example**:
    - Instead of writing:
    ```env
    DATABASE_URL=sqlite://C:\Users\User\Documents\DB_NAME
    ```
    - You should write:
    ```env
    DATABASE_URL=sqlite://C:/Users/User/Documents/DB_NAME
    ```

This approach avoids compatibility issues and errors when moving code between different operating systems.

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

[Read CONTRIBUTING.md](./CONTRIBUTING.md)

## Features

- [ ] Create a library to run seeder in the code and not with CLI
- [x] Add cargo features to CLI.
- [x] Add `fake` in column value to create fake data.

Example for `fake` feature:

```ron
{
    User(20): {
            "role": "client",
            "email": "{fake(FREE_EMAIL)}",
            // Templating have `i` to know the iteration
            "password": "hashed_password_admin{i}",
            "created_at": "2024-12-22 12:00:{mul_u32(i, 10)}",
            "updated_at": "2024-12-22 12:00:{mul_u32(i, 20)}"
    },
}
```

[see more](./FAKE-VARIANTS.md)

## License

This project is licensed under the [MIT License](LICENSE).
