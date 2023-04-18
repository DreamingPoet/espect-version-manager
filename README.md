需求：
生成一个data文件
其中包含 交付给客户的一些信息
客户logo，客户名称，title，描述等
用 JSON 字符串，然后通过 AES 编码 aes_key

```json
{
   "titile":"",
   "customer_name":"",
   "customer_logo":"data:image/webp;base64,UklGRg5XAABXRUJQVlA4IA"
}
```

前端页面功能：
1、选择图片
2、填写 titile 等


启动项目：
git clone

npm install 
将会安装 package-lock.json 记录的所有依赖包

npm run tauri dev 启动调试（也可进入 src-tauri 执行 cargo run dev）

npm run dev 启动前端页面调试

npm run build 执行构建

