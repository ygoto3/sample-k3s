variable "do_token" {
  type = string
}
variable "ssh_key_name" {
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
