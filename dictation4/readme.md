So I have to start the fourth version of these code. 

There is I read a sentence about the Rust code mentality: 

Favor getting things done rather than the perfect design that will never ship.

I am still a novice guy in rust


## 默写

1. 可以先为读取资源
2. 取得适配器，做初始化

### 资源篇
1. 创建内存资源
2. 为内存使用做分布和描述

其实大部分的细节都是在这里。
三个概念： create buffer, buffer data descriptor, layout and gourp. 

只要要运行的，需要cpu作为印刷的工具去把模版设置好后，
排好布，将对应的位置的数据填充进去。

然后从用你定义好的pipeline去执行，这个pipeline除了一些他自己的基本配置设置外，
也是将你配置的资源数据去进行一些简单的配置去写入。
也就是有具体的，关于先前定义的有什么group，有什么layout，都去从pass下一个一个设置资源绑定写进去。 

然后做encode，从一开始适配器中可以获得的相当于把你的代码执行器，
翻译好，将行为和数据写入和执行。o
