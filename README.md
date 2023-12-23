# Rust Grist

Rust grist is a rust client that generates a Grist table from json data.

For now, you need to use a preexisting grist table with colums: uid, name and fqdn. It is a very specific usage, you probably won't need this.

To run it, simply use this command

```
make run

```

In mongo, there is a script for exporting data in a format you can then import in grist.

## Todo

- dynamically create a new grist table for each file in data
- create a new table with a name corresponding of the data file name
