<template>
  <el-container class="main-container">
    <el-main class="client-content">



      <el-form :model="form" label-width="100px">

        <el-form-item label="exe文件">
          <div class="" v-html="exe_path"></div>
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
          <el-button v-bind:disabled="exe_path == ''" type="primary" @click="onSubmit">保存</el-button>
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
import { ElMessage, UploadFile, UploadInstance, UploadProps } from 'element-plus'



// 绑定到上传的组件上，方便接下来清除文件数组
const upload = ref<UploadInstance>()
const exe_path = ref("")


listen('tauri://file-drop', event => {




  let path = String(event.payload)

  if (path.indexOf('exe') == -1) {
    ElMessage.error('请传入exe!')
    return false
  }
  exe_path.value = path
  onLoadData(path)
  // ElMessage.info("Back data = " + exe_path.value)
})


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

  console.log(form.custom_logo)
  invoke("save_data", {
    exefilepath: exe_path.value,
    customname: form.custom_name,
    customlogo: form.custom_logo,
    appname: form.app_name,
    applogo: form.app_logo,
    version: form.version,
    internalversion: form.internal_version,
    developcompanyname: form.develop_company_name,
    developcompanynameen: form.develop_company_name_en,
    contactemail: form.contact_email,
    copyright: form.copy_right,
    commitsha: form.commit_sha,
  }).then(
    (data) => {
      ElMessage.info("Back data = " + String(data))
    }
  )


  console.log('submit!')
}


// 点击加载当前目录下的版本信息
// invoke 传递的参数名称不能带下划线
const onLoadData = function (exe_path: String) {
  ElMessage.info("onLoadData = " + exe_path)
  invoke("load_data", { exefilepath: exe_path }).then(
    (data) => {
      ElMessage.info("Back data = " + String(data))
    }
  )
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