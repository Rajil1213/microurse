<template>
  <div class="text-center">
    <h1>Posts</h1>
    <hr class="my-5" />
    <FeedbackContainer :feedback="feedback" />
    <div class="posts-list">
      <div v-for="post in posts" class="post-card" :key="post.id">
        <h1 class="text-lg">{{ post.title }}</h1>
        <ListComment :postId="post.id" :changedCommentId="changedCommentId" />
        <CreateComment :postId="post.id" @create="createCommentHandler" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { POSTS_ROUTE } from "@/constants";
import { postsClient } from "@/services";
import FeedbackContainer, { type Feedback } from "../FeedbackContainer.vue";
import ListComment from "../comments/ListComment.vue";
import CreateComment from "../comments/CreateComment.vue";

interface Post {
  id: string;
  title: string;
}

const posts = ref<Array<Post>>([]);
const feedback = ref<Feedback>({ status: null, message: "" });
const changedCommentId = ref<string>("");

const fetchPosts = async () => {
  try {
    const res = await postsClient.get<Array<Post>>(POSTS_ROUTE);
    if (res.status === 200) {
      posts.value = res.data;
    } else {
      feedback.value = {
        status: "error",
        message: res.statusText
      };
    }
  } catch (ex) {
    console.log(ex);
    feedback.value = { status: "error", message: "Could not fetch posts" };
  }
}

onMounted(() => {
  fetchPosts();
})

const createCommentHandler = (id: string) => {
  changedCommentId.value = `${id}|${new Date().getTime()}`;
}

</script>

<style scoped>
.posts-list {
  @apply flex justify-around w-full flex-wrap;
}

.post-card {
  @apply w-[30rem] max-h-fit min-h-[10rem] shadow-md bg-teal-200 py-5 px-1 m-3;
}
</style>