aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-batch
# ***batch***
```
docker build -f batch/dockerfile -t docker-test-batch:latest .
docker run -it   docker-test-batch:latest /bin/sh
24.7MB ->  13.5MB
```
# ~~***site-api use buildx compile***~~
```
docker buildx create --use --driver=docker-container --platform=linux/arm64

docker buildx build --platform=linux/arm64 \
    --push \
    -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest \
    -f site-api/dockerfile \
    --cache-to type=s3,region=ap-northeast-2,bucket=example-stockbot-cache,name=site-api \
    --cache-from type=s3,region=ap-northeast-2,bucket=example-stockbot-cache,name=site-api .





docker build -f site-api/dockerfile -t docker-test-site-api:latest .
docker run -it -p 8080:8080  docker-test-site-api:latest /bin/sh

 23.2MB -> 12MB

```****
## arch64
- https://www.docker.com/blog/cross-compiling-rust-code-for-multiple-architectures/
- https://stackoverflow.com/questions/65864484/compiling-a-containerized-rust-application-to-run-on-raspberry-pi-4-arm64-armv8

- exec ./hello: exec format error : https://stackoverflow.com/questions/73285601/docker-exec-usr-bin-sh-exec-format-error
```
FROM almalinux/arm64v8  as prod 이거와 
 
name: Set up Docker Buildx
uses: docker/setup-buildx-action@v2
with:
    platforms: linux/amd64/v8
    이거 일치시켜야됨.
```
```
https://velog.io/@infl_veggie/AWS-t4g-%EC%9D%B8%EC%8A%A4%ED%84%B4%EC%8A%A4-%EC%82%AC%EC%9A%A9%EA%B8%B0-with-%EB%8F%84%EC%BB%A4

```

sudo service codedeploy-agent restart

```
# 크로스컴파일 오류

https://github.com/nicolas-van/rust-cross-compile-example/blob/main/.github/workflows/rust.yml
https://www.docker.com/blog/cross-compiling-rust-code-for-multiple-architectures/

# hello
```
 docker build -f hello/dockerfile -t hello:lastest .
 docker run -it hello:lastest /bin/bash
 docker run hello:lastest 


docker buildx build --platform linux/arm64 -f hello/dockerfile  -t hello-buildx-p:latest -o type=docker .
docker run -it hello-buildx-p:latest /bin/sh
docker run hello-buildx-p:latest
docker images
```
# base 
```
 docker build -f aws/base.dockerfile -t base:latest .
 docker tag stockbot-base:latest 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest

 docker run -it base:latest /bin/sh
 docker run base:latest


docker buildx build --platform linux/arm64 -f aws/base.dockerfile  -t base-buildx:latest -o type=docker .
W
docker run base-buildx:latest
docker images

```

# base push
```
docker build -f aws/base.dockerfile -t base:latest .
docker tag base:latest 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest
docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest
```

#  ***site-api***

 ```
 export DOCKER_BUILDKIT=1
 time docker build  --progress=plain  -f site-api/dockerfile -t test-site-api:latest .
 docker run -it test-site-api /bin/sh
 docker run test-site-api 

docker tag test-site-api:latest 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest
docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest

aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api

docker buildx build --platform linux/arm64 -f site-api/dockerfile  -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest --push . 

ftp 
docker build  --progress=plain  -f site-api/dockerfile -t test-site-api:latest .
docker run -it test-site-api /bin/bash
docker cp 9a1ecca0f855:/stockbot/site-api/target/aarch64-unknown-linux-gnu/release/site-api ./site-api-test2
chmod +x site-api-test2

 ```
 #  ***site-api test-sccache ***
```
docker run -it --rm 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest /bin/sh
docker pull 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest

docker build  --progress=plain  -f site-api/test-sccache.dockerfile -t sccache-site-api:latest .


