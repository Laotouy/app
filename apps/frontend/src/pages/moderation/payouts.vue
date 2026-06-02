<template>
  <div>
    <NewModal ref="confirmModal">
      <template #title>
        <div class="truncate text-lg font-extrabold text-contrast">确认转账</div>
      </template>

      <div v-if="active" class="modal-body">
        <p class="modal-lead">请在提交云账户前核对收款人、支付宝账号、金额和订单号。</p>

        <div class="summary-panel">
          <div class="summary-row summary-row-strong">
            <span class="label">提现金额</span>
            <strong>{{ $formatMoney(active.amount) }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">用户承担服务费</span>
            <strong>{{ $formatMoney(active.fee || 0) }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">余额扣除</span>
            <strong>{{ $formatMoney(active.amount) }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">订单号</span>
            <code>{{ active.order_id }}</code>
          </div>
          <div class="summary-row">
            <span class="label">用户</span>
            <span>@{{ active.username }}</span>
          </div>
          <div class="summary-row">
            <span class="label">真实姓名</span>
            <strong>{{ active.real_name || "缺失" }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">支付宝账号</span>
            <strong class="monospace">{{ active.alipay_account || "缺失" }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">手机号</span>
            <span>{{ active.phone_masked || "缺失" }}</span>
          </div>
          <div class="summary-row">
            <span class="label">身份证</span>
            <span>**************{{ active.id_card_last4 || "缺失" }}</span>
          </div>
          <div class="summary-row">
            <span class="label">签约状态</span>
            <span :class="statusClass(active.sign_status)">
              {{ signStatusLabel(active.sign_status) }}
            </span>
          </div>
        </div>

        <p v-if="!active.kyc_matches_payout" class="warning">
          用户当前签约资料与提现创建时的收款账号不一致，后端也会拒绝确认。
        </p>

        <Checkbox
          v-model="confirmed"
          class="confirm-check"
          :disabled="!active.kyc_matches_payout"
          description="确认核对无误"
        >
          我已核对姓名、支付宝账号、金额和订单号，确认提交云账户转账。
        </Checkbox>
      </div>

      <div class="modal-actions">
        <ButtonStyled color="green">
          <button
            :disabled="confirming || !confirmed || !active?.kyc_matches_payout"
            @click="doConfirm"
          >
            <CheckIcon aria-hidden="true" />
            {{ confirming ? "提交中..." : "确认转账" }}
          </button>
        </ButtonStyled>
        <ButtonStyled>
          <button :disabled="confirming" @click="confirmModal?.hide()">取消</button>
        </ButtonStyled>
      </div>
    </NewModal>

    <NewModal ref="rejectModal">
      <template #title>
        <div class="truncate text-lg font-extrabold text-contrast">退回提现</div>
      </template>

      <div v-if="rejectActive" class="modal-body">
        <p class="modal-lead">请确认该订单不需要继续转账处理。</p>

        <div class="summary-panel">
          <div class="summary-row summary-row-strong">
            <span class="label">提现金额</span>
            <strong>{{ $formatMoney(rejectActive.amount) }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">用户承担服务费</span>
            <strong>{{ $formatMoney(rejectActive.fee || 0) }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">余额释放</span>
            <strong>{{ $formatMoney(rejectActive.amount) }}</strong>
          </div>
          <div class="summary-row">
            <span class="label">订单号</span>
            <code>{{ rejectActive.order_id }}</code>
          </div>
          <div class="summary-row">
            <span class="label">用户</span>
            <span>@{{ rejectActive.username }}</span>
          </div>
          <div class="summary-row">
            <span class="label">支付宝</span>
            <span class="monospace">{{ rejectActive.alipay_account_masked || "缺失" }}</span>
          </div>
        </div>

        <p class="warning">退回后状态会变为已取消，用户余额将释放。</p>

        <label class="reason-field">
          <span class="label">退回原因</span>
          <textarea
            v-model="rejectReason"
            maxlength="500"
            rows="3"
            placeholder="可选，填写给后台审计看的退回原因"
            :disabled="rejecting"
          />
        </label>
      </div>

      <div class="modal-actions">
        <ButtonStyled color="red">
          <button :disabled="rejecting || !rejectActive" @click="doReject">
            <XIcon aria-hidden="true" />
            {{ rejecting ? "处理中..." : "确认退回" }}
          </button>
        </ButtonStyled>
        <ButtonStyled>
          <button :disabled="rejecting" @click="rejectModal?.hide()">取消</button>
        </ButtonStyled>
      </div>
    </NewModal>

    <section class="universal-card">
      <div class="header-section">
        <div>
          <h2>提现处理</h2>
          <p class="description">管理员核对后提交云账户转账，处理中订单会优先展示。</p>
        </div>
        <button class="iconified-button" :disabled="loading" @click="refresh">
          <UpdatedIcon aria-hidden="true" />
          刷新
        </button>
      </div>

      <div class="tabs-wrapper">
        <NavTabs :links="statusTabs" query="status" />
      </div>

      <div v-if="loading" class="loading-section">
        <UpdatedIcon aria-hidden="true" class="animate-spin" />
        <span>加载中...</span>
      </div>

      <div v-else-if="payouts?.length" class="payout-list">
        <div v-for="item in payouts" :key="item.id" class="payout-item">
          <div class="payout-main">
            <div class="payout-header">
              <div class="payout-title">
                <strong>{{ $formatMoney(item.amount) }}</strong>
                <span v-if="item.fee" class="muted">+ 服务费 {{ $formatMoney(item.fee) }}</span>
                <code>{{ item.order_id }}</code>
              </div>
              <span class="status-pill" :class="payoutStateClass(item)">
                {{ payoutStateLabel(item) }}
              </span>
            </div>

            <div class="detail-grid">
              <span class="label">用户</span>
              <span>@{{ item.username }}</span>
              <span class="label">真实姓名</span>
              <span>{{ item.real_name || "缺失" }}</span>
              <span class="label">支付宝</span>
              <span class="monospace">{{ item.alipay_account_masked || "缺失" }}</span>
              <span class="label">身份证</span>
              <span>**************{{ item.id_card_last4 || "缺失" }}</span>
              <span class="label">手机号</span>
              <span>{{ item.phone_masked || "缺失" }}</span>
              <span class="label">申请时间</span>
              <span>{{ formatDateTime(item.created) }}</span>
              <span v-if="item.platform_id" class="label">平台流水</span>
              <span v-if="item.platform_id" class="monospace">{{ item.platform_id }}</span>
            </div>

            <p
              v-if="item.status === 'in-transit' && !item.kyc_matches_payout"
              class="warning compact"
            >
              当前签约资料与提现创建时的收款账号不一致，需先人工处理。
            </p>
            <p
              v-else-if="item.status === 'in-transit' && item.submit_error"
              class="warning compact"
            >
              {{ item.submit_error }}
            </p>
            <p
              v-else-if="item.status === 'in-transit' && item.submit_started_at"
              class="muted compact"
            >
              已提交云账户，等待查单同步。
            </p>
          </div>

          <div v-if="item.status === 'in-transit'" class="payout-actions">
            <ButtonStyled color="green">
              <button :disabled="detailLoading || !canConfirm(item)" @click="openConfirm(item)">
                <CheckIcon aria-hidden="true" />
                {{ detailLoading ? "加载中..." : "确认转账" }}
              </button>
            </ButtonStyled>
            <ButtonStyled color="red">
              <button :disabled="rejecting || !canReject(item)" @click="openReject(item)">
                <XIcon aria-hidden="true" />
                退回提现
              </button>
            </ButtonStyled>
          </div>
        </div>
      </div>

      <div v-else class="empty-section">
        <InfoIcon aria-hidden="true" />
        <p>{{ emptyText }}</p>
      </div>

      <Pagination :page="page" :count="pages" :link-function="pageLink" @switch-page="changePage" />
    </section>
  </div>
</template>

<script setup>
import { NewModal, ButtonStyled, Checkbox } from "@modrinth/ui";
import NavTabs from "~/components/ui/NavTabs.vue";
import Pagination from "~/components/ui/Pagination.vue";
import CheckIcon from "~/assets/images/utils/check.svg?component";
import InfoIcon from "~/assets/images/utils/info.svg?component";
import UpdatedIcon from "~/assets/images/utils/updated.svg?component";
import XIcon from "~/assets/images/utils/x.svg?component";

const auth = await useAuth();
const app = useNuxtApp();
const route = useNativeRoute();
const router = useNativeRouter();

if (auth.value?.user?.role !== "admin") {
  await navigateTo("/");
}

useHead({
  title: "提现处理 - BBSMC",
  meta: [{ name: "robots", content: "noindex, nofollow" }],
});

const confirmModal = ref(null);
const active = ref(null);
const confirmed = ref(false);
const confirming = ref(false);
const detailLoading = ref(false);
const rejectModal = ref(null);
const rejectActive = ref(null);
const rejectReason = ref("");
const rejecting = ref(false);
const pageSize = 20;
const allowedStatuses = ["all", "in-transit", "success", "failed", "cancelled"];

const selectedStatus = computed(() => {
  const raw = Array.isArray(route.query.status) ? route.query.status[0] : route.query.status;
  return allowedStatuses.includes(raw) ? raw : "all";
});

const page = computed(() => {
  const raw = Array.isArray(route.query.page) ? route.query.page[0] : route.query.page;
  const parsed = Number.parseInt(raw || "1", 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 1;
});

const statusTabs = computed(() => [
  { label: "全部", href: "" },
  { label: "处理中", href: "in-transit" },
  { label: "成功", href: "success" },
  { label: "失败", href: "failed" },
  { label: "已取消", href: "cancelled" },
]);

const {
  data: payoutPage,
  pending: loading,
  refresh,
} = await useAsyncData(
  "moderation-payouts-admin",
  () => {
    const query = {
      page: page.value,
      page_size: pageSize,
    };
    if (selectedStatus.value !== "all") {
      query.status = selectedStatus.value;
    }
    return useBaseFetch("payout/admin", {
      apiVersion: 3,
      query,
    });
  },
  {
    default: () => ({
      items: [],
      total: 0,
      page: 1,
      page_size: pageSize,
    }),
    watch: [selectedStatus, page],
  },
);

const payouts = computed(() => payoutPage.value?.items || []);
const total = computed(() => payoutPage.value?.total || 0);
const pages = computed(() => Math.max(1, Math.ceil(total.value / pageSize)));
const emptyText = computed(() =>
  selectedStatus.value === "all" ? "暂无提现订单" : `暂无${statusLabel(selectedStatus.value)}提现`,
);

async function openConfirm(item) {
  detailLoading.value = true;
  confirmed.value = false;
  active.value = null;

  try {
    active.value = await useBaseFetch(`payout/admin/${item.id}`, {
      apiVersion: 3,
    });
    confirmModal.value?.show();
  } catch (err) {
    app.$notify({
      group: "main",
      title: "加载失败",
      text: err?.data?.description || err?.message || "无法加载提现详情",
      type: "error",
    });
  } finally {
    detailLoading.value = false;
  }
}

function openReject(item) {
  rejectActive.value = item;
  rejectReason.value = "";
  rejectModal.value?.show();
}

async function doConfirm() {
  if (!active.value || !confirmed.value) return;

  confirming.value = true;
  try {
    await useBaseFetch(`payout/admin/${active.value.id}/confirm`, {
      method: "POST",
      apiVersion: 3,
    });
    app.$notify({
      group: "main",
      title: "已提交转账",
      text: `${active.value.order_id} 已提交云账户处理。`,
      type: "success",
    });
    confirmModal.value?.hide();
    await refresh();
  } catch (err) {
    app.$notify({
      group: "main",
      title: "确认失败",
      text: err?.data?.description || err?.message || "提交云账户失败",
      type: "error",
    });
  } finally {
    confirming.value = false;
  }
}

async function doReject() {
  if (!rejectActive.value) return;

  rejecting.value = true;
  try {
    await useBaseFetch(`payout/admin/${rejectActive.value.id}/reject`, {
      method: "POST",
      apiVersion: 3,
      body: {
        reason: rejectReason.value.trim() || undefined,
      },
    });
    app.$notify({
      group: "main",
      title: "已退回提现",
      text: `${rejectActive.value.order_id} 已退回，用户余额已释放。`,
      type: "success",
    });
    rejectModal.value?.hide();
    await refresh();
  } catch (err) {
    app.$notify({
      group: "main",
      title: "退回失败",
      text: err?.data?.description || err?.message || "无法退回提现",
      type: "error",
    });
  } finally {
    rejecting.value = false;
  }
}

function canConfirm(item) {
  return (
    item.status === "in-transit" &&
    item.kyc_matches_payout &&
    (!item.submit_started_at || item.submit_error)
  );
}

function canReject(item) {
  return item.status === "in-transit" && !item.platform_id && !item.submit_started_at;
}

function payoutStateLabel(item) {
  if (item.status !== "in-transit") return statusLabel(item.status);
  if (item.submit_error) return "提交失败";
  if (item.submit_started_at) return "云账户提交中";
  return "待人工确认";
}

function payoutStateClass(item) {
  return {
    "status-success": item.status === "success",
    "status-cancelled": item.status === "cancelled",
    "status-error": !!item.submit_error,
    "status-submitting": !!item.submit_started_at && !item.submit_error,
    "status-failed": item.status === "failed",
  };
}

function statusLabel(status) {
  return (
    {
      all: "全部",
      "in-transit": "处理中",
      success: "成功",
      failed: "失败",
      cancelled: "已取消",
      cancelling: "取消中",
    }[status] || status
  );
}

function changePage(newPage) {
  const query = { ...route.query };
  if (newPage > 1) {
    query.page = String(newPage);
  } else {
    delete query.page;
  }
  router.push({ query });
}

function pageLink(newPage) {
  const params = new URLSearchParams();
  if (selectedStatus.value !== "all") {
    params.set("status", selectedStatus.value);
  }
  if (newPage > 1) {
    params.set("page", String(newPage));
  }
  const query = params.toString();
  return query ? `?${query}` : "?";
}

function signStatusLabel(status) {
  return (
    {
      unsigned: "未签约",
      signing: "签约中",
      signed: "已签约",
      terminated: "已解约",
    }[status] || status
  );
}

function statusClass(status) {
  return {
    "text-green": status === "signed",
    "text-red": status !== "signed",
  };
}

function formatDateTime(value) {
  if (!value) return "";
  return new Date(value).toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}
</script>

<style scoped lang="scss">
.header-section {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--gap-md);
  margin-bottom: var(--gap-lg);

  h2 {
    margin: 0;
  }
}

.description,
.muted {
  color: var(--color-text);
}

.tabs-wrapper {
  margin-bottom: var(--gap-lg);
}

.loading-section,
.empty-section {
  display: flex;
  align-items: center;
  gap: var(--gap-sm);
  color: var(--color-text);
}

.payout-list {
  display: flex;
  flex-direction: column;
  gap: var(--gap-sm);
}

.payout-item {
  display: grid;
  grid-template-columns: minmax(0, 1fr) max-content;
  gap: var(--gap-md);
  padding: var(--gap-md);
  border: 1px solid var(--color-divider);
  border-radius: var(--radius-md);
  background: var(--color-raised-bg);
}

.payout-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--gap-md);
  margin-bottom: var(--gap-md);
}

.payout-title {
  display: flex;
  align-items: baseline;
  flex-wrap: wrap;
  gap: 0.35rem;
  min-width: 0;

  strong {
    font-size: 1.05rem;
    color: var(--color-heading);
  }

  code {
    color: var(--color-text);
    overflow-wrap: anywhere;
  }
}

.status-pill {
  padding: 0.15rem 0.5rem;
  border-radius: var(--radius-sm);
  background: var(--color-button-bg);
  color: var(--color-text);
  font-size: var(--font-size-sm);
}

.status-submitting {
  color: var(--color-blue, #2563eb);
}

.status-success {
  color: var(--color-green);
}

.status-error,
.status-failed,
.status-cancelled {
  color: var(--color-red);
}

.detail-grid,
.summary-panel {
  display: grid;
  grid-template-columns: 6rem minmax(0, 1fr);
  gap: var(--gap-xs) var(--gap-md);
  align-items: center;
}

.detail-grid {
  max-width: 42rem;
}

.summary-panel {
  padding: var(--gap-md);
  border: 1px solid var(--color-divider);
  border-radius: var(--radius-md);
  background: var(--color-bg);
}

.summary-row {
  display: contents;
}

.summary-row-strong {
  strong {
    font-size: 1.15rem;
    color: var(--color-heading);
  }
}

.label {
  color: var(--color-text);
}

.monospace,
code {
  font-family: var(--font-mono);
  overflow-wrap: anywhere;
}

.payout-actions {
  display: flex;
  flex-direction: column;
  gap: var(--gap-sm);
  align-items: stretch;
  min-width: 8.75rem;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: var(--gap-md);
}

.modal-lead {
  margin: 0;
  color: var(--color-text);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--gap-sm);
  margin-top: var(--gap-lg);
}

.confirm-check {
  align-items: flex-start;
  padding: var(--gap-sm) var(--gap-md);
  border: 1px solid var(--color-divider);
  border-radius: var(--radius-md);
  background: var(--color-bg);
}

.reason-field {
  display: flex;
  flex-direction: column;
  gap: var(--gap-xs);

  textarea {
    width: 100%;
    min-height: 5rem;
    resize: vertical;
  }
}

.warning {
  padding: var(--gap-sm) var(--gap-md);
  border-left: 3px solid var(--color-red);
  background: var(--color-red-bg, rgba(220, 38, 38, 0.08));
  color: var(--color-red);
}

.warning.compact {
  margin-bottom: 0;
}

.text-green {
  color: var(--color-green);
}

.text-red {
  color: var(--color-red);
}

:deep(.paginates) {
  margin-top: var(--gap-lg);
}

@media (max-width: 700px) {
  .header-section,
  .payout-item,
  .payout-header {
    grid-template-columns: 1fr;
    flex-direction: column;
  }

  .payout-actions {
    min-width: 0;
  }

  .detail-grid,
  .summary-panel {
    grid-template-columns: 1fr;
  }

  .modal-actions {
    flex-direction: column;
  }
}
</style>
