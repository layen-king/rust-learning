# 一步步学习Rust

## old文件夹
> 基础书写

## fist文件夹
> 用最简陋的方式实现web服务,第一次尝试

## tips文件夹
> 一些基础用法

## 一些问题

#### clone git仓库遇到错误

> GnuTLS recv error (-110): The TLS connection was non-properly terminated
```shell
sudo apt-get install build-essential fakeroot dpkg-dev -y
sudo apt-get build-dep git -y
sudo apt-get install libcurl4-openssl-dev -y
cd ~
mkdir source-git
cd source-git/
apt-get source git
cd git-2.*.*/
sed -i -- 's/libcurl4-gnutls-dev/libcurl4-openssl-dev/' ./debian/control
sed -i -- '/TEST\s*=\s*test/d' ./debian/rules
dpkg-buildpackage -rfakeroot -b -uc -us
sudo dpkg -i ../git_*ubuntu*.deb
```