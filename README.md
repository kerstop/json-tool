# json-tool
A command line tool to work with json files, using zero copy wherever possible.

### Format an object

```bash
> echo '{"hello":"world"}' | json-tool
{
    "hello": "world"
}
```
### Retrieve a key

```bash
> echo '{"hello":"world"}' | json-tool get hello
"world"
```
### Set a key

```bash
> echo '{"hello":"world"}' | json-tool set hello '"jason"'
{
    "hello": "jason"
}
```