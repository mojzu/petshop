# helm

-   <https://minikube.sigs.k8s.io/docs/>
-   <https://helm.sh/>
-   <https://cheatsheetseries.owasp.org/cheatsheets/Microservices_security.html>
-   <https://cheatsheetseries.owasp.org/cheatsheets/Microservices_based_Security_Arch_Doc_Cheat_Sheet.html>
-   <https://cheatsheetseries.owasp.org/cheatsheets/Docker_Security_Cheat_Sheet.html>
-   <https://cheatsheetseries.owasp.org/cheatsheets/Kubernetes_Security_Cheat_Sheet.html>

```shell
minikube start
minikube stop
minikube delete

minikube dashboard
minikube addons list
minikube addons enable helm-tiller

alias kubectl='minikube kubectl --'
kubectl version
helm version

# Install Postgres and Prometheus using charts
helm install -f examples/helm/prometheus.yaml prometheus prometheus-community/prometheus
helm install -f examples/helm/postgresql.yaml database bitnami/postgresql

# Install petshop using local chart
cargo make minikube-images
minikube cache list

helm ls
helm show values ./examples/helm
helm install --debug --dry-run petshop ./examples/helm
helm install petshop ./examples/helm
helm uninstall petshop

# Follow helm notes on forwarding service port to host, then test
# using the client playground at http://localhost:1234

# TODO: Add helm tests?
# TODO: Ingress istio, mtls examples?
```
