<template>
  <div>
    <!-- 审核弹窗 -->
    <NewModal ref="reviewModal">
      <template #title>
        <div class="truncate text-lg font-extrabold text-contrast">审核资料修改</div>
      </template>
      <div class="review-content">
        <div v-if="currentReview" class="review-summary">
          <div class="summary-row">
            <span class="label">用户：</span>
            <nuxt-link :to="`/user/${currentReview.username}`" class="user-link">
              <Avatar
                :src="currentReview.avatar_url"
                :alt="currentReview.username"
                size="xs"
                circle
              />
              <span>{{ currentReview.username }}</span>
            </nuxt-link>
          </div>
          <div class="summary-row">
            <span class="label">修改类型：</span>
            <span class="type-badge" :class="`type-${currentReview.review_type}`">
              {{ getTypeName(currentReview.review_type) }}
            </span>
          </div>
          <div class="summary-row">
            <span class="label">风控标签：</span>
            <span class="risk-labels">{{ currentReview.risk_labels }}</span>
          </div>

          <!-- 内容对比 -->
          <div class="diff-section">
            <template v-if="currentReview.review_type === 'avatar'">
              <div class="avatar-diff">
                <div class="diff-item">
                  <span class="diff-label">当前头像</span>
                  <Avatar
                    :src="getAvatarUrl(currentReview.old_value, 'avatar_url')"
                    size="md"
                    circle
                    alt="当前头像"
                  />
                </div>
                <span class="diff-arrow">&rarr;</span>
                <div class="diff-item">
                  <span class="diff-label">新头像</span>
                  <Avatar
                    :src="getAvatarUrl(currentReview.new_value, 'avatar_url')"
                    size="md"
                    circle
                    alt="新头像"
                  />
                </div>
              </div>
            </template>
            <template v-else>
              <div class="text-diff">
                <div class="diff-item">
                  <span class="diff-label">旧值</span>
                  <div class="diff-value old">{{ currentReview.old_value || "(空)" }}</div>
                </div>
                <div class="diff-item">
                  <span class="diff-label">新值</span>
                  <div class="diff-value new">{{ currentReview.new_value }}</div>
                </div>
              </div>
            </template>
          </div>
        </div>

        <div class="review-form">
          <label class="form-label">
            <span>审核决定</span>
            <span class="required">*</span>
          </label>
          <div class="decision-buttons">
            <button
              class="decision-btn approve"
              :class="{ active: reviewDecision === 'approved' }"
              @click="reviewDecision = 'approved'"
            >
              <CheckIcon aria-hidden="true" />
              批准修改
            </button>
            <button
              class="decision-btn reject"
              :class="{ active: reviewDecision === 'rejected' }"
              @click="reviewDecision = 'rejected'"
            >
              <CrossIcon aria-hidden="true" />
              拒绝修改
            </button>
          </div>

          <label class="form-label">
            <span>审核备注</span>
            <span class="optional">（可选，将通知用户）</span>
          </label>
          <textarea
            v-model="reviewNotes"
            class="review-textarea"
            placeholder="请输入审核备注..."
            rows="3"
          ></textarea>
        </div>
      </div>
      <div class="modal-actions">
        <ButtonStyled
          :color="
            reviewDecision === 'approved'
              ? 'primary'
              : reviewDecision === 'rejected'
                ? 'danger'
                : 'default'
          "
        >
          <button :disabled="!reviewDecision || submitting" @click="submitReview">
            <CheckIcon v-if="reviewDecision === 'approved'" aria-hidden="true" />
            <CrossIcon v-else-if="reviewDecision === 'rejected'" aria-hidden="true" />
            {{
              reviewDecision === "approved"
                ? "确认批准"
                : reviewDecision === "rejected"
                  ? "确认拒绝"
                  : "请选择审核决定"
            }}
          </button>
        </ButtonStyled>
        <ButtonStyled>
          <button @click="reviewModal?.hide()">取消</button>
        </ButtonStyled>
      </div>
    </NewModal>

    <section class="universal-card">
      <div class="header-section">
        <h2>资料审核管理</h2>
        <p class="description">管理用户触发风控的资料修改请求</p>
      </div>

      <!-- 筛选栏 -->
      <div class="filter-section">
        <Chips v-model="statusFilter" :items="statusOptions" :format-label="formatStatusLabel" />
      </div>

      <!-- 加载中 -->
      <div v-if="loading" class="loading-section">
        <UpdatedIcon aria-hidden="true" class="animate-spin" />
        <span>加载中...</span>
      </div>

      <!-- 审核列表 -->
      <div v-else-if="reviews.length > 0" class="reviews-container">
        <div class="reviews-list">
          <div v-for="review in reviews" :key="review.id" class="review-item">
            <div class="review-main">
              <div class="review-header">
                <div class="user-info">
                  <nuxt-link :to="`/user/${review.username}`" class="user-link">
                    <Avatar :src="review.avatar_url" :alt="review.username" size="sm" circle />
                    <span class="username">{{ review.username }}</span>
                  </nuxt-link>
                </div>
                <div class="review-badges">
                  <span class="type-badge" :class="`type-${review.review_type}`">
                    {{ getTypeName(review.review_type) }}
                  </span>
                  <span class="status-badge" :class="`status-${review.status}`">
                    {{ formatStatusLabel(review.status) }}
                  </span>
                </div>
              </div>

              <!-- 内容预览 -->
              <div class="review-preview">
                <template v-if="review.review_type === 'avatar'">
                  <div class="avatar-preview">
                    <Avatar
                      :src="getAvatarUrl(review.old_value, 'avatar_url')"
                      size="xs"
                      circle
                      alt="旧"
                    />
                    <span class="preview-arrow">&rarr;</span>
                    <Avatar
                      :src="getAvatarUrl(review.new_value, 'avatar_url')"
                      size="xs"
                      circle
                      alt="新"
                    />
                  </div>
                </template>
                <template v-else>
                  <div class="text-preview">
                    <span class="old-text">{{ truncate(review.old_value || "(空)", 30) }}</span>
                    <span class="preview-arrow">&rarr;</span>
                    <span class="new-text">{{ truncate(review.new_value, 30) }}</span>
                  </div>
                </template>
              </div>

              <div class="review-meta">
                <span class="risk-labels">{{ review.risk_labels }}</span>
                <span class="meta-time">{{ formatDateTime(review.created_at) }}</span>
              </div>

              <div v-if="review.review_notes" class="review-notes-display">
                <span class="label">审核备注：</span>
                <span>{{ review.review_notes }}</span>
              </div>
            </div>

            <div v-if="review.status === 'pending'" class="review-actions">
              <button class="btn btn-primary" @click="openReviewModal(review)">
                <EditIcon aria-hidden="true" />
                审核
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-else class="empty-section">
        <InfoIcon aria-hidden="true" />
        <p>
          {{
            statusFilter === "all" ? "暂无审核记录" : `暂无${formatStatusLabel(statusFilter)}的审核`
          }}
        </p>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from "vue";
