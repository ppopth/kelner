# [Kelner](https://github.com/haxxpop/kelner)

Kelner is a safe, portable, simple microkernel written purely in Rust.

* **Why Rust:** Kelner decided to use Rust because Rust is resistant to many kinds of system-level attacks such as stack-overflow, use-after-free. In additions, Rust is null-safe. It has nullable types and non-nullable types separately, while, in some other popular languages, their types are all nullable. Rust has no data race. We cannot have multiple pointers referencing and modifying the same data. Only one can do it! Using these features, we can reduce bugs in a very complicated kernel without using some sanitizer, like ASAN or LSAN.
* **Why implement a new kernel:** Many people may be thinking that we already have many Rust kernels. Why do we need another one? The answer is that most of those kernels are just toy projects and they are too simple to be used in real life. Some kernels are not intended to be toy projects, but the direction those kernels go does not satisfy me. For example, some of them intended to be a full-featured operating system which means it will include the shell, standard library, or even its own windowing systems. Kelner is in another way. We do not want to implement everything. We tend to port existing software to our kernel.
* **Kelner is POSIX compliant:** We need Kelner to be POSIX compliant because, as we mentioned earlier, portability is our main goal of this project. We cannot port many things without POSIX compatibility.


## Getting Started
Check out repositories and initialize all submodules.
```
git clone https://github.com/haxxpop/kelner.git
cd kelner
git submodule update --init --recursive
```
Install some dependencies, x86 Qemu, and Rust compiler.
```
sudo apt-get update
sudo apt-get install curl nasm qemu-system-x86 build-essential
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```
Compile the kernel after this step you will get many files in `target` which contains built files from Rust Cargo and `bulid` which contains some kernel image files to be run in Qemu.
```
make
```
Run the kernel with Qemu.
```
make qemu
```
## Running Kelner in VirtualBox
For those who want to run Kelner in a hardware virtualization system like VirtualBox instead of a software  virtualization system like Qemu, you can follow the following steps.

Build Kelner and get `build/disk`.
```
make
```

Since our `build/disk` is not in a standard virtual disk format, we need to build a new one in a valid format. In this case, we will use VMDK format. After this step, we will get the file `kelner.vmdk`.
```
dd if=/dev/zero of=kelner.img bs=512 count=2880
dd if=build/disk of=kelner.img bs=512 conv=notrunc
VBoxManage convertfromraw kelner.img kelner.vmdk --format VMDK
```
Create a new vm called `kelner`.
```
VBoxManage createvm --name kelner --register
```
Create a storage controller and attach our `kelner.vmdk` to it.
```
VBoxManage storagectl kelner --name kelnerhdd --add sata
VBoxManage storageattach kelner --type hdd --medium kelner.vmdk --storagectl kelnerhdd --port 1
```
Start our vm. Enjoy!
```
VBoxManage startvm kelner
```
