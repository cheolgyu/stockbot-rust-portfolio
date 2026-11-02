
id=i-00000000000000000
region=ap-northeast-2

aws configure set aws_access_key_id $AWS_ACCESS_KEY_ID
aws configure set aws_secret_access_key $AWS_SECRET_ACCESS_KEY

echo "aws ec2 stop-instances --instance-ids $id  --region $region  "
aws ec2 stop-instances --instance-ids $id  --region $region 
# iam 에 ec2 full 역활 추가 
# aws ec2 stop-instances --instance-ids i-00000000000000000  --region ap-northeast-2
