# helm

-   <https://microk8s.io/>
-   <https://helm.sh/>

```shell
sudo snap install microk8s --classic --channel=1.20/stable
sudo snap refresh microk8s --channel=1.20/stable
sudo snap remove microk8s

microk8s start
microk8s status --wait-ready
microk8s enable dashboard dns storage istio helm3
microk8s stop

alias kubectl='microk8s kubectl'
alias istioctl='microk8s istioctl'
alias helm3='microk8s helm3'

# Enable istio injection
kubectl label namespace default istio-injection=enabled --overwrite
kubectl get namespace -L istio-injection

# Open services in browser
kubectl get all --all-namespaces
# Find cluster IP for service/kubernetes-dashboard
# Open https://$CLUSTER_IP
token=$(kubectl -n kube-system get secret | grep default-token | cut -d " " -f1)
kubectl -n kube-system describe secret $token
# Find cluster IP for service/kiali
# Open http://$CLUSTER_IP:20001
# Find cluster IP for service/prometheus
# Open http://$CLUSTER_IP:9090
# Find cluster IP for service/tracing
# Open http://$CLUSTER_IP:80

# Build and load images into microk8s
cargo make microk8s-images
microk8s ctr images ls

# Helm
helm3 show values ./examples/helm
helm3 install --debug --dry-run petshop ./examples/helm
helm3 install petshop ./examples/helm
helm3 ls
helm3 uninstall petshop

# TODO: Prometheus integration
# <https://github.com/prometheus-community/helm-charts/tree/main/charts/prometheus>
```
