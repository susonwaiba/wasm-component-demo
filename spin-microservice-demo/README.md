# Spin Microservice Demo

This repository is sample implementation.

https://github.com/fermyon/spin


## Quick start

```bash
spin build
spin watch
```

```bash
spin build
spin up
```


## Example

```bash
curl -I http://127.0.0.1:3000
curl -I http://127.0.0.1:3000/404

curl http://127.0.0.1:3000/ts-adder
curl http://127.0.0.1:3000/ts-calculator
curl http://127.0.0.1:3000/ts-calculator/home/1
curl http://127.0.0.1:3000/ts-calculator/home/2

curl http://127.0.0.1:3000/rs-adder
curl http://127.0.0.1:3000/rs-calculator

curl http://127.0.0.1:3000/static/hello.txt
```

## Components

- ts-adder
- ts-calculator

