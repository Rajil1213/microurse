
<template>
  <div>
    <form action="post" @submit.prevent="createPost">
      <div class="form-group">
        <input id="post-title" type="text" placeholder="Comment" v-model="formData.content" class="form-control">
      </div>
      <button type="submit" class="btn btn-primary">Submit</button>
    </form>
    <FeedbackContainer :feedback="feedback" />
  </div>
</template>

<script setup lang="ts">
import { shallowReactive, ref } from "vue";
import { commentsClient } from "@/services";
import FeedbackContainer from "../FeedbackContainer.vue";
import type { Feedback } from "../FeedbackContainer.vue";
import { AxiosError } from "axios";

const { postId } = defineProps<{ postId: string }>();
const emit = defineEmits<{ "create": [ string ] }>()

interface PostData {
  content: string;
}

const formData = shallowReactive<PostData>({ content: "" });
const feedback = ref<Feedback>({ status: null, message: "" });

const createPost = async () => {
  try {
    const result = await commentsClient.post(`/posts/${postId}/comments`, formData);

    if (result.status === 201) {
      emit("create", postId);
    } else {
      console.log(result);
      feedback.value = { status: "error", message: "Something went wrong" };
    }
  } catch (ex: unknown) {
    console.log(ex);
    const defaultErrorMesage = "Something went wrong";
    if (ex instanceof AxiosError) {
      feedback.value = { status: "error", message: ex.response?.data.message || defaultErrorMesage };
    }
    feedback.value = { status: "error", message: defaultErrorMesage };
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
  @apply border-slate-500 border-2 m-2 rounded-sm p-1 px-2 w-[75%] text-sm; 
}

form button[type="submit"] {
  @apply text-center justify-center text-sm;
}
        
</style>
