resource "aws_ecs_cluster" "cluster" {
  name = "decisioning-${terraform.workspace}"
}

resource "aws_iam_instance_profile" "decisioning-ec2" {
  name = "decisioning-${terraform.workspace}"
  path = "/"
  role = "${aws_iam_role.decisioning-ec2.name}"
}

resource "aws_iam_role" "decisioning-ec2" {
  name = "decisioning-ec2-${terraform.workspace}"

  assume_role_policy = <<EOF
{
  "Version": "2008-10-17",
  "Statement": [

    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": [
          "ecs.amazonaws.com",
          "ec2.amazonaws.com"
        ]
      },
      "Effect": "Allow"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy" "decisioning-ec2" {
  name = "decisioning-ec2-${terraform.workspace}"
  role = "${aws_iam_role.decisioning-ec2.id}"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ecr:BatchCheckLayerAvailability",
        "ecr:BatchGetImage",
        "ecr:GetAuthorizationToken",
        "ecr:GetDownloadUrlForLayer",
        "ecs:DeregisterContainerInstance",
        "ecs:DiscoverPollEndpoint",
        "ecs:Poll",
        "ecs:RegisterContainerInstance",
        "ecs:StartTask",
        "ecs:StartTelemetrySession",
        "ecs:Submit*",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "*"
    }
  ]
}
EOF
}

resource "aws_autoscaling_group" "decisioning" {
  name                      = "decisioning-${terraform.workspace}"
  vpc_zone_identifier       = ["${data.aws_subnet_ids.private.ids}"]
  min_size                  = 1
  max_size                  = 1
  desired_capacity          = 1
  wait_for_capacity_timeout = 0
  health_check_type         = "EC2"
  launch_configuration      = "${aws_launch_configuration.decisioning.name}"

  tag {
    key                 = "Context"
    value               = "decisioning"
    propagate_at_launch = true
  }

  tag {
    key                 = "Name"
    value               = "decisioning-${terraform.workspace}"
    propagate_at_launch = true
  }

  tag {
    key                 = "cluster-name"
    value               = "decisioning-${terraform.workspace}"
    propagate_at_launch = true
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_launch_configuration" "decisioning" {
  name_prefix                 = "decisioning-${terraform.workspace}"
  image_id                    = "${lookup(var.amis, var.region)}"
  instance_type               = "${var.instance_type}"
  iam_instance_profile        = "${aws_iam_instance_profile.decisioning-ec2.name}"
  associate_public_ip_address = false
  key_name                    = "${var.ssh_key_name}"
  security_groups             = ["${aws_security_group.ephemeral_from_lb.id}"]

  user_data = <<EOF
#!/bin/bash
echo ECS_CLUSTER=${aws_ecs_cluster.cluster.name} >> /etc/ecs/ecs.config
EOF

  lifecycle {
    create_before_destroy = true
  }
}
