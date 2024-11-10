#!/bin/bash

# eksctl create cluster --name zktls-cluster-dev --fargate
# aws eks update-kubeconfig --name zktls-cluster-dev
# aws ecr create-repository --repository-name s3n-zktls-registry

aws ecr describe-repositories \
      --repository-names s3n-zktls-registry