provider "aws" {
  region = "${var.region}"
}

terraform {
  backend "s3" {
    bucket         = "deko-terraform-state"
    key            = "decisioning.tfstate"
    region         = "eu-west-2"
    encrypt        = true
    dynamodb_table = "terraform-state-lock"
  }
}