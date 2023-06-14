# Rust AWS Lambda

The goal of this repo is to showcase how to create and manage an AWS Lambda written in
Rust, using Terraform.

## Setup

```sh
# install the rust lambda helper
cargo install cargo-lambda
# set up terraform
terraform -chdir=infra init

# compile and hot reload your lambda
cargo lambda watch

# invoke the lambda, pass the payload via cli
cargo lambda invoke --data-ascii '{ "name": "world" }'

# or via a file
cargo lambda invoke rust-aws-lambda --data-file examples/hello_payload.json

# deploy the lambda
cargo lambda build --release --arm64 --output-format zip
terraform -chdir=infra apply
```


## Resources

* [The cargo-lambda documentation](https://www.cargo-lambda.info/)
