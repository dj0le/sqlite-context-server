# Zed SQLite Context Server

This extension provides a Model Context Server for SQLite, for use with the Zed AI assistant.

It adds a `/sqlite-schema` slash command to the Assistant Panel.

## Configuration

To use the extension, you will need to point the context server at a SQLite database by setting the `database_path` in your Zed `settings.json`:

```json
{
  "context_servers": {
    "sqlite-context-server": {
      "settings": {
        "database_path": "/path/to/your/database.sqlite"
      }
    }
  }
}
```

## Usage

- `/sqlite-schema <table-name>`: Retrieve the schema for the table with the given name.
- `/sqlite-schema all-tables`: Retrieve the schemas for all tables in the database.
