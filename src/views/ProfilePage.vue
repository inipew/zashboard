<template>
  <div class="flex h-full w-full flex-col gap-2 p-2">
    <div class="flex gap-2">
      <input
        type="text"
        class="input input-bordered w-full"
        v-model="profileUrl"
      />
      <button
        class="btn btn-primary"
        @click="getRemoteProfile"
      >
        {{ $t('getProfile') }}
      </button>
    </div>
    <textarea
      v-model="json"
      class="textarea textarea-bordered h-96 w-full flex-1"
      @change="handlerSaveProfile"
    />
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const profileUrl = ref('')
const json = ref('')
const getProfile = async () => {
  const profile = await invoke('get_profile')

  json.value = profile
}

const getRemoteProfile = async () => {
  const profile = await invoke('get_remote_profile', {
    url: profileUrl.value,
  })

  json.value = profile
}

const handlerSaveProfile = () => {
  invoke('set_profile', {
    profile: json.value,
  })
}

getProfile()
</script>
