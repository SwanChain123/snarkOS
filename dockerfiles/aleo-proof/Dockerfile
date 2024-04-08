FROM nvidia/cuda:12.3.1-base-ubuntu20.04
RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo 'Asia/Shanghai' >/etc/timezone
RUN apt-get update && apt install -y libssl-dev ca-certificates wget && apt-get clean

ADD swan-aleo-proof /mnt/init/cmd
RUN chmod +x /mnt/init/cmd
