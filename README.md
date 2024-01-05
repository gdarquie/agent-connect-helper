# Rust Grist

Rust grist is a rust client that generates a Grist table from json data.

For now, you need to use a preexisting grist table with colums: uid, name and fqdn. It is a very specific usage, you probably won't need this.

| uid | name | fqdn |
| --- | ---- | ---- |
| str | str  | str  |


## Install

```
cp .env.dist .env
```

Complete with proper information.


## Migrate data with a script

This command will upload a script on local docker agent connect and then execute the script to export all fqdn, idp name and uid in a file iin data folder.

It is mandatory to launch agent connect docker before to run this command.

```
make extract-mongo-fqdn

```

## Create grist (is working but very raw)

After generating one or many data files, this command upload the data on [grist](https://www.getgrist.com/product/).

To run it, simply use this command

```
make run

```

## Todo

- dynamically create a new grist table for each file in data
- create a new table with a name corresponding of the data file name
