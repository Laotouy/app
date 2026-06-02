<template>
  <div>
    <!-- 关闭确认弹窗 -->
    <NewModal ref="disableModal">
      <template #title>
        <div class="truncate text-lg font-extrabold text-contrast">关闭激励</div>
      </template>
      <div class="modal-content">
        <p>
          项目：<b>{{ active?.title || active?.project_id }}</b>
        </p>
        <p>
          累计有效下载：<b>{{ active?.lifetime_eff_downloads }}</b>
        </p>
        <p>
          待结算金额：<b class="text-amber">¥{{ formatMoney(active?.pending_amount) }}</b>
        </p>
        <p>已结算金额：¥{{ formatMoney(active?.settled_amount) }}</p>

        <div class="form-group mt-4">
          <label class="checkbox-row">
            <input v-model="voidPending" type="checkbox" />
            <span>
              <b>同时作废所有待结算金额</b>
              <small class="block text-secondary">
                未达 7 天结算窗口的金额将永远不再结算给作者。 适用于发现刷量嫌疑时强制中止。
              </small>
            </span>
          </label>
        </div>

        <div class="form-group">
          <label>关闭原因 / 备注</label>
          <textarea
            v-model="disableNotes"
            rows="3"
            maxlength="500"
            placeholder="留下关闭原因，会写入审计日志"
          />
        </div>
      </div>
      <div class="modal-actions">
        <ButtonStyled :color="voidPending ? 'red' : 'orange'">
          <button :disabled="submitting" @click="doDisable">
            {{ submitting ? "处理中..." : voidPending ? "关闭并作废待结算" : "关闭激励" }}
          </button>
        </ButtonStyled>
        <ButtonStyled>
          <button :disabled="submitting" @click="disableModal?.hide()">取消</button>
        </ButtonStyled>
      </div>
    </NewModal>

    <!-- 手动开通弹窗 -->
    <NewModal ref="enableModal">
      <template #title>
        <div class="truncate text-lg font-extrabold text-contrast">手动开通激励</div>
      </template>
      <div class="modal-content">
        <p>
          项目：<b>{{ active?.title || active?.project_id }}</b>
        </p>
        <p class="hint">
          手动开通会跳过作者申请流程，直接将该项目标记为「已开通激励」状态。
          已累计的有效下载和待结算金额会保留。
        </p>
        <div class="form-group">
          <label>备注（可选）</label>
          <textarea
            v-model="enableNotes"
            rows="3"
            maxlength="500"
            placeholder="留下开通原因，会写入审计日志"
          />
        </div>
      </div>
      <div class="modal-actions">
        <ButtonStyled color="green">
          <button :disabled="submitting" @click="doEnable">
            {{ submitting ? "处理中..." : "确认开通" }}
          </button>
        </ButtonStyled>
        <ButtonStyled>
          <button :disabled="submitting" @click="enableModal?.hide()">取消</button>
        </ButtonStyled>
      </div>
    </NewModal>

    <!-- 全局分析图表 -->
    <section v-if="!loadingStats && stats" class="universal-card">
      <div class="header-section">
        <h2>平台激励统计</h2>
        <p class="description">实时查看平台激励的累计与今日数据，用于评估预览阶段真实预算。</p>
      </div>

      <!-- 汇总卡 -->
      <div class="summary-grid">
        <div class="summary-card">
          <span class="label">激活项目</span>
          <span class="value">{{ formatNumber(stats.total_projects) }}</span>
          <span class="hint">已开通 {{ stats.total_enabled }}</span>
        </div>
        <div class="summary-card">
          <span class="label">累计有效下载</span>
          <span class="value">{{ formatNumber(stats.total_eff_downloads) }}</span>
        </div>
        <div class="summary-card highlight">
          <span class="label">待结算总额</span>
          <span class="value money pending">¥{{ formatMoney(stats.total_pending) }}</span>
        </div>
        <div class="summary-card">
          <span class="label">已结算总额</span>
          <span class="value money settled">¥{{ formatMoney(stats.total_settled) }}</span>
        </div>
        <div v-if="parseFloat(stats.total_voided) > 0" class="summary-card">
          <span class="label">已作废</span>
          <span class="value money voided">¥{{ formatMoney(stats.total_voided) }}</span>
        </div>
      </div>

      <!-- 今日卡片 -->
      <div class="today-grid">
        <div class="today-card">
          <span class="label">今日有效下载</span>
          <span class="value">{{ formatNumber(stats.today_eff_downloads) }}</span>
        </div>
        <div class="today-card">
          <span class="label">今日产生金额</span>
          <span class="value text-amber">¥{{ formatMoney(stats.today_amount) }}</span>
        </div>
        <div class="today-card">
          <span class="label">今日活跃项目</span>
          <span class="value">{{ formatNumber(stats.today_active_projects) }}</span>
        </div>
      </div>

      <!-- 30 天趋势图 -->
      <div v-if="stats.daily_trend?.length" class="charts-block">
        <h4>近 30 天趋势</h4>
        <div class="chart-grid">
          <client-only>
            <Chart
              name="每日有效下载"
              type="bar"
              :labels="trendLabels"
              :data="trendDownloadsData"
              :colors="['var(--color-brand)']"
              :hide-toolbar="true"
              :hide-legend="true"
            />
          </client-only>
          <client-only>
            <Chart
              name="每日产生金额"
              type="bar"
              :labels="trendLabels"
              :data="trendAmountData"
              :colors="['var(--color-green)']"
              prefix="¥"
              :hide-toolbar="true"
              :hide-legend="true"
            />
          </client-only>
          <client-only>
            <Chart
              name="每日活跃项目数"
              type="area"
              :labels="trendLabels"
              :data="trendProjectsData"
              :colors="['var(--color-blue, #2563eb)']"
              :hide-toolbar="true"
              :hide-legend="true"
            />
          </client-only>
        </div>
      </div>

      <!-- 档位分布 -->
      <div v-if="stats.tier_distribution?.length" class="charts-block">
        <h4>项目档位分布（按累计有效下载）</h4>
        <div class="tier-grid">
          <div v-for="t in stats.tier_distribution" :key="t.tier" class="tier-card">
            <div class="tier-label">{{ formatTierLabel(t.tier) }}</div>
            <div class="tier-count">{{ t.project_count }} 个项目</div>
            <div class="tier-downloads">累计 {{ formatNumber(t.total_downloads) }} 次下载</div>
            <div class="tier-bar">
              <div class="tier-bar-fill" :style="{ width: tierBarWidth(t) }" />
            </div>
          </div>
        </div>
      </div>

      <!-- Top 20 项目 -->
      <div v-if="stats.top_projects?.length" class="charts-block">
        <h4>Top 20 项目（按待结算金额）</h4>
        <div class="top-list">
          <div v-for="(p, idx) in stats.top_projects" :key="p.project_id" class="top-row">
            <span class="rank">#{{ idx + 1 }}</span>
            <nuxt-link
              :to="`/project/${p.slug || p.project_id}`"
              class="title link"
              target="_blank"
            >
              {{ p.title || p.project_id }}
            </nuxt-link>
            <span class="downloads">{{ formatNumber(p.lifetime_eff_downloads) }} 次</span>
            <span class="amount text-amber">¥{{ formatMoney(p.pending_amount) }}</span>
            <div class="bar">
              <div class="bar-fill" :style="{ width: topBarWidth(p) }" />
            </div>
          </div>
        </div>
      </div>
    </section>

    <section class="universal-card">
      <div class="header-section">
        <h2>项目列表</h2>
        <p class="description">
          展示所有有激励数据或已开通激励的项目。只有审核通过并开通激励的资源会继续累计有效下载和待结算金额。
        </p>
      </div>

      <!-- 筛选 -->
      <div class="filter-section">
        <Chips v-model="filterMode" :items="filterOptions" :format-label="formatFilterLabel" />
        <span class="filter-count">{{ filteredItems.length }} 个项目</span>
      </div>

      <!-- 列表 -->
      <div v-if="loading" class="loading-section">
        <UpdatedIcon class="animate-spin" />
        <span>加载中...</span>
      </div>

      <div v-else-if="filteredItems.length > 0" class="projects-list">
        <div
          v-for="item in filteredItems"
          :key="item.project_id"
          class="project-row"
          :class="{ 'is-disabled': !item.enabled }"
        >
          <div class="project-main">
            <div class="title-row">
              <nuxt-link
                :to="`/project/${item.slug || item.project_id}`"
                class="link"
                target="_blank"
              >
                <b>{{ item.title || item.project_id }}</b>
              </nuxt-link>
              <span class="status-badge" :class="item.enabled ? 'enabled' : 'auto'">
                {{ item.enabled ? "已开通" : "未开通" }}
              </span>
              <span v-if="parseFloat(item.pending_amount) > 0" class="pulse-dot"></span>
            </div>

            <div class="meta-row">
              <span v-if="item.slug" class="muted">slug: {{ item.slug }}</span>
              <span v-if="item.enabled_at">开通：{{ formatDateTime(item.enabled_at) }}</span>
              <span v-if="item.last_event_at">
                最后活动：{{ formatRelative(item.last_event_at) }}
              </span>
            </div>

            <div v-if="item.notes" class="notes-row">
              <span class="muted">备注：</span>{{ item.notes }}
            </div>

            <div class="stats-row">
              <div class="stat">
                <span class="stat-label">有效下载</span>
                <span class="stat-value">{{ formatNumber(item.lifetime_eff_downloads) }}</span>
              </div>
              <div class="stat">
                <span class="stat-label">待结算</span>
                <span class="stat-value text-amber">¥{{ formatMoney(item.pending_amount) }}</span>
              </div>
              <div class="stat">
                <span class="stat-label">已结算</span>
                <span class="stat-value text-green">¥{{ formatMoney(item.settled_amount) }}</span>
              </div>
              <div v-if="parseFloat(item.voided_amount) > 0" class="stat">
                <span class="stat-label">已作废</span>
                <span class="stat-value text-red">¥{{ formatMoney(item.voided_amount) }}</span>
              </div>
            </div>
          </div>

          <div class="project-actions">
            <button v-if="item.enabled" class="btn btn-danger" @click="openDisable(item)">
              关闭激励
            </button>
            <button v-else class="btn btn-secondary" @click="openEnable(item)">手动开通</button>
          </div>
        </div>
      </div>

      <div v-else class="empty-section">
        <InfoIcon aria-hidden="true" />
        <p>
          {{
            filterMode === "all"
              ? "暂无项目数据"
              : filterMode === "enabled"
                ? "暂无已开通激励的项目"
                : "暂无未开通激励的项目"
          }}
        </p>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { NewModal, ButtonStyled } from "@modrinth/ui";
