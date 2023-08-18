# http-bellhop

http-bellhop 0.1.0
HTTP Bellhop CLI tool for API testing

USAGE:
app [OPTIONS]

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-d, --dir <dir>      location of dir with json files to run
-e, --env <env>      what env setup should be used
-f, --file <file>      location of json file to run

```
bellhop -e dev -d ./requests
```
```
bellhop --env dev --dir ./requests
```
```
bellhop --env dev --file ./requests/localhost/test.json
```