import axios from "axios";
import { COMMENTS_SERVICE_URL, POSTS_SERVICE_URL, QUERY_SERVICE_URL } from "@/constants";

export const postsClient = axios.create({
  baseURL: POSTS_SERVICE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export const queryClient = axios.create({
  baseURL: QUERY_SERVICE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export const commentsClient = axios.create({
  baseURL: COMMENTS_SERVICE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});
