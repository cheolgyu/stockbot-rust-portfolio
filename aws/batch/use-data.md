site-api 에서 ec2 시작시킬때 use-data 에 실행해야될 batch 종류와 batch_timer_plan.id 값을 준다.
시작된 use-data가 실행되면서 .env.timer_plan 을 생성해줘야하는데 생성을 안하네?
aws consloe 사이트에서 ec2의 user-data를 확인하면 id 가 13. 그럼 현재 생성한 id는 18
aws consloe 사이트에서 ec2 중지됨 
-> 
슈거 batch 생성 id 19 및 site-api로그 는 
Killed
Killed
Killed
Killed
&& 
site-api.test-* log 는 특이사항 없음.
start.sh echo 로그 확인필요.
->


에러 ec2.site-api -> container.site-api 의 aws credentials 있지만 aws 명령어 실행시 결과값 Killed 출력. 값이 안바뀜.
aws ec2 modify-instance-attribute --instance-id i-00000000000000000 --region ap-northeast-2 --user-data Value=1

https://serverfault.com/questions/268288/most-long-running-commands-instantly-killed-on-amazon-ec2-ubuntu-10-04

aws 가 실행 되다가 안되가 되다가 안되다가 이런다.ㅋㅋㅋ 
site-api 컨테이너 메모리 키우기 cpu랑 해보자 . 항상 시도하면 실행되게.
됬을때는 aws console 사이트에서 ec2 사용자 데이터 바뀜. 하지만 script 가 실행이 안됬넴 .env.plan이 안말들어짐.

cd /var/lib/cloud/instances/i-00000000000000000
참고
https://docs.aws.amazon.com/ko_kr/AWSEC2/latest/UserGuide/user-data.html#user-data-shell-scripts
https://repost.aws/ko/knowledge-center/execute-user-data-ec2

# 결론
 user-data는 매번 실행이 안됨.    
[매번 실행하려면 cloud-boothook으로 실행해야함. ](https://cloudinit.readthedocs.io/en/latest/explanation/format.html#cloud-boothook)
- 방법
    1. cloud-boothook 사용.
    2. use-data + bootservice 만들기. use-data.txt 의 내용으로 script 를 만드는 서비스를 만든다. 그런데 stockbot-batch.service 가 있으므로 user-data.txt 의 내용을 
    읽어와 스크립트로 실행하는 코드를 추가하자.





 