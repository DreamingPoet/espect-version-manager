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
