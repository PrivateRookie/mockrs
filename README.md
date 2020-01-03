# mockrs
mock restful json http server

mockrs use a json file as database, serve as http server.
You can get part of json file by doing http request

## Install & Usage

```bash
cargo install --git https://github.com/PrivateRookie/mockrs.git

mockrs --help

mockrs 0.1.0
PrivateRookie <996514515@qq.com>
a mock restful json http server

USAGE:
    mockrs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    gen      Generate fake data based on template
    help     Prints this message or the help of the given subcommand(s)
    serve    Run http json server
```

### run http server

```bash
mockrs serve --help
mockrs-serve 0.1.0
Run http json server

USAGE:
    mockrs serve [OPTIONS] <db-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --host <host>    Listen ip [env: MOCKRS_HOST=]  [default: 127.0.0.1]
        --port <port>    Listen port [env: MOCKRS_PORT=]  [default: 9000]

ARGS:
    <db-file>    Json file as database [env: MOCKRS_DB_FILE=]
```

db.json content:

```json
{
  "posts": [
    {
      "name": "a"
    },
    {
      "name": "b"
    },
    {
      "name": "d"
    }
  ]
}
```

run mockrs

```bash
mockrs db.json
```

#### do query

```bash
# get all content
curl http://localhost:9000/ | jq

{
  "posts": [
    {
      "name": "a"
    },
    {
      "name": "b"
    },
    {
      "name": "d"
    }
  ]
}

# get first post
curl http://localhost:9000/posts/2 | jq

{
  "name": "d"
}

# get firs post's name

curl http://localhost:9000/posts/2/name | jq

"d"
```

#### update entry 

```bash
# both "post" and "put" are OK
curl http://localhost:9000/posts/2 \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"name": "XD"}'

curl http://localhost:9000/posts/2 | jq

{
  "name":"XD"
}
```

**note** you can insert entry on top field

```bash
curl http://localhost:9000/users \
  -X POST \
  -H "Content-Type: application/json" \
  -d '[]'

curl http://localhost:9000/users | jq

[]
```

#### delete entry

```bash
curl http://localhost:9000/posts/2 -X DELETE

curl http://localhost:9000/posts | jq

[
  {
    "name": "a"
  },
  {
    "name": "d"
  }
]
```

#### flush data to a file

```bash
curl http://localhost:9000/_actions/flush \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"file": <path_to_file> }'
```

### generate fake data

Thanks to [jen](https://github.com/whitfin/jen), we can generate json file base on tera template.

Usage:

```bash
Generate fake data based on template

USAGE:
    mockrs gen [OPTIONS] <template>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --output <output>    Output json file

ARGS:
    <template>    Template file to generate json file
```

tera template sample

```jinja2
{
    "_id": "{{ objectId() }}",
    "eyeColor": "{{ random(values = ["blue", "brown", "green"]) }}",
    "name": "{{ name() }}",
    "company": "{{ company() }}",
    "email": "{{ email() }}",
    "friends": [
        {% for i in range(end=3) %}
        {
            "id": "{{ index() }}",
            "name": "{{ name() }}"
        }{% if i != 2 %},{% endif %}
        {% endfor %}
    ]
}
```

**Note: **, in array structure, remember to add "," to make output as valid json file, take a look at `{% if i != 2 %},{% endif %}`

output

```json
{
    "_id": "5e0f068d3331625d4044c77d",
    "eyeColor": "green",
    "name": "D'angelo Bayer",
    "company": "Quitzon LLC",
    "email": "candace_amet@hotmail.com",
    "friends": [
        
        {
            "id": "3",
            "name": "Wilburn Reichert"
        },
        
        {
            "id": "4",
            "name": "Ms. Jacinthe Farrell"
        },
        
        {
            "id": "5",
            "name": "Lorenz Kirlin"
        }
        
    ]
}
```

#### template helper

copy from [jen - README](https://github.com/whitfin/jen/blob/master/README.md).

| Helper                                 | Description                                           |
| -------------------------------------- | ----------------------------------------------------- |
| bool()                                 | Generates a random boolean value                      |
| city()                                 | Generates a random city name                          |
| company()                              | Generates a random company name                       |
| domain()                               | Generates a random domain name                        |
| email()                                | Generates a random email address                      |
| firstName()                            | Generates a random first name                         |
| float(start=f64::MIN, end=f64::MAX)    | Generates a random float value between two bounds     |
| index()                                | Retrieves the current index of the generated document |
| industry()                             | Generates a random industry type                      |
| integer(start=i64::MIN, end=i64::MAX)  | Generates a random integer value between two bounds   |
| lastName()                             | Generates a random last name                          |
| latitude()                             | Generates a random latitude location value            |
| longitude()                            | Generates a random longitude location value           |
| name()                                 | Generates a random full name                          |
| objectId()                             | Generates a random object identifier                  |
| paragraph()                            | Generates a random paragraph of Lorem Ipsum           |
| phone()                                | Generates a random phone number                       |
| postcode()                             | Generates a random postcode value                     |
| profession()                           | Generates a random job profession                     |
| random(values=["red","blue","yellow"]) | Retrieves a random value from the provided values     |
| sentence()                             | Generates a random sentence of Lorem Ipsum            |
| state()                                | Retrieves a random US state name                      |
| stateCode()                            | Retrieves a random US state code                      |
| street()                               | Generates a random street name                        |
| timestamp()                            | Generates a random timestamp value in seconds         |
| title()                                | Generates a random job title                          |
| userAgent()                            | Generates a random browser user agent                 |
| username()                             | Generates a random account username                   |
| uuid()                                 | Generates a v4 UUID                                   |
| word()                                 | Retrieves a random word of Lorem Ipsum                |
| zip()                                  | Generates a random US zip code                        |
