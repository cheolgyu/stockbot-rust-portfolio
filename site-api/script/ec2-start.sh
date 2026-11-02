echo date

# ec2: stockbot-batch id
id=i-00000000000000000
region=ap-northeast-2
f_user_data_base64=~/user-data_base64.txt 
f_user_data=~/user_data.txt

BATCH_TYPE=$1
COUNTRY_TYPE=$2
RUN_TYPE=$3
BATCH_TIMER_PLAN_ID=$4

echo "BATCH_TYPE=$BATCH_TYPE"
echo "COUNTRY_TYPE=$COUNTRY_TYPE"
echo "RUN_TYPE=$RUN_TYPE"
echo "BATCH_TIMER_PLAN_ID=$BATCH_TIMER_PLAN_ID"

#####################################
# aws 로그인
#####################################
echo "===================================="
echo "aws 로그인"
aws configure set aws_access_key_id $AWS_ACCESS_KEY_ID
aws configure set aws_secret_access_key $AWS_SECRET_ACCESS_KEY


#####################################
# user-data 생성
#####################################
echo "===================================="
echo "use_data 삭제"

rm  $f_user_data 
rm  $f_user_data_base64 
echo "use_data 삭제 완료"
echo "===================================="
echo "use_data 생성"
cat > $f_user_data <<- EOF
#!/bin/bash
echo "BATCH_TYPE=$BATCH_TYPE" > $f_user_data
echo "COUNTRY_TYPE=$COUNTRY_TYPE" >> $f_user_data
echo "RUN_TYPE=$RUN_TYPE" >> $f_user_data
echo "BATCH_TIMER_PLAN_ID=$BATCH_TIMER_PLAN_ID" >> $f_user_data

EOF
cat $f_user_data
echo "use_data 생성 완료"
#####################################
# user-data 생성 인코딩
#####################################
echo "===================================="
echo "user_data 생성 인코딩"
base64 $f_user_data > $f_user_data_base64
echo "user_data 생성 인코딩 완료"


# #####################################
# # ec2 user-data 수정
# #####################################
echo "===================================="
echo "AWS  ec2 user-data 수정"
echo aws ec2 modify-instance-attribute --instance-id $id --region $region --attribute userData  --value file://$f_user_data_base64
aws ec2 modify-instance-attribute --instance-id $id --region $region --attribute userData  --value file://$f_user_data_base64

echo "AWS  ec2 user-data 수정 완료"
# #####################################
# # ec2 실행
# #####################################
echo "===================================="
echo "AWS  ec2 실행"
aws ec2 start-instances --instance-ids $id  --region $region 


# #####################################
# # view user-data
# #####################################
echo "===================================="
echo "AWS  view user-data"
#aws ec2 describe-instance-attribute --instance-id $id --region $region  --attribute userData



