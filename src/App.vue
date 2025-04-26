<script setup>
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "./components/Button.vue";
import { createToaster } from "@meforma/vue-toaster";

const clickFunc = async (ev) => {
  let path = await open({
    filters: [
      {
        name: "Mod File",
        extensions: ["jar", "jar.dis"],
      },
    ],
    multiple: false,
  });

  let typeOfFile = "enable";

  if (path.includes(".dis")) {
    typeOfFile = "disable";
  }

  await invoke("toggle", { path: path, typeOfFile: typeOfFile });

  const toaster = createToaster();

  toaster.success("✅ Successfully toggled!", {
    position: "top-right",
  });
};
</script>

<template>
  <div class="content">
    <h1 class="title">Mod Toggler</h1>
    <Button :clickFunc="clickFunc">Choose mod</Button>
  </div>
</template>

<style>
body {
  height: 100vh !important;
  font-family: "Winky Rough" !important;
  height: 100%;
  background-color: #ba0012 !important;
}

#app,
.content {
  height: 100%;
}

h1 {
  color: white !important;
  text-align: center;
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 50px;
}
</style>
