variable "ssh_key_id" {
  type = string
}
variable "project_name" {
  type = string
}
variable "droplet_name" {
  type = string
}
variable "droplet_size" {
  type = string
  default = "s-1vcpu-2gb"
}
variable "droplet_region" {
  type = string
  default = "sgp1"
}
