#!/bin/bash
# This is a startup script for running a basic local Kubernetes
# server using Minikube
#
# Resources:
# Kubernetes cheat sheet: https://kubernetes.io/docs/user-guide/kubectl-cheatsheet/
# Overview of kubectl: https://kubernetes.io/docs/user-guide/kubectl-cheatsheet/

#################################################
# Kubernetes commmands

POD_NAME=hello-minikube;
POD_IMAGE=gcr.io/google_containers/echoserver:1.4;
PORT=8080;

# Source kubernetes commands
source <(kubectl completion bash);

# Start Minikube
minikube start;

# Create a new pod
kubectl run $POD_NAME --image=$POD_IMAGE --port=$PORT;

# Expose pod to the outside world
kubectl expose deployment $POD_NAME --type=NodePort;

# Retrieve the basic config information for the pod
kubectl get pods # Also: kubectl get pod

# Curl the pod by it's kubernetes-assigned url
curl $(minikube service $POD_NAME --url)

# Kill minikube service
minikube stop;

# Get current status of minikube
minikube status;

# SSH into Minikube vm; this is where the docker containers are run
minikube ssh;
#################################################



#################################################
# Setting up a redis container with port forwarding
# from the tutorial at:
# https://kubernetes.io/docs/tasks/access-application-cluster/port-forward-access-application-cluster/

# Create container from file
kubectl create -f ./redis-master.yml;

# Check pod was created
kubectl get pods;

# Verify that redis is running in the container
kubectl get pods redis-master --template='{{(index (index .spec.containers 0).ports 0).containerPort}}{{"\n"}}';

# Forward local port to container port
kubectl port-forward redis-master 6379:6379;

#################################################



#################################################
# Docker commands to remember

# List all containers (including stopped containers)
docker ps -a

# NOTE: either docker or docker-compose doesn't actually kill stopped
# containers when they exit; rather it keeps them around in the docker
# process list until you force-remove them.

# Force removal of all stopped containers
docker rm $(docker ps -a -q --filter status=exited)
#################################################