```

## base 이미지 만들어 ecr 에 넣고 쓰기
 ```
 docker build -f aws/base.dockerfile -t stockbot-base:latest .
 docker tag stockbot-base:latest 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest
 docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest
 ```
## case1. exe builder 를 alpine으로 만들고 builx 로 arm 으로 변환 해준후 이미지파일 ecr 로 넣고 운영에서 돌가는지 체크하기. ==>실패  : exec /bin/sh: exec format error 
==> [apk add libc6-compat 의존성 필요.] (https://stackoverflow.com/questions/66963068/docker-alpine-executable-binary-not-found-even-if-in-path)

## case2. exe builder 를 arm64v8/alpine으로 만들고 builx 로 arm 으로 변환 해준후 이미지파일 ecr 로 넣고 운영에서 돌가는지 체크하기. ==> 접속은됨. ,chmod +x site-api 해도  /bin/sh: site-api: not found
    ==> 해결 https://kldp.org/node/32553 
    dot(.) 한칸 띄고 ex-script 이래야 실행되더군요~~~~
    /bin/sh: site-api: line 1: ELF��@h�@8: not found
    /bin/sh: site-api: line 2: syntax error: unexpected ")"
    어떻게하나?
==> [apk add libc6-compat 의존성 필요.] (https://stackoverflow.com/questions/66963068/docker-alpine-executable-binary-not-found-even-if-in-path)

## case3. exe builder 를 arm64v8/alpine으로 만들고 builx 안쓰고                이미지파일 ecr 로 넣고 운영에서 돌가는지 체크하기. ==>
case2 와 마찬가지 증상 파일이 있는데 내가 실행을 못시키는건지 안되네;
==> [apk add libc6-compat 의존성 필요.] (https://stackoverflow.com/questions/66963068/docker-alpine-executable-binary-not-found-even-if-in-path)
**Error loading shared library libgcc_s.so.1: No such file or directory **
==> apk add libgcc
apk add libc6-compat 
apk add gcompat
apk add  libstdc++

https://gist.github.com/larzza/0f070a1b61c1d6a699653c9a792294be
https://github.com/gliderlabs/docker-alpine/issues/11

Error relocating /stockbot/site-api: __res_init: symbol not found
해결못하겠따
arm64v8/데비안 슬림 or arm64v8/우분투로 가야겠다


## case4. exe builder 를 arm64v8/ubuntu 으로 만들고 builx 안쓰고                이미지파일 ecr 로 넣고 운영에서 돌가는지 체크하기. ==>  ok 된다. => 용량크다. case 3에서 해결보자

```

docker buildx build --platform=linux/arm64    \
    --push --progress=plain \
    -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest \
    -f site-api/dockerfile \
    .
    <!-- --cache-to type=s3,region=ap-northeast-2,bucket=example-stockbot-cache,name=site-api \
    --cache-from type=s3,region=ap-northeast-2,bucket=example-stockbot-cache,name=site-api . -->
docker build  --progress=plain  -f site-api/dockerfile -t test-site-api:latest .    
docker tag test-site-api:latest 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest
docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api:latest
```

# batch, batch_exe openssl 오류 재현.
```
aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api
docker buildx create --use --driver=docker-container --platform=linux/arm64

docker buildx build \
    --push \
    --platform=linux/arm64 \
    --progress=plain  \
    -f batch/dockerfile \
    --build-arg="AWS_ACCESS_KEY_ID=<>" \
    --build-arg="AWS_SECRET_ACCESS_KEY=<>" \
    --build-arg="SCCACHE_BUCKET=example-stockbot-cache" \
    --build-arg="SCCACHE_S3_KEY_PREFIX=sccache/" \
    --build-arg="SCCACHE_REGION=ap-northeast-2" \
    -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-batch:latest .    
cargo tree -i openssl    --target all

추가 openssl = { version = "0.10", features = ["vendored"] }

```

# hello openssl 오류 재현.
```
https://stackoverflow.com/questions/54775076/how-to-cross-compile-a-rust-project-with-openssl
https://github.com/cross-rs/cross/issues/229#issuecomment-597898074

aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-site-api
docker buildx create --use --driver=docker-container --platform=linux/arm64

docker buildx build \
    --load \
    --platform=linux/arm64 \
    --progress=plain  \
    -f hello/dockerfile \
    --build-arg="AWS_ACCESS_KEY_ID=<>" \
    --build-arg="AWS_SECRET_ACCESS_KEY=<>" \
    --build-arg="SCCACHE_BUCKET=example-stockbot-cache" \
    --build-arg="SCCACHE_S3_KEY_PREFIX=sccache/" \
    --build-arg="SCCACHE_REGION=ap-northeast-2" \
    -t rm-hello:latest .    

docker run 09b01a22d418

```