# Base docker development file
# Tag: johnellis1392/dev
FROM ubuntu:latest

# Install relevant utils
RUN apt-get update && \
  apt-get install -y \
    build-essential \
    sudo \
    net-tools \
    dnsutils \
    iptables \
    netcat \
    wget \
    curl \
    git \
    python \
    python3

# Create dev user
ENV username dev
RUN useradd -ms /bin/bash ${username}
USER ${username}

# Create env vars
ENV work_dir /home/${username}
ENV dev_dir ${work_dir}/.dev
RUN mkdir -p ${dev_dir}

WORKDIR ${work_dir}

CMD [ "/bin/bash" ]
