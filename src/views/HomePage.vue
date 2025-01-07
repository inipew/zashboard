<template>
  <div class="w-128 drawer md:drawer-open">
    <input
      id="sidebar"
      type="checkbox"
      class="drawer-toggle"
    />

    <SideBar />

    <div
      class="drawer-content fixed bottom-0 flex h-full w-full flex-col overflow-hidden bg-base-200/40 md:relative md:w-auto"
    >
      <component
        v-if="ctrlComp && isSidebarCollapsed"
        :is="ctrlComp"
        class="w-full bg-base-100"
        :horizontal="true"
      />
      <div class="relative h-0 flex-1">
        <RouterView
          class="absolute h-full w-full"
          :class="isPWA ? 'max-md:pb-[5.5rem]' : 'max-md:pb-14'"
        />
      </div>
      <div
        ref="navBarRef"
        :class="`btm-nav glass z-30 md:hidden ${isPWA ? 'h-20 pb-8' : 'h-12'}`"
      >
        <button
          v-for="r in renderRoutes"
          :key="r"
          @click="router.push({ name: r })"
          :class="r === route.name ? 'active bg-inherit' : ''"
        >
          <component
            :is="ROUTE_ICON_MAP[r]"
            class="h-5 w-5"
          />
          <span class="text-xs">
            {{ $t(r) }}
          </span>
        </button>
      </div>
    </div>

    <dialog
      id="autoSwitchBackend"
      ref="modalRef"
      class="modal"
    >
      <div class="modal-box">
        <h3 class="text-lg font-bold">{{ $t('currentBackendUnavailable') }}</h3>
        <div class="flex justify-end gap-2">
          <button
            class="btn btn-sm"
            @click="closeModal"
          >
            {{ $t('cancel') }}
          </button>
          <button
            class="btn btn-primary btn-sm"
            @click="autoSwitchBackend"
          >
            {{ $t('confirm') }}
          </button>
        </div>
      </div>
    </dialog>
  </div>
</template>

<script setup lang="ts">
import { isBackendAvailable } from '@/api'
import ConnectionCtrl from '@/components/sidebar/ConnectionCtrl.vue'
import LogsCtrl from '@/components/sidebar/LogsCtrl.vue'
import ProxiesCtrl from '@/components/sidebar/ProxiesCtrl.vue'
import RulesCtrl from '@/components/sidebar/RulesCtrl.vue'
import SideBar from '@/components/sidebar/SideBar.vue'
import { useProxies } from '@/composables/proxies'
import { rulesTabShow } from '@/composables/rules'
import { useSettings } from '@/composables/settings'
import { useTip } from '@/composables/tip'
import { PROXY_TAB_TYPE, ROUTE_ICON_MAP, ROUTE_NAME, RULE_TAB_TYPE } from '@/config'
import { getUrlFromBackend, renderRoutes } from '@/helper'
import { fetchConfigs } from '@/store/config'
import { initConnections } from '@/store/connections'
import { initLogs } from '@/store/logs'
import { initSatistic } from '@/store/overview'
import { fetchProxies } from '@/store/proxies'
import { fetchRules } from '@/store/rules'
import { isSidebarCollapsed } from '@/store/settings'
import { activeBackend, activeUuid, backendList } from '@/store/setup'
import { useDocumentVisibility, useSwipe } from '@vueuse/core'
import { computed, ref, watch, type Component } from 'vue'
import { RouterView, useRoute, useRouter } from 'vue-router'

const modalRef = ref<HTMLDialogElement | null>(null)
const isPWA = (() => {
  return window.matchMedia('(display-mode: standalone)').matches || navigator.standalone
})()

const ctrlsMap: Record<string, Component> = {
  [ROUTE_NAME.connections]: ConnectionCtrl,
  [ROUTE_NAME.logs]: LogsCtrl,
  [ROUTE_NAME.proxies]: ProxiesCtrl,
  [ROUTE_NAME.rules]: RulesCtrl,
}

const router = useRouter()
const route = useRoute()
const ctrlComp = computed(() => {
  return ctrlsMap[route.name as keyof typeof ctrlsMap]
})

const navBarRef = ref()
const { direction } = useSwipe(navBarRef, { threshold: 15 })

const getNextRouteName = () => {
  const routeName = route.name as ROUTE_NAME

  if (routeName === ROUTE_NAME.setup) {
    return ROUTE_NAME.proxies
  }

  return renderRoutes.value[(renderRoutes.value.indexOf(routeName) + 1) % renderRoutes.value.length]
}
const getPrevRouteName = () => {
  const routeName = route.name as ROUTE_NAME

  if (routeName === ROUTE_NAME.setup) {
    return ROUTE_NAME.proxies
  }

  return renderRoutes.value[
    (renderRoutes.value.indexOf(routeName) - 1 + renderRoutes.value.length) %
      renderRoutes.value.length
  ]
}

watch(direction, () => {
  if (direction.value === 'right') {
    router.push({ name: getPrevRouteName() })
  } else if (direction.value === 'left') {
    router.push({ name: getNextRouteName() })
  }
})

const { proxiesTabShow } = useProxies()
const { checkUIUpdate } = useSettings()
watch(
  activeUuid,
  () => {
    if (!activeUuid.value) return
    rulesTabShow.value = RULE_TAB_TYPE.RULES
    proxiesTabShow.value = PROXY_TAB_TYPE.PROXIES
    fetchConfigs()
    fetchProxies()
    fetchRules()
    initConnections()
    initLogs()
    initSatistic()
    checkUIUpdate()
  },
  {
    immediate: true,
  },
)

const closeModal = () => {
  modalRef.value?.close()
}

const { showTip } = useTip()
const autoSwitchBackend = async () => {
  const otherEnds = backendList.value.filter((end) => end.uuid !== activeUuid.value)

  closeModal()
  const avaliable = (
    await Promise.all(
      otherEnds.map(async (end) => {
        return (await isBackendAvailable(end)) ? end : null
      }),
    )
  ).filter((end) => end !== null)

  if (avaliable.length > 0) {
    activeUuid.value = avaliable[0].uuid
    showTip('backendSwitchTo', {
      backend: getUrlFromBackend(avaliable[0]),
    })
  }
}

const documentVisible = useDocumentVisibility()

watch(
  documentVisible,
  async () => {
    if (
      !activeBackend.value ||
      backendList.value.length === 0 ||
      documentVisible.value !== 'visible'
    ) {
      return
    }
    try {
      const isAvailable = await isBackendAvailable(activeBackend.value)

      if (!isAvailable) {
        modalRef.value?.showModal()
      }
    } catch {
      modalRef.value?.showModal()
    }
  },
  {
    immediate: true,
  },
)
</script>
