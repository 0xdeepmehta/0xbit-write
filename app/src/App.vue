<script setup>
import Navbar from './components/Navbar.vue';
</script>

<template>
  <Navbar title="DemoApp"/>
  <div class="pt-20">
    <router-view />
  </div>
</template>

<style>
</style>
