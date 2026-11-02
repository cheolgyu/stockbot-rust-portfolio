#=======================================
# aws cli 설치
# curl "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip" -o "awscliv2.zip"
# unzip awscliv2.zip
# sudo ./aws/install --bin-dir /usr/local/bin --install-dir /usr/local/aws-cli --update
# dir 만들기
cd ~
mkdir ./stockbot
# https://linuxconfig.org/how-to-run-script-on-startup-on-ubuntu-20-04-focal-fossa-server-desktop
sudo apt-get update -y
# 한글
sudo apt-get install -y language-pack-ko
sudo nano /etc/default/locale
LANG=ko_KR.UTF-8
#LANG=C.UTF-8
#aws config
# env 설정

# root에다 만들기
aws configure set aws_access_key_id <비밀>
aws configure set aws_secret_access_key <비밀>


#=======================================
# systemd 시작 서비스 만들기
# cd stockbot 
scp -i "~/.ssh/example.pem" ./aws/batch/stockbot-batch-start.sh  ubuntu@ec2-example.example.invalid:~/stockbot-batch-start.sh
sudo su
mv /home/ubuntu/stockbot-batch-start.sh /usr/local/stockbot-batch-start.sh

scp -i "~/.ssh/example.pem" ./aws/batch/stockbot-batch.service  ubuntu@ec2-example.example.invalid:~/stockbot-batch.service
mv /home/ubuntu/stockbot-batch.service /etc/systemd/system/stockbot-batch.service

sudo mv ./stockbot-batch-start.sh  /usr/local/stockbot-batch-start.sh
sudo mv ./stockbot-batch.service  /etc/systemd/system/stockbot-batch.service

sudo chmod 744 /usr/local/stockbot-batch-start.sh
sudo chmod 664 /etc/systemd/system/stockbot-batch.service

sudo systemctl daemon-reload
sudo systemctl enable stockbot-batch.service

sudo ls /root/

systemctl start stockbot-batch
systemctl stop stockbot-batch
systemctl enable --now stockbot-batch



sudo systemctl daemon-reload
sudo systemctl restart stockbot-batch.service
sudo systemctl status stockbot-batch.service

nano /usr/local/stockbot-batch-start.sh
cat /home/ubuntu/stockbot-batch-start.txt

# 