import Chips from "~/components/ui/Chips.vue";
import Chart from "~/components/ui/charts/Chart.client.vue";
import InfoIcon from "~/assets/images/utils/info.svg?component";
import UpdatedIcon from "~/assets/images/utils/updated.svg?component";

const auth = await useAuth();
const app = useNuxtApp();

if (auth.value?.user?.role !== "admin") {
  await navigateTo("/");
}

useHead({
  title: "激励项目监控 - BBSMC",
  meta: [{ name: "robots", content: "noindex, nofollow" }],
});

const loading = ref(true);
const loadingStats = ref(true);
const submitting = ref(false);
const items = ref([]);
const stats = ref(null);

const filterMode = ref("all");
const filterOptions = ["all", "enabled", "auto"];
const formatFilterLabel = (v) => ({ all: "全部", enabled: "已开通", auto: "未开通" })[v] || v;

const disableModal = ref(null);
const enableModal = ref(null);
const active = ref(null);
const voidPending = ref(false);
const disableNotes = ref("");
const enableNotes = ref("");

const formatDateTime = (s) =>
  s ? app.$dayjs(s).tz("Asia/Shanghai").format("YYYY-MM-DD HH:mm") : "";
const formatRelative = (s) => (s ? app.$dayjs(s).fromNow() : "");

