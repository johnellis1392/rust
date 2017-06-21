#!/bin/bash

# Run replicated hello world node instance
kubectl run hello-world --replicas=5 --labels="run=load-balancer-example" --image=gcr.io/google-samples/node-hello:1.0 --port=8080

# View new deployment
# kubectl get deployments hello-world

# View deployment configuration
# kubectl describe deployments hello-world

# View replicated pods
# kubectl get replicasets

# View replicaset configurations
# kubectl describe replicasets

# Create load-balancer service for load-balancing between replicated pods
kubectl expose deployment hello-world --type=LoadBalancer --name=my-service

# Get created load-balancer
# kubectl get services my-service

# Describe service
# kubectl describe services my-service

# Curl host through minikube
PORT=# Get randomly-assigned port from `kubectl get service my-service`
curl $(minikube ip):$PORT

# Reapply config file
kubectl apply -f $FILENAME

# Rollback deployment to earlier state
kubectl rollout undo deployment/nginx-deployment --to-revision=2

# Scale deployment
kubectl scale deployment nginx-deployment --replicas 10

# Set deployment to auto-scale
kubectl autoscale deployment nginx-deployment --min=10 --max=15 --cpu-percent=80

# Reset container image
kubectl set image deploy/nginx-deployment nginx=nginx:latest

# Pause deployment
kubectl rollout pause deployment/nginx-deployment

# Resume deployment
kubectl rollout resume deploy nginx-deployment

# Get rollout status
kubectl rollout status deploy/nginx-deployment
