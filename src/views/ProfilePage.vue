<template>
  <div class="flex h-full w-full flex-col gap-2 p-2">
    <div class="flex gap-2">
      <input
        type="text"
        class="input input-sm input-bordered w-full"
        v-model="profileUrl"
      />
      <button
        class="btn btn-primary btn-sm"
        @click="getRemoteProfile"
      >
        {{ $t('updateProfile') }}
      </button>
      <button
        class="btn btn-sm"
        @click="runSingBox"
      >
        {{ $t('start') }}
      </button>
      <button
        class="btn btn-sm"
        @click="stopSingBox"
      >
        {{ $t('stop') }}
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
import { useStorage } from '@vueuse/core'
import { ref } from 'vue'

const profileUrl = useStorage('app/profile-url', '')
const json = ref('')
const getProfile = async () => {
  const profile = await invoke<string>('get_profile')

  json.value = profile
}

const getRemoteProfile = async () => {
  const profile = await invoke<string>('get_remote_profile', {
    url: profileUrl.value + '?t=' + Date.now(),
  })

  json.value = profile
}

const handlerSaveProfile = () => {
  invoke('set_profile', {
    profile: json.value,
  })
}

const runSingBox = async () => {
  await invoke('run_sing_box')
  location.reload()
}

const stopSingBox = async () => {
  await invoke('stop_sing_box')
  location.reload()
}

getProfile()
</script>
