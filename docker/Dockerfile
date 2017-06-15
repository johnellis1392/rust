# Base docker development file
# Tag: johnellis1392/dev
FROM ubuntu:latest

# Install system utils
RUN apt-get update && \
  apt-get install -y \
    build-essential \
    net-tools \
    dnsutils \
    iptables \
    netcat \
    wget \
    curl \
    git
    # sudo \

RUN apt-get install -y \
  python \
  python-dev \
  python 3 \
  python3-dev \
  openjdk-8-jdk \
  openjdk-8-jre-headless \
  rsync \
  software-properties-common \
  unzip

RUN apt-get clean

# Create dev user
ENV username dev
RUN useradd -ms /bin/bash ${username}
USER ${username}

# Create env vars
ENV work_dir /home/${username}
ENV dev_dir ${work_dir}/.dev
RUN mkdir -p ${dev_dir}

WORKDIR ${work_dir}

# NOTE: Run with `--privileged=true` to allow root access to host computer
CMD [ "/bin/bash" ]
