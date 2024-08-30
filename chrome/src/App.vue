<script setup>
import CreateAndEdit from './components/CreateAndEdit.vue'
</script>

<template>
  <div class="container">
    <div class="row">
      <h3>密码管理工具1.0</h3>
    </div>
    <div class="row">
      <div class="col-12">
        <input type="text" class="form-control" placeholder="请输入你要查询的网址">
      </div>
    </div>
    <div class="row">
      <div class="col-5 offset-7 mt-2">
        <input type="button" value="查询" class="btn btn-primary btn-sm me-2">
        <input type="button" value="添加" class="btn btn-primary btn-sm">
      </div>
    </div>
    <div class="row" id="app">
      <CreateAndEdit></CreateAndEdit>
    </div>
  </div>
</template>

<style scoped></style>
