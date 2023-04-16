<template>
  <el-container class="main-container">
    <el-main class="client-content">



      <el-form :model="form" label-width="100px">


        <el-form-item label="exe文件">
          <el-upload class="upload-exe" ref="upload" action="" :show-file-list="false" :on-change="onFileChange"
            :limit="1" :auto-upload="false">
            <el-icon class="el-icon--upload"><upload-filled /></el-icon>
            <div class="el-upload__text">
              点击选择
            </div>   
          </el-upload>
        </el-form-item>
        
        <el-form-item label="">
          <div class="el-upload__tip" v-html="exe_path"></div>
        </el-form-item>

        <el-form-item label="客户名称">
          <el-input v-model="form.custom_name" />
        </el-form-item>
        <el-form-item label="客户图标">
          <ImageInput v-model:imageUrl="form.custom_logo" />
        </el-form-item>

        <el-form-item label="应用名称">
          <el-input v-model="form.app_name" />
        </el-form-item>
        <el-form-item label="应用图标">
          <ImageInput v-model:imageUrl="form.app_logo" />
        </el-form-item>

        <el-form-item label="版本号">
          <el-input v-model="form.version" />
        </el-form-item>

        <el-form-item label="内部版本号">
          <el-input v-model="form.internal_version" />
        </el-form-item>

        <el-form-item label="开发公司名称">
          <el-input v-model="form.develop_company_name" />
        </el-form-item>

        <el-form-item label="开发公司英文">
          <el-input v-model="form.develop_company_name_en" />
        </el-form-item>

        <el-form-item label="联系邮箱">
          <el-input v-model="form.contact_email" />
        </el-form-item>

        <el-form-item label="版权">
          <el-input v-model="form.copy_right" />
        </el-form-item>

        <el-form-item label="Commit SHA">
          <el-input v-model="form.commit_sha" />
        </el-form-item>

        <el-form-item>
          <el-button v-bind:disabled="exe_path==''" type="primary" @click="onSubmit">保存</el-button>
        </el-form-item>
      </el-form>
    </el-main>
  </el-container>
</template>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import ImageInput from './components/ImageInput.vue'

// 与 tauri 后端通信
import { invoke } from '@tauri-apps/api'
import { listen, Event } from "@tauri-apps/api/event"
import { ElMessage, UploadInstance, UploadProps } from 'element-plus'

import { UploadFilled } from '@element-plus/icons-vue'

// 绑定到上传的组件上，方便接下来清除文件数组
const upload = ref<UploadInstance>()
const exe_path = ref("")

// 定义表单数据
const form = reactive({
  custom_name: 'VR 互动教学系统',
  custom_logo: '',
  app_name: '',
  app_logo: '',
  version: '',
  internal_version: '',
  develop_company_name: '',
  develop_company_name_en: '',
  contact_email: '',
  copy_right: '© 2022-2025',
  commit_sha: '',
})

const onSubmit = () => {
  console.log('submit!')
}


// 点击加载当前目录下的版本信息
// invoke 传递的参数名称不能带下划线
const onLoadData = function(exe_path: String) {
  ElMessage.info("onLoadData = " + exe_path)
  invoke("load_data", {exefilepath: exe_path}).then(
    (data) => {
      ElMessage.info("Back data = " + String(data))
    }
  )
}


const onFileChange: UploadProps['onChange'] = (rawFile) => {
  upload.value!.clearFiles()
  // type element as HTMLInputElement
  const input = document.getElementsByClassName('el-upload__input')[0] as HTMLInputElement
  let path = input?.value
  if (path.indexOf('exe') == -1) {
    ElMessage.error('请传入exe!')
    return false
  }
  exe_path.value = path
  onLoadData(path)

  return true
}
</script>

<style scoped>
:deep().el-upload {
  --el-upload-dragger-padding-horizontal: 0px;
  --el-upload-dragger-padding-vertical: 10px;
}

:deep().el-upload-dragger .el-icon--upload {
  font-size: 30px;
  margin-bottom: 0px;
  line-height: 50px;
}

:deep().el-form-item__content {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  flex: 1;
  line-height: 20px;
  position: relative;
  font-size: var(--font-size);
  min-width: 0;
}
</style>

<style>
.el-form-item {
  display: flex;
  margin-bottom: 10px;
}
</style>