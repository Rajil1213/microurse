import axios from "axios";
import { POSTS_SERVICE_URL, COMMENTS_SERVICE_URL } from "@/constants";

export const postsClient = axios.create({
  baseURL: POSTS_SERVICE_URL,
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
