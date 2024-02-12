<template>
  <div class="text-center">
    <FeedbackContainer :feedback="feedback" />
    <hr class="my-5" />
    <p v-if="noComments" class="italic text-xs m-2">No Comments</p>
    <div v-else>
      <h4 class="text-md">Comments</h4>
      <li class="comments-list" :key="commentKey">
        <ul v-for="comment in comments" class="comment-card" :key="comment.id">
          <h4 class="text-sm" :class="statusToColor(comment.status)">{{ moderatedComment(comment) }}</h4>
        </ul>
      </li>
    </div>
  </div>
</template>

<script lang="ts">
export interface Comment {
  id: string;
  content: string;
  status: "approved" | "pending" | "rejected";
}
</script>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { commentsClient } from "@/services";
import FeedbackContainer, { type Feedback } from "../FeedbackContainer.vue";
import { AxiosError } from "axios";

const props = defineProps<{ comments: Array<Comment>, postId: string, changedCommentId: string }>();

const comments = ref<Array<Comment>>(props.comments);
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

watch(() => props.changedCommentId, async () => {
  const id = props.changedCommentId.split("|")[ 0 ]; // the second part is a random value

  if (id === props.postId) {
    await fetchComments();
  }
});

const noComments = computed(() => comments.value.length === 0);

const statusToColor = (status: Comment[ "status" ]): string => {
  switch (status) {
    case "approved":
      return "bg-green-200";
    case "pending":
      return "bg-yellow-200";
    case "rejected":
      return "bg-red-200";
  }
}

const moderatedComment = (comment: Comment): string => {
  const { status, content } = comment;

  switch (status) {
    case "approved":
      return content;
    case "pending":
      return "<awaiting approval>";
    case "rejected":
      return "[censored]"
  }
}
</script>

<style scoped>
.comments-list {
  @apply list-none m-2 my-5 max-h-[4rem] overflow-scroll bg-slate-300;
}

.comment-card {
  @apply h-fit shadow-md bg-teal-100 p-1 m-2;
}
</style>