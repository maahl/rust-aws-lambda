# Rust AWS Lambda

The goal of this repo is to showcase how to create and manage an AWS Lambda written in
Rust, using Terraform.

## Setup

```sh
# install the rust lambda helper
cargo install cargo-lambda

# compile and hot reload your lambda
cargo lambda watch

# invoke the lambda, pass the payload via cli
cargo lambda invoke --data-ascii '{ "httpMethod": "POST", "queryStringParameters": { "name": "world" } }'

# or via a file
cargo lambda invoke rust-aws-lambda --data-file examples/hello_payload.json
```


## Resources

* [The cargo-lambda documentation](https://www.cargo-lambda.info/)
