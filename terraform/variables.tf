variable "region" {
  description = "The AWS region to create resources in"
}

variable "vpc" {
  description = "The VPC to deploy to"
}

variable "environment" {
  description = "Name of the environment."
}

variable "db_multi_az_enabled" {
  description = "Whether RDS instances have multi-AZ."
}

variable "docker_image" {
  description = "The image to use for the ECS Task Definition."
}

output "docker_image" {
  value = "${var.docker_image}"
}

variable "service_autoscale_min" {
  default     = "1"
  description = "Minimum autoscale (number of tasks)"
}

variable "service_autoscale_max" {
  default     = "12"
  description = "Maximum autoscale (number of tasks)"
}

variable "autoscale_min" {
  default     = "1"
  description = "Minimum autoscale (number of EC2 instances)"
}

variable "autoscale_max" {
  default     = "2"
  description = "Maximum autoscale (number of EC2 instances)"
}

variable "instance_type" {
  default = "t2.small"
}

variable "key_name" {
  description = "The aws ssh key name."
}

variable "full_infrastructure" {
  description = "Create a full infrastructure? - \"true\" or \"false\""
}

locals {
  namespace = "${terraform.workspace}-decisioning"
}
