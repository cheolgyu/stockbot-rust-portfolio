# key

1. "~/.ssh/example.pem"

```
chmod 400 "~/.ssh/example.pem"

```

2. 접속

```
ssh -i "~/.ssh/example.pem" ubuntu@ec2-example.example.invalid


```

2. 파일업로드 compose

```
 scp -i "~/.ssh/example.pem" ./aws/docker-compose-db.yml ubuntu@ec2-example.example.invalid
:/home/ubuntu

```

3. 파일업로드 dump

```
 scp -i "~/.ssh/example.pem" ./stockbot-dev-rust_v3.6.1.dump ubuntu@ec2-example.example.invalid
:/home/ubuntu

```

4.  pgadmin 연결

    1. 보안그룹 5432 내 ip 만 허용
    2. 자동 할당된 IP 주소 [퍼블릭 IP]

5.  backup dev

    ## stockbot-dev-rust_v3.6.1.sql

    ```
    docker exec -t stockbot_db_dev pg_dump -U postgres  stockbot-dev > stockbot-dev-rust_v3.6.1.dump
    ```

6.  restore op
    - scp 업로드 실패로 pgadmin 이용



# 용량 부족 
1. 8gb -> 15g
2. 
```
sudo growpart /dev/nvme0n1p1 1
sudo mount -o size=10M,rw,nodev,nosuid -t tmpfs tmpfs /tmp


sudo resize2fs /dev/nvme0n1p1
df -h 
sudo umount /tmp

$ sudo growpart /dev/nvme0n1p1 1
WARN: unknown label 
failed [sfd_dump:1] sfdisk --unit=S --dump /dev/nvme0n1p1
sfdisk: /dev/nvme0n1p1: does not contain a recognized partition table
FAILED: failed to dump sfdisk info for /dev/nvme0n1p1

# https://mfdo.tistory.com/97
sudo growpart /dev/nvme0n1 1
sudo resize2fs /dev/nvme0n1p1
df -h 

    SELECT PG_SIZE_PRETTY(PG_DATABASE_SIZE('stockbot-dev')); 11G  15G-> 22G
    6시간 후 수정하래 ;;
     바로수정안됨. 
     그냥 새로 만듬.

```