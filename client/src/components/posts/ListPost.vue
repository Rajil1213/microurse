<template>
  <div class="text-center">
    <h1>Posts</h1>
    <hr class="my-5" />
    <FeedbackContainer :feedback="feedback" />
    <div class="posts-list">
      <div v-for="postComment in postComments" class="post-card" :key="postComment.id">
        <h1 class="text-lg">{{ postComment.post }}</h1>
        <ListComment
          :comments="postComment.comments"
          :postId="postComment.id"
          :changedCommentId="changedCommentId"
        />
        <CreateComment :postId="postComment.id" @create="createCommentHandler" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { QUERY_ROUTE } from "@/constants";
import { backendClient } from "@/services";
import FeedbackContainer, { type Feedback } from "../FeedbackContainer.vue";
import ListComment from "@/components/comments/ListComment.vue";
import CreateComment from "@/components/comments/CreateComment.vue";
import type { Comment } from "@/components/comments/ListComment.vue";

interface PostComments {
  id: string;
  post: string;
  comments: Array<Comment>;
}

const postComments = ref<Array<PostComments>>([]);
const feedback = ref<Feedback>({ status: null, message: "" });
const changedCommentId = ref<string>("");

const fetchPosts = async () => {
  try {
    const res = await backendClient.get<Array<PostComments>>(QUERY_ROUTE);
    if (res.status === 200) {
      postComments.value = res.data;
    } else {
      feedback.value = {
        status: "error",
        message: res.statusText,
      };
    }
  } catch (ex) {
    console.log(ex);
    feedback.value = { status: "error", message: "Could not fetch posts" };
  }
};

onMounted(() => {
  fetchPosts();
});

const createCommentHandler = (id: string) => {
  changedCommentId.value = `${id}|${new Date().getTime()}`;
};
</script>

<style scoped>
.posts-list {
  @apply flex w-full flex-wrap justify-around;
}

.post-card {
  @apply m-3 max-h-fit min-h-[10rem] w-[30rem] bg-teal-200 px-1 py-5 shadow-md;
}
</style>
