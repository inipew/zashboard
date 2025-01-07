<template>
  <template v-if="rulesTabShow === RULE_TAB_TYPE.PROVIDER">
    <div class="flex flex-col gap-1 overflow-y-auto overflow-x-hidden p-2">
      <RuleProvider
        v-for="(ruleProvider, index) in renderRulesProvider"
        :key="ruleProvider.name"
        :ruleProvider="ruleProvider"
        :index="index + 1"
      />
    </div>
  </template>
  <template v-else>
    <div>
      <VirtualScroller :data="renderRules">
        <template v-slot="{ item: rule }: { item: Rule }">
          <RuleCard
            :key="rule.payload"
            :rule="rule"
            :index="rules.indexOf(rule) + 1"
          />
        </template>
      </VirtualScroller>
    </div>
  </template>
</template>

<script setup lang="ts">
import VirtualScroller from '@/components/common/VirtualScroller.vue'
import RuleCard from '@/components/rules/RuleCard.vue'
import RuleProvider from '@/components/rules/RuleProvider.vue'
import { rulesTabShow } from '@/composables/rules'
import { RULE_TAB_TYPE } from '@/config'
import { renderRules, renderRulesProvider, rules } from '@/store/rules'
import type { Rule } from '@/types'
</script>
