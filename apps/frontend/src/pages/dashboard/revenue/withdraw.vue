<template>
  <section class="universal-card">
    <Breadcrumbs
      current-title="提现"
      :link-stack="[{ href: '/dashboard/revenue', label: '收益' }]"
    />

    <h2>提现功能升级中</h2>
    <p>
      原有的 PayPal / Tremendous / Venmo 提现通道已下线，新通道（云账户）正在接入中。
      您当前的余额
      <strong>{{ $formatMoney(userBalance.available) }}</strong>
      将完整保留，新通道上线后即可提现。
    </p>
    <p>
      如有疑问，请通过邮箱
      <a class="text-link" href="mailto:support@bbsmc.net">support@bbsmc.net</a>
      联系我们。
    </p>
    <div class="button-group">
      <nuxt-link to="/dashboard/revenue" class="iconified-button">
        <XIcon />
        返回
      </nuxt-link>
    </div>
  </section>
</template>

<script setup>
import { XIcon } from "@modrinth/assets";
import { Breadcrumbs } from "@modrinth/ui";

const { data: userBalance } = await useAsyncData(`payout/balance`, () =>
  useBaseFetch(`payout/balance`, { apiVersion: 3 }),
);
</script>

<style lang="scss" scoped>
.button-group {
  margin-top: var(--spacing-card-md);
}
</style>
