# CNCF Landscape

## Used directly (circled green)

- Postgres - used in to-do app and log apps throughout the course
- NATS - used in to-do app in part 4
- Google Kubernetes Engine - used in part 3, focus point of the part
- Prometheus - used from part 2 onward for monitoring and in part 4 for deployment strategies
- Grafana loki - used from part 2 onward for viewing log output from pods
- Knative - used in part 5 for deploying ping-pong as a serverless app
- Linkerd - used in part 5 for setting up a service mesh for to-do app
- Helm - used from part 2 to deploy, among other things, Prometheus, but also later things like NATS and CRDs for various things
- Flux - used in part 4 for setting up GitOps workflow for to-do app
- Flagger - used in part 5 for setting up canary releases with Linkerd
- Google Container Registry - used most extensively in part 3 when building images for use with GKE
- Docker (and Docker Hub) - used for building images throughout the course and deploying images before moving to GCR
- Traefik - used as an ingress from part 2 onwards
- Kubernetes - that's what this course was all about
- Github Actions - CI/CD system utilized in part 3 and part 4
- Contour - used during part 5 for Knative ingress
- Podman - used throughout the course when needing to build images locally (I prefer Podman over Docker)

## Used indirectly (circled yellow)

- k3s - utilized by k3d as the Kubernetes implementation
- Grafana - required by Grafana loki, but not explored directly in detail during the course
- containerd - used by Docker as a daemon for executing containers
- runc - used by Podman when building images and testing containers during the course on my Fedora machine
- Flannel - used by k3d, no clear idea how it works
- etcd - used by GKE
- Istio - used by Knative
- OpenAPI Initiative - in part 5 defining the custom resource definitions involved creating an OpenAPI schema 

## Used outside of the course (circled blue)

- Redis - used at work
- MySQL - used with an open source project
- MariaDB - used with a personal project
- MongoDB - used during Full Stack course
- Kaniko - used at work
- Argo - used at work
- GitLab CI - used at work and have used for personal projects
- Travis CI - used during various university courses
- MicroK8s - used at work
- AWS - used at work
- DigitalOcean - used for personal projects for a while, have not used their Kubernetes offering
- kind - tested briefly outside of work and courses
- minikube - similarly tested outside of work and courses
- Harbor - used at work
- Pulumi - investigated at work, no significant usage
- nginx - used personally and professionally
- 
