<template>
  <div v-if="showFeedback" class="feedback-container">
    <div class="feedback-message" :class="{ 'feedback-success': !isError, 'feedback-error': isError }">
      <p>{{ feedback.message }}
      </p>
      <button @click="clearFeedback">x</button>
    </div>
  </div>
</template>

<script lang="ts">
export interface Feedback {
  status: "success" | "error" | null;
  message: string;
}
</script>

<script setup lang="ts">
import { watch, ref, computed } from "vue";

const props = defineProps<{ feedback: Feedback }>();

const isError = computed(() => props.feedback.status === "error");

const showFeedback = ref<boolean>(props.feedback.status !== null);

watch(() => props.feedback, () => {
  showFeedback.value = props.feedback !== null;
})

const clearFeedback = () => {
  showFeedback.value = false;
}
</script>

<style scoped>
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