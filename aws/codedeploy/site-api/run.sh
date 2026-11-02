#  tail -1000 /opt/codedeploy-agent/deployment-root/deployment-logs/codedeploy-agent-deployments.log
CONTAINER=site-api
IMG_REGISTRY=000000000000.dkr.ecr.ap-northeast-2.amazonaws.com
IMG_REPOSITORY=stockbot/site-api
IMG_TAG=latest
IMAGE=${IMG_REGISTRY}/${IMG_REPOSITORY}:${IMG_TAG}
#
aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin ${IMG_REGISTRY}/${IMG_REPOSITORY}
#docker image prune -a -f 
docker pull ${IMAGE}
docker stop ${CONTAINER} || true && docker rm ${CONTAINER} || true
# ipv4
# echo docker run -d --network ec1 --name ${CONTAINER}   -p 80:80 ${IMAGE}
# docker run -d --network ec1 --name ${CONTAINER}  -p 80:80 ${IMAGE}
# ipv6
# ipv6 network
docker network create --ipv6 --subnet 2001:db8::/64 ip6net
echo docker run -d --network ip6net --name ${CONTAINER}    -p 80:80 ${IMAGE}
docker run -d --network ip6net --name ${CONTAINER}    -p 80:80 ${IMAGE}
# docker run -d --network ip6net --name site-api    -p 80:80 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot/site-api:latest

# docker rmi -f $(docker images -f "dangling=true" -q)
docker ps -a
#  docker  rmi hello -f