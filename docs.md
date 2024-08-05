


Setup instructions.

Firstly,install LLVM 17


Do this to link llvm-config-17 to llvm-config
sudo ln -s /usr/bin/llvm-config-17 /usr/bin/llvm-config


Set up Polly
wget https://apt.llvm.org/llvm.sh && \
    chmod u+x llvm.sh && \
    sudo ./llvm.sh 17 && \
    sudo apt install libpolly-17-dev libz-dev