const formatNumber = (n) => {
  const x = Number(n) || 0;
  return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
};

const formatMoney = (n) => {
  const x = Number(n) || 0;
  return x.toFixed(x < 1 ? 4 : 2).replace(/\B(?=(\d{3})+(?!\d))/g, ",");
};

const filteredItems = computed(() => {
  if (filterMode.value === "enabled") return items.value.filter((i) => i.enabled);
  if (filterMode.value === "auto") return items.value.filter((i) => !i.enabled);
  return items.value;
});

const fetchItems = async () => {
  loading.value = true;
  try {
    const data = await useBaseFetch("admin/incentive/projects", {
      method: "GET",
      internal: true,
    });
    items.value = Array.isArray(data) ? data : [];
  } catch (e) {
    addNotification({
      group: "main",
      title: "加载失败",
      text: e?.data?.description || "无法加载项目列表",
      type: "error",
    });
  } finally {
    loading.value = false;
  }
};

const fetchStats = async () => {
  loadingStats.value = true;
  try {
    stats.value = await useBaseFetch("admin/incentive/stats", {
      method: "GET",
      internal: true,
    });
  } catch (e) {
    console.error("加载 stats 失败", e);
    stats.value = null;
  } finally {
    loadingStats.value = false;
  }
};

// 趋势图数据
const trendLabels = computed(() => stats.value?.daily_trend?.map((d) => d.date) || []);
const trendDownloadsData = computed(() => [
  {
    name: "有效下载",
    data:
      stats.value?.daily_trend?.map((d) => ({
        x: new Date(d.date).getTime(),
        y: d.effective_downloads,
      })) || [],
  },
]);
const trendAmountData = computed(() => [
  {
    name: "金额（元）",
    data:
      stats.value?.daily_trend?.map((d) => ({
        x: new Date(d.date).getTime(),
        y: parseFloat(parseFloat(d.daily_amount || "0").toFixed(2)),
      })) || [],
  },
]);
const trendProjectsData = computed(() => [
  {
    name: "活跃项目",
    data:
      stats.value?.daily_trend?.map((d) => ({
        x: new Date(d.date).getTime(),
        y: d.active_projects,
      })) || [],
  },
]);

