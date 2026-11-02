#  db

1. ec2(t4g.micro-amd) cpu:2, 메모리: 1GiB ,ubuntu

   1. [도커 install](https://docs.docker.com/engine/install/ubuntu/)

      ```
      # Add Docker's official GPG key:
      sudo apt-get update
      sudo apt-get install ca-certificates curl
      sudo install -m 0755 -d /etc/apt/keyrings
      sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
      sudo chmod a+r /etc/apt/keyrings/docker.asc

      # Add the repository to Apt sources:
      echo \
      "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
      $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
      sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
      sudo apt-get update


      sudo apt-get -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

      ```

   2. [도커 컴포즈 설치](https://docs.docker.com/compose/install/linux/#install-using-the-repository)

      ```
         sudo apt-get update
         sudo apt-get install docker-compose-plugin
      ```

   3. aws/docker-compose-db.yml 업로드 or  vi docker-compose.yml
      ```
      scp -i "~/.ssh/example.pem" ./aws/docker-compose-db.yml ubuntu@ec2-example.example.invalid:/home/ubuntu
      ```
   4. 파일이름변경 `mv docker-compose-db.yml docker-compose.yml`

   5. 권한 
      ```
         sudo usermod -a -G docker $USER 
         newgrp docker
       ```
   6. docker compose up -d



2. 변경 t4g.nano (메모리 0.5) -> t4g.micro (cpu2 메모리 1)
   1. 사유: pg_restore 중 서버 죽음.

# 백업 && 복원
```
방법1. 성공
   pgadmin 툴이용
방법2. 실패- 메모리 부족으로
   pg_dump -U postgres -Fc -Z 0 --file=20230529.dump test3
   docker cp cf68a77c884f:20230529.dump .


   scp -i "~/.ssh/example.pem" ./20230529.dump ubuntu@ec2-example.example.invalid:/home/ubuntu/20230529.dump
   docker cp 20230529.dump b14e42be1ec7:/20230529.dump
   docker exec -it  b14e42be1ec7 /bin/sh

   pg_restore -U postgres -Fc -v -j 1 -d stockbot 20230529.dump
```
