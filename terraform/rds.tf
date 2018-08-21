resource "aws_db_instance" "decisioning" {
  count                      = "${var.full_infrastructure == "true" ? 1 : 0}"
  identifier                 = "${local.namespace}"
  skip_final_snapshot        = true
  allocated_storage          = 10
  storage_type               = "gp2"
  storage_encrypted          = true
  engine                     = "postgres"
  engine_version             = "9.6"
  instance_class             = "db.t2.small"
  name                       = "decisioning"
  username                   = "decisioning"
  password                   = "decisioning"
  vpc_security_group_ids     = ["${aws_security_group.decisioning_db.id}"]
  db_subnet_group_name       = "${aws_db_subnet_group.decisioning.name}"
  apply_immediately          = true
  multi_az                   = "${var.db_multi_az_enabled}"
  backup_retention_period    = 30
  auto_minor_version_upgrade = true
}

resource "aws_security_group" "decisioning_db" {
  count       = "${var.full_infrastructure == "true" ? 1 : 0}"
  name_prefix = "${local.namespace}-"
  description = "Allows inbound traffic from ECS Instances"
  vpc_id      = "${var.vpc}"

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["${data.aws_vpc.vpc.cidr_block}"]
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_db_subnet_group" "decisioning" {
  count      = "${var.full_infrastructure == "true" ? 1 : 0}"
  subnet_ids = ["${data.aws_subnet_ids.private.ids}"]
}

output "rds_endpoint" {
  value = "${join("", aws_db_instance.decisioning.*.endpoint)}"
}
