# 关于这个bind的事情理解

~~这个是关于资源绑定和着色器访问的概念。（这句话是总结的话，学习的时候一定要忽略）~~

## 两个关键字


**Bind Group** ,  **Bind Group Layout**

从常见中的技术去类比解释，就是schema的概念
在layout设定好他的结构后，然后去创建或生成数据。

从游戏的角度，他们喜欢叫蓝图，layout是蓝图，然后在创建和运行。

其实我认为最后细看他的layout创建实例的参数一个一个去看。当然这是我理解后的后话，不要听。

我先放一个链接https://gpuweb.github.io/gpuweb/#dictdef-gpubindgrouplayoutentry
大概看看他每个字段是说明什么
从 binding 到 visibility 到 [Shader Stage](https://gpuweb.github.io/gpuweb/#namespacedef-gpushaderstage)

然后再看我这句话： 
> 当 GPU 在执行渲染管线时，它会按照一定的顺序执行各个着色器阶段。在顶点着色器阶段，GPU 会处理每个顶点的数据；在片段着色器阶段，GPU 会处理每个像素的颜色和深度。如果一个资源被设置为只能在片段着色器阶段访问，那么在顶点着色器阶段，这个资源是不可见的，只有在片段着色器阶段，这个资源才可以被访问。

--- 

中间暂停了一下，从ipad和vscode上有记录了很多的理解。那我在这里尽量复述一下自己的理解。

其中我现在记得比较深的是buffer，与他有点相同有不同的是uniform。
其实就是完全不同哈哈，先不要紧，先提出来。

最主要是继Bind group 和layout之后的是 texture 的一些内容。

在texture从一个文件的读取加载， 经过对这个图片的加载的一些选项或者说加载的方式设置，然后会有一个概念是view。

view，也就是texture view，他是可以这样直接说来理解，把资源读取加载进来后，这个过程可以在程序一开始就可以去开展贴图的加载，然后在对这个图片的数据进行不同的view创建，从一次加载好的数据，可以不同的view，比如一个大图的多个少图的截取来实现多个不同的view。然后就是通过组建好的包括了view sampler还有api下创建出来的texture，把他们放到bindgroup里面去，而这个bindgroup的layout就是去填写设计好的layout（也是api下的创建，填写一些参数）

然后就是大，上的东西，pipeline，这个概念就是负责处理这些资源的如何安排，如何运行，如何运作的，可以多个pipeline一些组建不同的画面处理。

queue 和 pipeline， queue 更上层一点，他从adapter的api下获得了（device，queue）主要是从大的wgpu的对象下运行一些API来发出设备级别的命令，比如获取device消息，

还有一个中间的， pass， 这个对象将之前所有做了的工作，比如我们准备好的layout，和pipeline， 都通过在这个pass下， 意思是在这一次渲染中进行设置的一次主动调起。
```
render_pass.set_pipeline
render_pass.set_bind_group
render_pass.set_vertex_buffer
render_pass.set_index_buffer
render_pass.set_bind_group
render_pass.set_vertex_buffer
render_pass.set_vertex_buffer
render_pass.set_index_buffer
render_pass.set_index_buffer
render_pass.draw_indexed
```
然后最后用queue去submit这个pass


pass 之上还有一个encoder，问过，可以多个，只要记住，他是用于编码的，
将所有的rust代码转译到gpu上，因为我们此前所有执行的commands都可以通过这里
把命令都转化到gpu可运作的内容

## encoder 
指令的“欢送” 
下面的代码，是让我们记得在encoder和pass之间的关系，以最简单和最基本的视野去看，一下子就明朗很多。
```rust
// 创建一个 Encoder
let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    label: Some("Render Encoder"),
});

// 开始一个新的 RenderPass
{
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    // 设置管线状态
    render_pass.set_pipeline(&render_pipeline);

    // 绑定顶点缓冲区
    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

    // 绑定索引缓冲区
    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

    // 绑定纹理和采样器
    render_pass.set_bind_group(0, &bind_group, &[]);

    // 提交绘制命令
    render_pass.draw_indexed(0..index_count, 0, 0..1);
}

// 提交 Encoder
queue.submit(Some(encoder.finish()));
```

然后主要是device来管理encoder的
