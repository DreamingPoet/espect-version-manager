<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
</script>

<template>
  <el-container class="main-container" style="height: 580px">
    <el-header class="main-header">


      <div class="title">
        <p class="app-name"> Espect 应用管理系统</p>
        <p class="app-version">v1.0.0</p>
      </div>

    </el-header>

    <el-container>

      <el-main class="client-content">

        <!-- 内部容器 main-->

      </el-main>

    </el-container>
  </el-container>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