// 档位分布
const formatTierLabel = (tier) => {
  const m = {
    "01_<100": "< 100 次",
    "02_100-1K": "100 - 1,000 次",
    "03_1K-1W": "1K - 10K 次",
    "04_1W-10W": "10K - 100K 次",
    "05_>10W": "> 100K 次",
  };
  return m[tier] || tier;
};

const tierBarWidth = (t) => {
  const max = Math.max(...(stats.value?.tier_distribution?.map((x) => x.project_count) || [1]));
  return `${(t.project_count / max) * 100}%`;
};

// Top 项目条形图宽度
const topBarWidth = (p) => {
  const max = parseFloat(stats.value?.top_projects?.[0]?.pending_amount || "1");
  if (max <= 0) return "0%";
  const v = parseFloat(p.pending_amount || "0");
  return `${(v / max) * 100}%`;
};

const openDisable = (item) => {
  active.value = item;
  voidPending.value = false;
  disableNotes.value = "";
  disableModal.value?.show();
};

const openEnable = (item) => {
  active.value = item;
  enableNotes.value = "";
  enableModal.value?.show();
};

const doDisable = async () => {
  if (!active.value || submitting.value) return;
  submitting.value = true;
  try {
    await useBaseFetch(`admin/projects/${active.value.project_id}/incentive`, {
      method: "PATCH",
      internal: true,
      body: {
        enable: false,
        void_pending: voidPending.value,
        notes: disableNotes.value || null,
      },
    });
    addNotification({
      group: "main",
      title: "成功",
      text: voidPending.value ? "激励已关闭，待结算已作废" : "激励已关闭",
      type: "success",
    });
    disableModal.value?.hide();
    await fetchItems();
  } catch (e) {
    addNotification({
      group: "main",
      title: "操作失败",
      text: e?.data?.description || "关闭失败",
      type: "error",
    });
  } finally {
    submitting.value = false;
  }
};

