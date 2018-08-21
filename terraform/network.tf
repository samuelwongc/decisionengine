data "aws_caller_identity" "current" {}

data "aws_vpc" "vpc" {
  count = "${var.full_infrastructure == "true" ? 1 : 0}"
  id    = "${var.vpc}"
}

data "aws_subnet_ids" "private" {
  count  = "${var.full_infrastructure == "true" ? 1 : 0}"
  vpc_id = "${var.vpc}"

  tags {
    Tier = "private"
  }
}

data "aws_subnet_ids" "public" {
  count  = "${var.full_infrastructure == "true" ? 1 : 0}"
  vpc_id = "${var.vpc}"

  tags {
    Tier = "public"
  }
}
