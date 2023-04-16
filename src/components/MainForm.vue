<template>
    <el-form :model="form" label-width="100px">
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
            <el-button type="success" @click="onLoadData">加载</el-button>
            <el-button type="primary" @click="onSubmit">保存</el-button>
        </el-form-item>
    </el-form>
</template>
  
<script lang="ts" setup>
import { reactive } from 'vue'
import ImageInput from './ImageInput.vue'



// do not use same name with ref
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

const onLoadData = () => {
    console.log('load!')
    console.log(form.custom_logo)
}
</script>

<style>
.el-form-item {
    display: flex;
    margin-bottom: 10px;
}
</style>