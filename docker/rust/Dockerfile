# Dockerfile for building and setting up rust dev utils & emscripten
# Tag: johnellis1392/rust
FROM johnellis1392/dev

# Set relevant env vars
ENV cargo_dir ${work_dir}/.cargo
ENV PATH $PATH:${cargo_dir}/bin
ENV emsdk_dir ${dev_dir}/emsdk

# Get rustup
RUN curl https://sh.rustup.rs -sSf > ${dev_dir}/rustup.sh
RUN chmod u+x ${dev_dir}/rustup.sh
RUN ${dev_dir}/rustup.sh -y
RUN rm -f ${dev_dir}/rustup.sh

# Set up rust
RUN rustup install stable
RUN rustup default stable
ENV PATH $PATH:$HOME/.cargo/bin

# Set up wasm
RUN rustup target add wasm32-unknown-emscripten
RUN cd ${dev_dir} && git clone https://github.com/juj/emsdk
# RUN cd ${emsdk_dir} && ./emsdk update
RUN cd ${emsdk_dir} && ./emsdk install latest
RUN cd ${emsdk_dir} && ./emsdk activate latest
# RUN cd ${emsdk_dir} && source ./emsdk_env.sh

# NOTE: When compiling wasm projects with rustc use:
# rustc --target=wasm32-unknown-emscripten <input>.rs -o <output>.html
# Then run `python -m SimpleHTTPServer` in the target directory and navigate
# to localhost:8000/<output>html to see the result

WORKDIR ${work_dir}

CMD [ "/bin/bash -c", "cd ${emsdk_dir} && source ./emsdk_env.sh;", "cd ${work_dir}", "/bin/bash" ]
