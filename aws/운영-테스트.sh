docker start  site-api -i /bin/sh
docker run -it 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest /bin/sh

docker run -it --rm --env-file ./.env.prod -p 80:80  \
000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest \
/bin/sh

docker run -d --env-file ./.env.prod -p 80:80  000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest 
./site-api