import { NewModal, ButtonStyled } from "@modrinth/ui";
import Avatar from "~/components/ui/Avatar.vue";
import Chips from "~/components/ui/Chips.vue";
import CheckIcon from "~/assets/images/utils/check.svg?component";
import CrossIcon from "~/assets/images/utils/x.svg?component";
import InfoIcon from "~/assets/images/utils/info.svg?component";
import UpdatedIcon from "~/assets/images/utils/updated.svg?component";
import EditIcon from "~/assets/images/utils/edit.svg?component";

const auth = await useAuth();
const app = useNuxtApp();

// 权限守卫：仅管理员/版主可访问
if (!["admin", "moderator"].includes(auth.value?.user?.role)) {
  await navigateTo("/");
}

useHead({
  title: "资料审核管理 - BBSMC",
});

const loading = ref(true);
const reviews = ref([]);
const statusFilter = ref("pending");
const statusOptions = ["all", "pending", "approved", "rejected", "cancelled"];

const reviewModal = ref(null);
const currentReview = ref(null);
const reviewDecision = ref("");
const reviewNotes = ref("");
const submitting = ref(false);

const getTypeName = (type) => {
  const types = {
    avatar: "头像",
    username: "用户名",
    bio: "简介",
  };
  return types[type] || type;
};

