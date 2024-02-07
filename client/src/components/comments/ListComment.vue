<template>
  <div class="text-center">
    <FeedbackContainer :feedback="feedback" />
    <hr class="my-5" />
    <p v-if="noComments" class="italic text-xs m-2">No Comments</p>
    <div v-else>
      <h4 class="text-md">Comments</h4>
      <li class="comments-list" :key="commentKey">
        <ul v-for="comment in comments" class="comment-card" :key="comment.id">
          <h4 class="text-sm">{{ comment.content }}</h4>
        </ul>
      </li>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, watch } from "vue";
import { commentsClient } from "@/services";
import FeedbackContainer, { type Feedback } from "../FeedbackContainer.vue";
import { AxiosError } from "axios";

interface Comment {
  id: string;
  content: string;
}

const props = defineProps<{ postId: string, changedCommentId: string }>();

const comments = ref<Array<Comment>>([]);
const feedback = ref<Feedback>({ status: null, message: "" });
const commentKey = ref<number>(0);

const fetchComments = async () => {
  try {
    const res = await commentsClient.get<Array<Comment>>(`/posts/${props.postId}/comments`);
    if (res.status === 200) {
      comments.value = res.data;
    } else {
      feedback.value = {
        status: "error",
        message: res.statusText
      };
    }
  } catch (ex: unknown) {
    if (!(ex instanceof AxiosError)) {
      feedback.value = { status: "error", message: "Could not fetch comments" };
      return;
    }

    if (ex.response && ex.response.status !== 404) {
      feedback.value = {
        status: "error",
        message: ex.message
      };
      return;
    }
  }
}

onMounted(async () => {
  await fetchComments();
})

watch(() => props.changedCommentId, async () => {
  const id = props.changedCommentId.split("|")[ 0 ]; // the second part is a random value

  if (id === props.postId) {
    await fetchComments();
  }
});

const noComments = computed(() => comments.value.length === 0);
</script>

<style scoped>
.comments-list {
  @apply list-none m-2 my-5 max-h-[4rem] overflow-scroll bg-slate-300;
}

.comment-card {
  @apply h-fit shadow-md bg-teal-100 p-1 m-2;
}
</style>