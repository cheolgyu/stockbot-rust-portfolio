#!/bin/bash
# =========================================================
# 파일 위치: sudo /usr/local/stockbot-batch-start.sh
# 변경시 sudo systemctl daemon-reload
# sudo systemctl status stockbot-batch.service
# sudo systemctl restart stockbot-batch.service
# 로그
# journalctl -u stockbot-batch.service
# sudo docker logs --tail 100 -f  batch
##### ################
##########!/bin/bash user_data.sh
# echo "BATCH_TYPE=BATCH" > /root/user_data.txt
# echo "COUNTRY_TYPE=KR" >> /root/user_data.txt
# echo "RUN_TYPE=START" >> /root/user_data.txt
# echo "BATCH_TIMER_PLAN_ID=12" >> /root/user_data.txt
# =========================================================
f_user_data=/user_data.txt
f_env_batch_plan=/home/ubuntu/stockbot/.env.batch_plan

date >> /home/ubuntu/stockbot-batch-start.txt
# use-data 가져오기
rm $f_user_data 
rm $f_env_batch_plan 


sudo cp /var/lib/cloud/instances/i-00000000000000000/user-data.txt /user_data.txt
sudo mv /user_data.txt /user_data.sh
sudo chmod +x /user_data.sh
sudo /user_data.sh

echo "=================================="
sudo cat /root/user_data.txt 
echo "=================================="
sudo cat /root/user_data.txt > $f_env_batch_plan
echo "================================== "
sudo cat $f_env_batch_plan
echo "=================================="




cd /home/ubuntu/
#
CONTAINER=batch
IMG_REGISTRY=000000000000.dkr.ecr.ap-northeast-2.amazonaws.com
IMG_REPOSITORY=stockbot/batch
IMG_TAG=latest
IMAGE=${IMG_REGISTRY}/${IMG_REPOSITORY}:${IMG_TAG}
echo "IMAGE=$IMAGE"
#
#aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-batch:latest
aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin ${IMG_REGISTRY}/${IMG_REPOSITORY}

# docker image prune -a -f 
#docker pull --platform linux/amd64 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-batch:latest
docker pull ${IMAGE}

docker stop ${CONTAINER} || true && docker rm ${CONTAINER} || true
docker run -d --name ${CONTAINER} --env-file ${f_env_batch_plan}  -p 80:80 ${IMAGE}
# docker rmi -f $(docker images -f "dangling=true" -q)
docker ps -a
#  docker  rmi hello -f
# docker run --rm -it --name batch --env-file /home/ubuntu/stockbot/.env.batch_plan -p 80:80 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-batch:latest /bin/bash
# docker pull --platform linux/amd64 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-batch:latest