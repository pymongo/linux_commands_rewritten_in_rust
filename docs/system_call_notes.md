# Linux system programming

This repo is cover these topics:
- linux commands rewritten in Rust

## 系统调用通用知识

### 相同名字的命令(可执行文件)和系统调用函数成对出现

大部分系统调用函数都会有一个同名的命令，例如chmod命令对应chmod系统调用函数

`man 1 chmod`能打开chmod命令的文档，`man 2 chmod`能打开chmod系统调用函数的文档

---

### 所有系统调用函数指针类型入参都有`restrict`修饰

这是为了编译器优化，可以参考: <https://www.zhihu.com/question/41653775>

通过程序员保证传入的指针独享其指向的内存，不会有其它指针也指向该内存，让编译器进行更多优化

但对 Rust 来说，由于所有权机制就像一个RwLock，同时只能有一个指针对某块内存有mutable的权限，所以不需要C语言的restrict关键词去修饰指针

如果用了 restrict 去修饰两个指针，而它们在作用域内又指向同一地址，那么是 UB

---

### 获取C语言函数返回值的几种情况

1. 返回值是`T`，直接用let接收返回值即可，例如: rand, time
2. (尽量不用)返回值是`*const T`，解引用返回值原始指针，例如: localtime
3. 返回值是`*const T`但有入参是`*mut T`，允许Rust传递可变指针，C函数内部将返回值写入到可变指针中，例如: localtime_r

一般系统调用函数例如localtime既提供返回引用的localtime也提供调用方传入可变指针作为返回值的localtime_r

因为FFI调用更倾向于调用方进行内存分配/回收管理，所以Rust一定就要用传入可变指针让系统调用函数把返回值写进去的方式

所以同样是localtime的系统调用Rust会用localtime_r而不用localtime

---

### 系统调用函数错误处理

系统调用失败时会返回 -1(例如stat) 或 NULL(例如localtime_r)

此时会更新 thread_local 的 errno 变量，可以通过 errno 相关的几个系统调用知道错误码或错误原因

---

### 系统调用常见缩写
- dirp -> directory stream pointer
- ent -> entry: dirent.h, ENOENT(error no entry)
- nam -> name: getpwnam, tmpnam
- ppid -> parent PID
