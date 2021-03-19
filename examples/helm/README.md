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

cargo make minikube-images
minikube cache list

helm ls
helm show values ./examples/helm
helm install --debug --dry-run petshop ./examples/helm
helm install petshop ./examples/helm
helm uninstall petshop

# Follow helm notes on forwarding service port to host, then the client
# playground endpoint can be changed to test it (port 8080)

# TODO: Add helm tests
```
