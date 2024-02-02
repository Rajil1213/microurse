<template>
  <header>
    <div class="text-center bg-teal-100 p-2 mb-5">
      <h1>Blog App</h1>
    </div>
  </header>
  <CreatePost @create="createHandler" />
  <hr class="my-5" />
  <ListPost :key="postsKey" />
</template>

<script setup lang="ts">
import { ref } from "vue";
import CreatePost from "@/components/posts/CreatePost.vue";
import ListPost from "@/components/posts/ListPost.vue";

const postsKey = ref<number>(0);

const createHandler = () => {
  // increment key to re-render `ListPost`
  postsKey.value++;
}
</script>