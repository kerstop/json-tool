# json-tool
A command line tool to work with json files, using zero copy wherever possible.

## Features

currently json-tool supports two operations

### Formating

```bash
> echo '{"hello":"world"}' | json-tool fmt
{
    "hello": "world"
}
```