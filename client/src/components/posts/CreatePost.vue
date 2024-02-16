<template>
  <div>
    <form action="post" @submit.prevent="createPost">
      <div class="form-group">
        <input
          id="post-title"
          type="text"
          placeholder="Post Title"
          v-model="formData.title"
          class="form-control"
        />
      </div>
      <button type="submit" class="btn btn-primary">Submit</button>
    </form>
    <FeedbackContainer :feedback="feedback" />
  </div>
</template>

<script setup lang="ts">
import { shallowReactive, ref } from "vue";
import { backendClient } from "@/services";
import FeedbackContainer from "../FeedbackContainer.vue";
import type { Feedback } from "../FeedbackContainer.vue";
import { POSTS_ROUTE } from "@/constants";

const emit = defineEmits<{ (e: "create"): void }>();

interface PostData {
  title: string;
}

const formData = shallowReactive<PostData>({ title: "" });
const feedback = ref<Feedback>({ status: null, message: "" });

const createPost = async () => {
  try {
    const result = await backendClient.post(POSTS_ROUTE, formData);

    if (result.status === 201) {
      feedback.value = { status: "success", message: "Post created successfully" };
      emit("create");
    } else {
      console.log(result);
      feedback.value = { status: "error", message: "Something went wrong" };
    }
  } catch (ex: unknown) {
    console.log(ex);
    feedback.value = { status: "error", message: "Something went wrong" };
  }
};
</script>

<style scoped>
form {
  @apply w-full justify-center text-center;
}

.form-group {
  @apply m-2;
}

.form-control {
  @apply m-2 w-1/3 rounded-sm border-2 border-slate-500 p-1 px-2;
}

form button[type="submit"] {
  @apply justify-center text-center;
}
</style>
