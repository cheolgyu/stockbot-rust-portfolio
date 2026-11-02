#
CONTAINER=batch_exe
IMG_REGISTRY=000000000000.dkr.ecr.ap-northeast-2.amazonaws.com
IMG_REPOSITORY=stockbot/batch_exe
IMG_TAG=latest
IMAGE=${IMG_REGISTRY}/${IMG_REPOSITORY}:${IMG_TAG}
#
aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin ${IMG_REGISTRY}/${IMG_REPOSITORY}
# docker image prune -a -f 
docker pull ${IMAGE}
docker stop ${CONTAINER} || true && docker rm ${CONTAINER} || true
echo docker run --network ec1  -d --name ${CONTAINER}   -p 81:81 ${IMAGE}
docker run --network ec1  -d --name ${CONTAINER}  -p 81:81 ${IMAGE}
#docker rmi -f $(docker images -f "dangling=true" -q)
docker ps -a
#  docker  rmi hello -f