# Platform comparison: Rancher vs OpenShift

- OpenShift builds on and integrates tightly with existing Red Hat ecosystem and tools
- Forrester report claims Red Hat has both a stronger current offering and strategy than Rancher
- Rancher's deployment plans focus on self-managed deployments of k3s on buyer-operated resources, OpenShift offerings come in self-managed and Red Hat managed variants
- Rancher ecosystem focuses on Docker, whereas the Red Hat ecosystem leverages Podman, which provides autonomy from Docker's management decisions and a better developer experience due to daemonless non-root containers
- OpenShift puts more focus on scalability. Default Rancher k3s deployments utilize SQLite as the database, which limits horizontal scalability and scalability is hardly addressed in Rancher documentation
