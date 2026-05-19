<template>
  <div>
    <section class="universal-card">
      <h2 class="text-2xl">收益</h2>
      <div v-if="userBalance.available >= minWithdraw">
        <p>
          您当前有
          <strong>{{ $formatMoney(userBalance.available) }}</strong>
          可供提现。另有
          <strong>{{ $formatMoney(userBalance.pending) }}</strong>
          余额处于<nuxt-link class="text-link" to="/legal/incentive#pending">待结算</nuxt-link>状态。
        </p>
      </div>
      <p v-else>
        您已累计获得
        <strong>{{ $formatMoney(userBalance.available) }}</strong>，未达到 ¥{{
          minWithdraw
        }}
        的提现最低额度。另有
        <strong>{{ $formatMoney(userBalance.pending) }}</strong>
        余额处于<nuxt-link class="text-link" to="/legal/incentive#pending">待结算</nuxt-link>状态。
      </p>
      <div class="input-group mt-4">
        <nuxt-link class="iconified-button brand-button" to="/dashboard/revenue/withdraw">
          <TransferIcon /> 提现
        </nuxt-link>
        <NuxtLink class="iconified-button" to="/dashboard/revenue/transfers">
          <HistoryIcon /> 查看转账记录
        </NuxtLink>
      </div>
      <p>
        通过在 BBSMC 上传项目并从您的账户提现，即表示您同意
        <nuxt-link to="/legal/incentive" class="text-link">《创作者激励计划协议》</nuxt-link>。如需了解激励机制的详细规则，请查看
        <nuxt-link to="/legal/incentive#rules" class="text-link">激励规则</nuxt-link>章节。
      </p>
    </section>
    <section class="universal-card">
      <h2 class="text-2xl">提现通道</h2>
      <p>
        BBSMC 正在升级提现通道，原有的提现方式已下线，新通道即将上线。在此期间，您的余额将继续累计，无需任何操作。
      </p>
    </section>
  </div>
</template>
<script setup>
import { TransferIcon, HistoryIcon } from "@modrinth/assets";

useHead({
  title: "收益 - BBSMC",
  meta: [{ name: "robots", content: "noindex, nofollow" }],
});

const minWithdraw = ref(50);

const { data: userBalance } = await useAsyncData(`payout/balance`, () =>
  useBaseFetch(`payout/balance`, { apiVersion: 3 }),
);
</script>
<style lang="scss" scoped>
strong {
  color: var(--color-text-dark);
  font-weight: 500;
}
</style>
