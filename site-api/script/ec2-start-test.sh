#
# 
# sudo chmod 777 test-ec2-start.sh
# ./test-ec2-start.sh BATCH KR START 4
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
# user-data 생성
#####################################
echo "===================================="
echo "use_data 삭제"

rm  $f_user_data 
rm  $f_user_data_base64 
echo "===================================="
echo "use_data 생성"
sudo  cat > $f_user_data <<- EOF
#!/bin/bash
echo "BATCH_TYPE=$BATCH_TYPE" > $f_user_data
echo "COUNTRY_TYPE=$COUNTRY_TYPE" >> $f_user_data
echo "RUN_TYPE=$RUN_TYPE" >> $f_user_data
echo "BATCH_TIMER_PLAN_ID=$BATCH_TIMER_PLAN_ID" >> $f_user_data

EOF
cat $f_user_data
#####################################
# user-data 생성 인코딩
#####################################
echo "===================================="
echo "user_data 생성 인코딩"
base64 $f_user_data > $f_user_data_base64
cat $f_user_data_base64
