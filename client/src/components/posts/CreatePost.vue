<template>
  <div>
    <form action="post" @submit.prevent="createPost">
      <div class="form-group">
        <input id="post-title" type="text" placeholder="Post Title" v-model="formData.title" class="form-control">
      </div>
      <button type="submit" class="btn btn-primary">Submit</button>
    </form>
    <div v-if="!!feedback.status" class="feedback-container">
      <div class="feedback-message"
        :class="{ 'feedback-success': feedback.status === 'success', 'feedback-error': feedback.status === 'error' }">
        <p>{{ feedback.message }}
        </p>
        <button @click="clearFeedback">x</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { shallowReactive, ref } from "vue";
import axios from "axios";

interface PostData {
  title: string;
}

const formData = shallowReactive<PostData>({ title: "" });
const feedback = ref<{ status: "error" | "success" | null, message: string }>({ status: null, message: "" });

const createPost = async () => {
  try {
    const result = await axios.post("http://localhost:4000/posts", formData);

    if (result.status === 201) {
      feedback.value = { status: "success", message: "Post created successfully" };
    } else {
      console.log(result);
      feedback.value = { status: "error", message: "Something went wrong" };
    }
  } catch (ex: unknown) {
    console.log(ex);
    feedback.value = { status: "error", message: "Something went wrong" };
  }
};

const clearFeedback = () => {
  feedback.value = { status: null, message: "" };
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
  @apply border-slate-500 border-2 m-2 rounded-sm p-1 px-2 w-1/3; 
}

form button[type="submit"] {
  @apply text-center justify-center;
}
        
.feedback-container {
  @apply my-4 p-2 w-full text-center justify-center flex;
}
        
.feedback-message {
  @apply w-fit rounded-lg p-2 px-4;
}

.feedback-success {
  @apply bg-teal-400;
}

.feedback-error {
  @apply bg-red-300;
}

.feedback-message * {
  @apply inline-block
}
        
.feedback-message button {
  @apply mx-2;
}
</style>
