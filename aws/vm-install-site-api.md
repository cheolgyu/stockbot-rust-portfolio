1. ec2(t4g.nano-amd) cpu:2, 메모리: 1GiB ,ubuntu 로 수정 : code_deploy 배포실패 오류
2. ec2(t4g.nano-amd) cpu:2, 메모리: 0.5GiB ,ubuntu


## VM

1. ec2(t4g.nano-amd) cpu:2, 메모리: 1GiB ,ubuntu
   1. [도커 install](https://docs.docker.com/engine/install/ubuntu/)
   2. sudo usermod -a -G docker $USER
   3. sudo newgrp docker
2. [deploy agent 설치] (https://docs.aws.amazon.com/ko_kr/codedeploy/latest/userguide/codedeploy-agent-operations-install-ubuntu.html)
   ```
      wget https://aws-codedeploy-ap-northeast-2.s3.ap-northeast-2.amazonaws.com/latest/install
   ```
3. [ecr pull - aws-cli : amd64로 설치] (https://docs.aws.amazon.com/ko_kr/cli/latest/userguide/getting-started-install.html)
4. docker network create ec1
5. docker run
6. ec2 iam 역활 설정 ( code_depoly 용 s3 ecr codedeploy )
7. sudo service codedeploy-agent restart
   1. systemctl status codedeploy-agent
8. vm-db inbound 보안그룹에 site-api 보안그룹 추가

```
t4g 유형은 --platform linux/amd64  추가해야됨. ^^
WARNING: The requested image's platform (linux/amd64) does not match the detected host platform (linux/arm64/v8) and no specific platform was requested
```
