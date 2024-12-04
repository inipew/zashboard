import { ROUTE_NAME } from '@/config'
import { i18n } from '@/i18n'
import { language } from '@/store/settings'
import { activeBackend } from '@/store/setup'
import ConnectionsPage from '@/views/ConnectionsPage.vue'
import LogsPage from '@/views/LogsPage.vue'
import ProfilePage from '@/views/ProfilePage.vue'
import ProxiesPage from '@/views/ProxiesPage.vue'
import RulesPage from '@/views/RulesPage.vue'
import SettingsPage from '@/views/SettingsPage.vue'
import { useTitle } from '@vueuse/core'
import { watch } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/proxies',
      name: ROUTE_NAME.proxies,
      component: ProxiesPage,
    },
    {
      path: '/connections',
      name: ROUTE_NAME.connections,
      component: ConnectionsPage,
    },
    {
      path: '/logs',
      name: ROUTE_NAME.logs,
      component: LogsPage,
    },
    {
      path: '/rules',
      name: ROUTE_NAME.rules,
      component: RulesPage,
    },
    {
      path: '/settings',
      name: ROUTE_NAME.settings,
      component: SettingsPage,
    },
    {
      path: '/profile',
      name: ROUTE_NAME.profile,
      component: ProfilePage,
    },
    {
      path: '/:catchAll(.*)',
      redirect: ROUTE_NAME.proxies,
    },
  ],
})

const title = useTitle('zashboard')
const setTitleByName = (name: string | symbol | undefined) => {
  if (typeof name === 'string' && activeBackend.value) {
    title.value = `zashboard | ${i18n.global.t(name)}`
  } else {
    title.value = 'zashboard'
  }
}

router.afterEach((to) => {
  setTitleByName(to.name)
})

watch(language, () => {
  setTitleByName(router.currentRoute.value.name)
})

export default router
