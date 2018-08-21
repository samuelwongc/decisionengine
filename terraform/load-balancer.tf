resource "aws_lb" "decisioning" {
  name      = "decisioning-${terraform.workspace}"
  internal  = false
  subnets   = ["${data.aws_subnet_ids.public.ids}"]
  security_groups = ["${aws_security_group.decisioning.id}"]
}

resource "aws_lb_target_group" "decisioning" {
  name     = "decisioning-${terraform.workspace}"
  port     = 80
  protocol = "HTTP"
  vpc_id   = "${var.vpc}"
  deregistration_delay = 35
}

resource "aws_autoscaling_attachment" "decisioning" {
 autoscaling_group_name = "${aws_autoscaling_group.decisioning.id}"
 alb_target_group_arn   = "${aws_lb_target_group.decisioning.arn}"
}

resource "aws_lb_listener" "http" {
  load_balancer_arn = "${aws_lb.decisioning.arn}"
  port              = "80"
  protocol          = "HTTP"

  default_action {
    target_group_arn = "${aws_lb_target_group.decisioning.arn}"
    type             = "forward"
  }
}

resource "aws_security_group" "decisioning" {
  name        = "decisioning-web-from-all-${terraform.workspace}"
  description = "Allow inbound web traffic"
  vpc_id      = "${var.vpc}"

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 3000
    to_port     = 3000
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 65535
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group" "ephemeral_from_lb" {
  name        = "decisioning-ephemeral-ingress-from-lb-${terraform.workspace}"
  description = "Allow inbound web traffic"
  vpc_id      = "${var.vpc}"

  ingress {
    from_port   = 0
    to_port     = 65535
    protocol    = "TCP"
    security_groups = ["${aws_security_group.decisioning.id}"]
  }

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "TCP"
    cidr_blocks = ["${data.aws_vpc.vpc.cidr_block}"]
  }

  egress {
    from_port   = 0
    to_port     = 65535
    protocol    = "TCP"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

