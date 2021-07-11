/**
最近我在看一些系统编程相关的书，里面提到4种不同的数据库，但几乎都是处理同一个数据模型(例如订单数据)
每当我学一种新的数据库时都要重复写大量模型绑定或测试代码，因此我想到了以下抽象提高数据库驱动开发和学习的效率，大伙帮忙看看有什么能改进的地方吗？
源码在: https://github.com/pymongo/linux_commands_rewritten_in_rust/tree/main/src/database

我用trait建立了数据库+模型两个泛型参数的组合的抽象
这样每当书介绍新的数据库我都能通过trait快速开发新数据库驱动并自动适应各种数据模型
目前支持mmap和dbm数据库，将来会支持mysql,sqlite,redis，代码文件结构如图2
数据库适配器层: 负责析构函数
Model层: 数据模型的定义和序列化
DAO: 数据模型的读写需求
service层: 处理业务
*/
pub mod adapters;
pub mod database_config;
mod models;
