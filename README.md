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
| grow run \[NAME.ron] | Run the seeder. If no file is specified, it will run all seeders in alphabetical order.     |

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
    // TABLE_NAME: DATA[],
    User: [
        {
            "role": "Admin",
            "email": "admin@example.com",
            "password": "hashed_password_admin",
            "created_at": "2024-12-22 12:00:00",
            "updated_at": "2024-12-22 12:00:00"
        },
    ]

    // Repeated data
    // TABLE_NAME(REPEATED_TIMES): DATA,
    User(4): [
        {
            "role": "client",
            "email": "{fake(EMAIL_PT_BR)}",
            // Templating have `i` to know the iteration
            "password": "hashed_password_admin{i}",
            "created_at": "2024-12-22 12:00:{mul_u32(i, 10)}",
            "updated_at": "2024-12-22 12:00:{mul_u32(i, 20)}"
        },
    ]
}
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

[Read CONTRIBUTING.md](./CONTRIBUTING.md)

## Features

- [ ] Create a library to run seeder in the code and not with CLI
- [x] Add cargo features to CLI.
- [x] Add `fake` in column value to create fake data.

Example for `fake` feature:

```ron
{
    // Generate 20 fake users
    User(20): [
        {
            "role": "CLIENT",
            "email": "{fake(EMAIL)}",
            "password": "{fake(PASSWORD)}",
            // Multi-language support
            "first_name": "{fake(FIRST_NAME_FR_FR)}",
        },
    ]
}
```

[see more](./FAKE-VARIANTS.md)

## License

This project is licensed under the [MIT License](LICENSE).
