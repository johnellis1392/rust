# Develpoment image for machine learning.
# Based on the TensorFlow docker image available at:
# https://github.com/tensorflow/tensorflow/blob/master/tensorflow/tools/docker/Dockerfile
FROM johnellis1392/dev

# Install system dependencies
RUN apt-get update && \
  apt-get install -y \
  python \
  python3

# Install python utilities
RUN curl -0 https://bootstrap.pypa.io/get-pip.py && \
  python get-pip.py && \
  rm get-pip.py

RUN pip --no-cache-dir install \
  ipykernel \
  jupyter \
  matplotlib \
  numpy \
  scipy \
  sklearn \
  pandas \
  Pillow \
  && python -m ipykernel.kernelspec


# Install tensorflow
RUN pip --no-cache-dir install \
    http://storage.googleapis.com/tensorflow/linux/cpu/tensorflow-0.0.0-cp27-none-linux_x86_64.whl

# TensorBoard
EXPOSE 6006
# IPython
EXPOSE 8888

WORKDIR ${work_dir}
CMD [ "/bin/bash" ]
