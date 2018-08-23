locals {
  container_name = "decisioning"
}

resource "aws_ecs_service" "decisioning" {
  name            = "decisioning-${terraform.workspace}"
  cluster         = "${aws_ecs_cluster.cluster.id}"
  task_definition = "${aws_ecs_task_definition.decisioning.arn}"
  desired_count   = 1

  depends_on = [
    "aws_iam_role_policy.decisioning-ec2",
    "aws_lb.decisioning",
  ]

  load_balancer {
    target_group_arn = "${aws_lb_target_group.decisioning.arn}"
    container_name   = "decisioning-${terraform.workspace}"
    container_port   = 80
  }
}

resource "aws_ecs_task_definition" "decisioning" {
  family        = "decisioning-${terraform.workspace}"
  task_role_arn = "${aws_iam_role.decisioning-ecs.arn}"

  container_definitions = <<EOF
[
  {
    "essential": true,
    "image": "${var.docker_image}",
    "name": "decisioning-${terraform.workspace}",
    "memory": 128,
    "portMappings": [
      {
        "containerPort": 80,
        "hostPort": 0
      }
    ],
    "environment": [
      { "name": "HOST_DB_PORT", "value": "${aws_db_instance.decisioning.port}" },
      { "name": "HOST_DB_HOST", "value": "${aws_db_instance.decisioning.address}" },
      { "name": "HOST_DB_NAME", "value": "${aws_db_instance.decisioning.name}" },
      { "name": "HOST_DB_USER", "value": "decisioning" },
      { "name": "HOST_DB_PASSWORD", "value": "decisioning" }
    ]
  }
]
EOF
}

resource "aws_iam_role" "decisioning-ecs" {
  name = "decisioning-ecs-${terraform.workspace}"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": [   
          "ecs.amazonaws.com",
          "ec2.amazonaws.com",
          "ecs-tasks.amazonaws.com"
        ]
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}