const formatStatusLabel = (status) => {
  const labels = {
    all: "全部",
    pending: "待审核",
    approved: "已批准",
    rejected: "已拒绝",
    cancelled: "已撤销",
  };
  return labels[status] || status;
};

const formatDateTime = (date) => {
  return app.$dayjs(date).format("YYYY-MM-DD HH:mm");
};

const truncate = (text, maxLen) => {
  if (!text) return "";
  return text.length > maxLen ? text.substring(0, maxLen) + "..." : text;
};

const getAvatarUrl = (jsonStr, field) => {
  try {
    const obj = JSON.parse(jsonStr);
    return obj[field] || null;
  } catch {
    return null;
  }
};

const fetchReviews = async () => {
  loading.value = true;
  try {
    const params = { count: 100 };
    if (statusFilter.value !== "all") {
      params.status = statusFilter.value;
    }
    const response = await useBaseFetch("moderation/profile-reviews", {
      method: "GET",
      params,
      internal: true,
    });
    if (response && Array.isArray(response)) {
      reviews.value = response;
    }
  } catch (error) {
    console.error("加载审核列表失败:", error);
    addNotification({
      group: "main",
      title: "加载失败",
      text: "无法加载审核列表",
      type: "error",
    });
  }
  loading.value = false;
};

const openReviewModal = (review) => {
  currentReview.value = review;
  reviewDecision.value = "";
  reviewNotes.value = "";
  reviewModal.value?.show();
};

const submitReview = async () => {
  if (!currentReview.value || !reviewDecision.value) return;
  submitting.value = true;
  try {
    const action = reviewDecision.value === "approved" ? "approve" : "reject";
    await useBaseFetch(`moderation/profile-reviews/${currentReview.value.id}/${action}`, {
      method: "POST",
      body: { notes: reviewNotes.value || null },
      internal: true,
    });
    addNotification({
      group: "main",
      title: "审核成功",
      text: `已${reviewDecision.value === "approved" ? "批准" : "拒绝"}该资料修改`,
      type: "success",
    });
    reviewModal.value?.hide();
    await fetchReviews();
  } catch (error) {
    console.error("提交审核失败:", error);
    addNotification({
      group: "main",
      title: "审核失败",
      text: error?.data?.description || "操作失败，请重试",
      type: "error",
    });
  } finally {
    submitting.value = false;
  }
};

watch(statusFilter, () => {
  fetchReviews();
});

onMounted(() => {
  fetchReviews();
});
</script>

<style lang="scss" scoped>
.header-section {
  margin-bottom: 1rem;

  h2 {
    margin: 0 0 0.25rem;
    font-size: 1.5rem;
  }

  .description {
    color: var(--color-text-secondary);
    margin: 0;
  }
}

.filter-section {
  margin-bottom: 1rem;
}

.loading-section {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 3rem;
  color: var(--color-text-secondary);
}

.reviews-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.review-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 1rem;
  border: 1px solid var(--color-button-bg);
  border-radius: var(--radius-lg);
  background: var(--color-raised-bg);
  gap: 1rem;
}

.review-main {
  flex: 1;
  min-width: 0;
}

.review-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.user-info {
  display: flex;
  align-items: center;
}

.user-link {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  text-decoration: none;
  color: var(--color-text);

  &:hover {
    color: var(--color-brand);
  }
}

.username {
  font-weight: 600;
}

