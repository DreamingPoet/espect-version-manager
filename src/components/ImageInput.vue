<template>
  <el-upload class="image-uploader" ref="upload" action="" :show-file-list="false" :on-change="onImageChange" :limit="1"
    :auto-upload="false">
    <img v-if="imageUrl" :src="imageUrl" class="image" />
    <el-icon v-else class="image-uploader-icon">
      <Plus />
    </el-icon>
  </el-upload>
</template>

<script lang="ts" setup>
import { ElMessage, UploadFile, UploadInstance } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'

import type { UploadProps } from 'element-plus'


// 数据绑定组件到组件
// 关于组件通信见微信公众号 "web前端开发" 的 7个 Vue 3 中的组件通信方式
import { defineProps, ref } from 'vue'

// 本案例使用 v-model 的方式, 因为该子组件及其父组件都需要修改该变量
// 在子组件中定义一个变量的名称及其类型
// 在父组件中用 v-model:imageUrl="xxx" 绑定
const props = defineProps({
  imageUrl: {
    type: String,
    default: () => "",
  },
})

// 绑定到上传的组件上，方便接下来清除文件数组
const upload = ref<UploadInstance>()

// 定义更新到父组件中的变量的函数
const emits = defineEmits(['update:imageUrl'])

const onImageChange: UploadProps['onChange'] = (rawFile) => {

  // 必须清除，才能触发OnChange
  upload.value!.clearFiles()

  if (rawFile.raw?.type !== 'image/jpeg' && rawFile.raw?.type !== 'image/png') {
    ElMessage.error('仅支持 JPG 和 PNG 格式!')
    return false
  } else if (rawFile.raw?.size / 1024 / 1024 > 1) {
    ElMessage.error('图片大小不能超过 1MB!')
    return false
  }


  UploadFileToBase64(rawFile)
    .then(base64String => {

      // 更新到父组件中的变量
      let image_Url = props.imageUrl
      // image_Url = URL.createObjectURL(rawFile.raw!)
      emits('update:imageUrl', "data:image/*;base64," + base64String)

    })
    .catch(error => {
      ElMessage.error('图片转码失败!')
    });


  return true
}


function UploadFileToBase64(uploadFile: UploadFile): Promise<string> {
  const file = uploadFile.raw;
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file!);
    reader.onloadend = function () {
      const base64String = reader.result?.toString().split(",")[1];
      if (base64String) {
        resolve(base64String);
      } else {
        reject("Failed to convert file to Base64");
      }
    };
  });
}

</script>

<style scoped>
/* scoped style 只在当前文件有效 */

.image-uploader .image {
  width: var(--upload-image-size);
  height: var(--upload-image-size);
  display: block;
}

/* :deep() 穿透设置属性*/
:deep().el-upload {
  width: var(--upload-image-size);
  height: var(--upload-image-size);
}
</style>

<style>
/* style 可以定义变量
  var()函数可以代替元素中任何属性中的值的任何部分。
  var()函数不能作为属性名、选择器或者其他除了属性值之外的值。
*/

/* 可以定义在父标签中,然后其子标签都可以使用
  body {
    --upload-image-size: 100px;
  }
*/

/* 定义到全局
  :root是一个伪类，表示文档根元素，非IE及ie8及以上浏览器都支持，
  在:root中声明相当于全局属性，只要当前页面引用了:root segment所在文件，都可以使用var()来引用
  不能定义在 scoped 的 style 中
  :root {
    --upload-image-size: 100px;
  }
*/

/* 变量还可以参与计算
  calc(var(--el-menu-item-height) - 6px);
*/

body {
  /* 上传图标的显示大小 */
  --upload-image-size: 100px;
}

.image-uploader .el-upload {
  border: 1px dashed var(--el-border-color);
  border-radius: 6px;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  transition: var(--el-transition-duration-fast);
}

.image-uploader .el-upload:hover {
  border-color: var(--el-color-primary);
}

.el-icon.image-uploader-icon {
  font-size: 28px;
  color: #8c939d;
  width: 178px;
  height: 178px;
  text-align: center;
}
</style>