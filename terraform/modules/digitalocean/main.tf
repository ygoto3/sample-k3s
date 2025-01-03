resource "digitalocean_droplet" "droplet" {
  image    = "ubuntu-24-04-x64"
  name     = "${var.droplet_name}"
  region   = "${var.droplet_region}"
  size     = "${var.droplet_size}"
  ssh_keys = ["${var.ssh_key_id}"]
}

data "digitalocean_project" "project" {
  name      = "${var.project_name}"
}

resource "digitalocean_project_resources" "project" {
  project     = data.digitalocean_project.project.id
  resources   = ["${digitalocean_droplet.droplet.urn}"]
}