.review-badges {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.type-badge {
  padding: 0.125rem 0.5rem;
  border-radius: var(--radius-md);
  font-size: 0.75rem;
  font-weight: 600;

  &.type-avatar {
    background: var(--color-brand-highlight);
    color: var(--color-brand);
  }

  &.type-username {
    background: rgba(59, 130, 246, 0.1);
    color: rgb(59, 130, 246);
  }

  &.type-bio {
    background: rgba(16, 185, 129, 0.1);
    color: rgb(16, 185, 129);
  }
}

.status-badge {
  padding: 0.125rem 0.5rem;
  border-radius: var(--radius-md);
  font-size: 0.75rem;
  font-weight: 600;

  &.status-pending {
    background: rgba(245, 158, 11, 0.15);
    color: rgb(245, 158, 11);
  }

  &.status-approved {
    background: rgba(34, 197, 94, 0.15);
    color: rgb(34, 197, 94);
  }

  &.status-rejected {
    background: rgba(239, 68, 68, 0.15);
    color: rgb(239, 68, 68);
  }

  &.status-cancelled {
    background: rgba(107, 114, 128, 0.15);
    color: rgb(107, 114, 128);
  }
}

.review-preview {
  margin-bottom: 0.5rem;
}

.avatar-preview {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.text-preview {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.old-text {
  color: var(--color-text-secondary);
  text-decoration: line-through;
}

.new-text {
  color: var(--color-text);
  font-weight: 500;
}

.preview-arrow {
  color: var(--color-text-secondary);
}

.review-meta {
  display: flex;
  align-items: center;
  gap: 1rem;
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.risk-labels {
  color: rgb(239, 68, 68);
  font-size: 0.75rem;
}

.review-notes-display {
  margin-top: 0.5rem;
  font-size: 0.875rem;
  color: var(--color-text-secondary);

  .label {
    font-weight: 600;
  }
}

.review-actions {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.empty-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 3rem;
  color: var(--color-text-secondary);

  svg {
    width: 2rem;
    height: 2rem;
  }
}

// 审核弹窗样式
.review-content {
  padding: 1rem;
  min-width: 500px;
}

.review-summary {
  margin-bottom: 1.5rem;
}

.summary-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;

  .label {
    font-weight: 600;
    color: var(--color-text-secondary);
    min-width: 5rem;
  }
}

.diff-section {
  margin-top: 1rem;
  padding: 1rem;
  border: 1px solid var(--color-button-bg);
  border-radius: var(--radius-lg);
  background: var(--color-bg);
}

.avatar-diff {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2rem;
}

.diff-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.diff-label {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  font-weight: 600;
}

.diff-arrow {
  font-size: 1.5rem;
  color: var(--color-text-secondary);
}

.text-diff {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.diff-value {
  padding: 0.5rem 0.75rem;
  border-radius: var(--radius-md);
  font-size: 0.875rem;
  word-break: break-all;

  &.old {
    background: rgba(239, 68, 68, 0.05);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: var(--color-text-secondary);
  }

  &.new {
    background: rgba(34, 197, 94, 0.05);
    border: 1px solid rgba(34, 197, 94, 0.2);
    color: var(--color-text);
  }
}

.review-form {
  .form-label {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    margin-bottom: 0.5rem;
    font-weight: 600;

    .required {
      color: rgb(239, 68, 68);
    }

    .optional {
      font-weight: 400;
      color: var(--color-text-secondary);
      font-size: 0.875rem;
    }
  }
}

.decision-buttons {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.decision-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 2px solid var(--color-button-bg);
  border-radius: var(--radius-lg);
  background: var(--color-raised-bg);
  color: var(--color-text);
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;

  svg {
    width: 1rem;
    height: 1rem;
  }

  &:hover {
    border-color: var(--color-text-secondary);
  }

  &.approve.active {
    border-color: rgb(34, 197, 94);
    background: rgba(34, 197, 94, 0.1);
    color: rgb(34, 197, 94);
  }

  &.reject.active {
    border-color: rgb(239, 68, 68);
    background: rgba(239, 68, 68, 0.1);
    color: rgb(239, 68, 68);
  }
}

.review-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-button-bg);
  border-radius: var(--radius-md);
  background: var(--color-bg);
  color: var(--color-text);
  resize: vertical;
  font-family: inherit;
  margin-bottom: 1rem;

  &:focus {
    outline: none;
    border-color: var(--color-brand);
  }
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 0 1rem 1rem;
}
</style>