const doEnable = async () => {
  if (!active.value || submitting.value) return;
  submitting.value = true;
  try {
    await useBaseFetch(`admin/projects/${active.value.project_id}/incentive`, {
      method: "PATCH",
      internal: true,
      body: {
        enable: true,
        notes: enableNotes.value || null,
      },
    });
    addNotification({
      group: "main",
      title: "成功",
      text: "激励已开通",
      type: "success",
    });
    enableModal.value?.hide();
    await fetchItems();
  } catch (e) {
    addNotification({
      group: "main",
      title: "操作失败",
      text: e?.data?.description || "开通失败",
      type: "error",
    });
  } finally {
    submitting.value = false;
  }
};

onMounted(() => {
  fetchItems();
  fetchStats();
});
</script>

<style lang="scss" scoped>
.header-section {
  margin-bottom: 1rem;
  h2 {
    margin: 0;
  }
  .description {
    color: var(--color-text-secondary);
    margin: 0.25rem 0 0;
  }
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;

  .summary-card {
    background: var(--color-bg);
    border: 1px solid var(--color-divider);
    border-radius: var(--radius-md);
    padding: 1rem 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    &.highlight {
      border-color: var(--color-orange, #f59e0b);
      background: var(--color-orange-bg, #fef3c7);
    }

    .label {
      color: var(--color-text-secondary);
      font-size: 0.85rem;
    }
    .value {
      font-size: 1.5rem;
      font-weight: 700;
      &.money {
        font-size: 1.35rem;
      }
      &.pending {
        color: var(--color-orange, #d97706);
      }
      &.settled {
        color: var(--color-green, #059669);
      }
      &.voided {
        color: var(--color-red, #dc2626);
      }
    }
    .hint {
      color: var(--color-text-secondary);
      font-size: 0.8rem;
    }
  }
}

.today-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;

  .today-card {
    background: var(--color-raised-bg, var(--color-bg));
    border: 1px solid var(--color-divider);
    border-radius: var(--radius-md);
    padding: 0.75rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    .label {
      color: var(--color-text-secondary);
      font-size: 0.85rem;
    }
    .value {
      font-size: 1.25rem;
      font-weight: 600;
    }
  }
}

.charts-block {
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--color-divider);

  h4 {
    margin-bottom: 1rem;
  }
}

.chart-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 1rem;
}

.tier-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 0.75rem;

  .tier-card {
    background: var(--color-bg);
    border: 1px solid var(--color-divider);
    border-radius: var(--radius-md);
    padding: 0.75rem 1rem;

    .tier-label {
      font-weight: 600;
      margin-bottom: 0.25rem;
    }
    .tier-count {
      font-size: 1.25rem;
      font-weight: 700;
      color: var(--color-brand);
    }
    .tier-downloads {
      color: var(--color-text-secondary);
      font-size: 0.85rem;
      margin-bottom: 0.5rem;
    }
    .tier-bar {
      height: 4px;
      background: var(--color-divider);
      border-radius: 2px;
      overflow: hidden;

      .tier-bar-fill {
        height: 100%;
        background: var(--color-brand);
        border-radius: 2px;
        transition: width 0.3s;
      }
    }
  }
}

.top-list {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;

  .top-row {
    display: grid;
    grid-template-columns: 2.5rem 1fr 8rem 7rem;
    grid-template-rows: auto auto;
    align-items: center;
    gap: 0.5rem 0.75rem;
    padding: 0.5rem 0.75rem;
    background: var(--color-bg);
    border-radius: var(--radius-sm);

    .rank {
      color: var(--color-text-secondary);
      font-weight: 600;
    }
    .title {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .downloads {
      color: var(--color-text-secondary);
      font-size: 0.85rem;
      text-align: right;
    }
    .amount {
      font-weight: 700;
      text-align: right;
    }
    .bar {
      grid-column: 1 / -1;
      height: 3px;
      background: var(--color-divider);
      border-radius: 2px;
      overflow: hidden;

      .bar-fill {
        height: 100%;
        background: linear-gradient(90deg, var(--color-orange, #f59e0b), var(--color-red, #dc2626));
        border-radius: 2px;
        transition: width 0.4s;
      }
    }
  }
}

.filter-section {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
  .filter-count {
    color: var(--color-text-secondary);
    font-size: 0.9rem;
  }
}

.loading-section,
.empty-section {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 3rem;
  color: var(--color-text-secondary);
}

.projects-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.project-row {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem 1.25rem;
  background: var(--color-bg);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-divider);
  transition: border-color 0.15s;

  &:hover {
    border-color: var(--color-button-bg);
  }

  &.is-disabled {
    background: transparent;
  }

  .project-main {
    flex: 1;
    min-width: 0;
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    flex-wrap: wrap;
  }

  .meta-row,
  .notes-row {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    color: var(--color-text-secondary);
    font-size: 0.85rem;
    margin-top: 0.25rem;

    .muted {
      color: var(--color-text-secondary);
    }
  }

  .stats-row {
    display: flex;
    flex-wrap: wrap;
    gap: 1.5rem;
    margin-top: 0.75rem;

    .stat {
      display: flex;
      flex-direction: column;
      gap: 0.125rem;

      .stat-label {
        color: var(--color-text-secondary);
        font-size: 0.8rem;
      }
      .stat-value {
        font-size: 1.05rem;
        font-weight: 600;
      }
    }
  }
}

.status-badge {
  padding: 0.125rem 0.5rem;
  border-radius: var(--radius-sm);
  font-size: 0.85rem;

  &.enabled {
    background: var(--color-green-bg, #d1fae5);
    color: var(--color-green, #059669);
  }
  &.auto {
    background: var(--color-blue-bg, #dbeafe);
    color: var(--color-blue, #2563eb);
  }
}

.pulse-dot {
  width: 0.5rem;
  height: 0.5rem;
  border-radius: 50%;
  background: var(--color-orange, #f59e0b);
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.4;
  }
}

.text-amber {
  color: var(--color-orange, #d97706);
}
.text-green {
  color: var(--color-green, #059669);
}
.text-red {
  color: var(--color-red, #dc2626);
}

.project-actions {
  display: flex;
  gap: 0.5rem;
  align-items: flex-start;
  flex-shrink: 0;
}

.modal-content {
  padding: 0 1rem;
  p {
    margin: 0.5rem 0;
  }
  .hint {
    color: var(--color-text-secondary);
    font-size: 0.9rem;
  }
  code {
    background: var(--color-bg);
    padding: 0.1rem 0.35rem;
    border-radius: var(--radius-sm);
    font-size: 0.85em;
  }
}

.form-group {
  margin: 1rem 0;
  label {
    display: block;
    margin-bottom: 0.5rem;
  }
  textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid var(--color-divider);
    border-radius: var(--radius-sm);
  }
  .checkbox-row {
    display: flex;
    gap: 0.5rem;
    align-items: flex-start;
    cursor: pointer;
    small {
      font-size: 0.85rem;
    }
  }
}

.modal-actions {
  display: flex;
  gap: 0.5rem;
  padding: 1rem;
  justify-content: flex-end;
}
</style>
