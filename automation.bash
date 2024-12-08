#!/bin/bash

# export PASS_DOCKER=$PASS_DOCKER
export TAG=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version')
echo $TAG
echo $PASS_DOCKER | docker login -u $USER_DOCKER --password-stdin
# docker build -t $USER_DOCKER/rust-workshop-jumpbox:$TAG .  
# docker push $USER_DOCKER/rust-workshop-jumpbox:$TAG

export path="kubernetes/"
cd $path && yq -i '.spec.template.spec.containers[0].image = "teewasza8989/rust-workshop-jumpbox:'"$TAG"'"' /rust-app-deployment.yaml
# kubectl config set-context colima
# kubectl apply -f kubernetes/rust-app-deployment.yaml