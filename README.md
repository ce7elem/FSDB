# FSDB
FileSystem as a DataBase


## Idea

FSDB is a database system using the Linux filesystem as its backend.

In this system, a classic table is represented as a folder, columns as subfolders,
rows as files with the same name (representing the primary key) in each "column subfolder", and foreign keys as symbolic links.

### Specifications

Structure

- Database: A root directory containing multiple table directories.
- Table: A directory representing a table in a database.
- Row: A subdirectory within a table directory.
- Column: A set of files in a row/ folder.
- Column types: a `.types` file in the table/ folder ?
- Foreign Key: A symbolic link pointing to a file in another table's column subdirectory.

*Example Structure*

```
db_root/
├── books/
│   ├── 1/
│   │   ├── name
│   │   ├── author
│   │   └── ...
│   ├── 2/
│   │   ├── name
│   │   ├── author
│   │   └── ...
│   └── ...
└── user/
    ├── 1/
    │   ├── username
    │   ├── favourite_book -> ../books/2 (foreign key)
    │   └── ...
    ├── 2/
    │   ├── username
    │   ├── favourite_book -> /dev/null  (optionnal)
    │   └── ...
    └── ...
```

### Features

1. Core Features

- Database Creation and Deletion
- Table Management
- Column Management
- Row Management
- Foreign Key Management


2. Advanced Features

- Query Language
    Implement a simple query language to perform CRUD operations.
    Support for basic SQL-like queries (SELECT, INSERT, UPDATE, DELETE).
- Indexing
    Implement indexing to speed up search operations.
    Support for creating and managing indexes.
- Transactions
    Support for basic transaction management (BEGIN, COMMIT, ROLLBACK).
- Data Integrity
    Ensure data integrity with constraints (e.g., unique, not null).
    Support for referential integrity with foreign keys.
- Backup and Restore
    Implement backup and restore functionality for databases.
    Support for exporting and importing data.
- Concurrency Control
    Implement basic concurrency control mechanisms.
    Support for locking and unlocking resources.

## Contributing

Apes together strong. Send PRs.
