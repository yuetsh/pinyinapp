<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const pinyin = ref("");
const word = ref("");

async function toPinyin() {
  pinyin.value = await invoke("word_to_pinyin", { word: word.value });
}
</script>

<template>
  <div class="container">
    <div class="panel" v-if="pinyin">
      <p class="pinyin">{{ pinyin }}</p>
    </div>
    <form class="row" @submit.prevent="toPinyin">
      <input autofocus maxlength="10" v-model="word" placeholder="请输入文字" />
      <button type="submit">拼音</button>
    </form>
  </div>
</template>
<style>
input {
  margin-right: 1rem;
}

.pinyin {
  font-size: 6rem;
  margin: 0;
  text-align: center;
}

.container {
  padding: 2rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  width: 100vw;
  height: 100vh;
  background-image: url("./assets/background.png");
  background-size: cover;
}

.panel {
  margin-bottom: 3rem;
  padding: 1rem;
  background-color: white;
  border-radius: 8px;
  width: 100%;
  max-width: 622px;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

.row {
  display: flex;
  justify-content: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 2em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  text-align: center;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
