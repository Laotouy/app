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
          余额处于<nuxt-link class="text-link" to="/legal/incentive#pending">待结算</nuxt-link
          >状态。
        </p>
      </div>
      <p v-else>
        您已累计获得
        <strong>{{ $formatMoney(userBalance.available) }}</strong
        >，未达到 ¥{{ minWithdraw }}
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
        <nuxt-link to="/legal/incentive" class="text-link">《创作者激励计划协议》</nuxt-link
        >。如需了解激励机制的详细规则，请查看
        <nuxt-link to="/legal/incentive#rules" class="text-link">激励规则</nuxt-link>章节。
      </p>
    </section>

    <!-- 云账户实名 + 支付宝绑定 -->
    <section class="universal-card">
      <h2 class="text-2xl">实名认证 &amp; 收款账号</h2>
      <p class="hint">
        BBSMC
        通过<strong>云账户（天津）共享经济信息咨询有限公司</strong>合规结算激励，提现前需完成实名认证并绑定支付宝收款账号。
        身份证号将加密存储于本地，仅在结算时传给云账户用于实名校验。
      </p>

      <!-- 已完成：展示脱敏信息 -->
      <div v-if="profile.kyc_completed && !editing">
        <ul class="info-list">
          <li>
            <span class="label">真实姓名</span><span>{{ profile.real_name }}</span>
          </li>
          <li>
            <span class="label">身份证号</span
            ><span>**************{{ profile.id_card_last4 }}</span>
          </li>
          <li>
            <span class="label">手机号</span><span>{{ profile.phone_masked }}</span>
          </li>
          <li>
            <span class="label">支付宝账号</span><span>{{ profile.alipay_account_masked }}</span>
          </li>
          <li>
            <span class="label">签约状态</span>
            <span :class="signStatusClass">{{ signStatusLabel }}</span>
          </li>
        </ul>

        <div class="alipay-transfer-tip">
          <strong>支付宝收款提醒</strong>
          <p>
            请确认支付宝账号已允许陌生人通过手机号、邮箱向您转账。打开支付宝首页搜索
            <strong>“隐私”</strong
            >，进入第一个结果，选择<strong>常用隐私设置</strong>，找到并开启“允许通过手机号向我转账”和“允许通过邮箱向我转账”。
          </p>
        </div>

        <div class="input-group mt-4">
          <!-- 未签约 / 已解约：去签约 -->
          <button
            v-if="profile.sign_status === 'unsigned' || profile.sign_status === 'terminated'"
            class="iconified-button brand-button"
            :disabled="signing || hasActivePayout"
            @click="goSign"
          >
            <FileTextIcon />
            {{ hasActivePayout ? "提现处理中，暂不可签约" : signing ? "正在跳转..." : "去签约" }}
          </button>
          <!-- 签约中：刷新状态 -->
          <button
            v-else-if="profile.sign_status === 'signing'"
            class="iconified-button"
            :disabled="signing"
            @click="refreshSign"
          >
            <UpdatedIcon /> {{ signing ? "查询中..." : "我已完成签约，刷新状态" }}
          </button>
          <!-- 已签约：申请解约（生成 H5 解约页面，用户扫码完成手机号/人脸验证） -->
          <button
            v-else
            class="iconified-button"
            :disabled="signing || hasActivePayout"
            @click="goRelease"
          >
            <XIcon />
            {{ hasActivePayout ? "提现处理中，暂不可解约" : signing ? "生成中..." : "申请解约" }}
          </button>

          <button class="iconified-button" :disabled="hasActivePayout" @click="startEdit">
            <EditIcon /> 编辑实名信息
          </button>
        </div>

        <p v-if="hasActivePayout" class="warn">
          当前有提现订单正在处理中。提现完成或由管理员退回前，不能修改签约状态或发起解约。
        </p>

        <p v-if="signError" class="invalid">{{ signError }}</p>

        <p class="warn">
          ⚠️
          修改实名信息后需要<strong>重新签约</strong>。云账户要求签约时填写的姓名与身份证号在结算阶段保持一致，否则会被风控拦截。
        </p>
      </div>

      <!-- 未完成或正在编辑：展示表单 -->
      <form v-else class="kyc-form" @submit.prevent="submitProfile">
        <label>
          <span>真实姓名 <em>*</em></span>
          <input
            v-model="form.real_name"
            type="text"
            placeholder="请填写身份证上的姓名"
            maxlength="30"
          />
        </label>
        <label>
          <span>身份证号 <em>*</em></span>
          <input
            v-model="form.id_card"
            type="text"
            placeholder="18 位身份证号，末位 X 请大写"
            maxlength="18"
          />
        </label>
        <label>
          <span>手机号 <em>*</em></span>
          <input
            v-model="form.phone"
            type="text"
            placeholder="11 位手机号"
            maxlength="11"
            inputmode="numeric"
          />
        </label>
        <label>
          <span>支付宝账号 <em>*</em></span>
          <input
            v-model="form.alipay_account"
            type="text"
            placeholder="支付宝账号（手机号或邮箱）"
          />
        </label>

        <p v-if="submitError" class="invalid">{{ submitError }}</p>

        <div class="input-group">
          <button class="iconified-button brand-button" type="submit" :disabled="submitting">
            <SaveIcon /> {{ submitting ? "保存中..." : "保存信息" }}
          </button>
          <button v-if="editing" type="button" class="iconified-button" @click="cancelEdit">
            <XIcon /> 取消
          </button>
        </div>
      </form>
    </section>

    <!-- 解约确认弹窗 -->
    <ConfirmModal
      ref="releaseConfirmRef"
      title="确认申请解约？"
      description="解约后将无法提现，需重新签约才能恢复。"
      proceed-label="发起解约"
      @proceed="doRelease"
    />

    <!-- 签约 / 解约 二维码弹窗（复用） -->
    <NewModal
      ref="signModalRef"
      :header="signModalMode === 'release' ? '扫码完成解约验证' : '扫码完成实名签约'"
      :on-hide="onSignModalHide"
    >
      <div class="sign-modal-body">
        <p class="sign-modal-hint">
          请使用<strong>微信「扫一扫」</strong>扫描下方二维码，进入云账户 H5 页面
          {{
            signModalMode === "release" ? "完成手机号 / 人脸识别验证，确认解约" : "完成实名签约"
          }}。
        </p>
        <div class="qr-wrapper">
          <QrcodeVue
            v-if="signUrl"
            :value="signUrl"
            :size="240"
            level="M"
            background="#ffffff"
            foreground="#000000"
            margin="3"
          />
        </div>
        <p class="sign-modal-hint">
          {{
            signModalMode === "release"
              ? "解约完成后会跳转到结果提示页；也可以点击「我已完成解约」同步状态。"
              : "签约完成后会跳转到结果提示页；也可以点击「我已完成签约」同步状态。"
          }}
        </p>
        <details class="sign-url-details">
          <summary>无法扫码？点击展开链接</summary>
          <code class="sign-url">{{ signUrl }}</code>
          <button type="button" class="iconified-button" @click="copySignUrl">
            <SaveIcon /> 复制链接
          </button>
        </details>
      </div>
      <template #actions>
        <div class="input-group">
          <button class="iconified-button" @click="refreshSign">
            <UpdatedIcon />
            {{ signModalMode === "release" ? "我已完成解约" : "我已完成签约" }}
          </button>
          <button class="iconified-button" @click="closeSignModal"><XIcon /> 关闭</button>
        </div>
      </template>
    </NewModal>
  </div>
</template>

<script setup>
import {
  TransferIcon,
  HistoryIcon,
  EditIcon,
  SaveIcon,
  XIcon,
  FileTextIcon,
  UpdatedIcon,
} from "@modrinth/assets";
import { NewModal, ConfirmModal } from "@modrinth/ui";
import QrcodeVue from "qrcode.vue";

useHead({
  title: "收益 - BBSMC",
  meta: [{ name: "robots", content: "noindex, nofollow" }],
});

const minWithdraw = ref(5);
const data = useNuxtApp();

const [{ data: userBalance }, { data: profile, refresh: refreshProfile }] = await Promise.all([
  useAsyncData("payout/balance", () => useBaseFetch("payout/balance", { apiVersion: 3 })),
  useAsyncData("yunzhanghu/profile", () => useBaseFetch("yunzhanghu/profile", { apiVersion: 3 })),
]);

const editing = ref(false);
const submitting = ref(false);
const submitError = ref("");
const signing = ref(false);
const signError = ref("");
const signModalRef = ref(null);
const releaseConfirmRef = ref(null);
const signUrl = ref("");
const signModalMode = ref("sign"); // "sign" | "release"

const route = useRoute();
const router = useRouter();

// 用户从云账户 H5 签约回跳时携带 ?sign=ok 或 ?sign=fail
onMounted(async () => {
  const result = route.query.sign;
  if (result === "ok") {
    await refreshProfile();
    data.$notify({
      group: "main",
      title: "签约完成",
      text: "您已完成实名签约，现在可以提现了！",
      type: "success",
    });
    router.replace({ query: {} });
  } else if (result === "fail") {
    data.$notify({
      group: "main",
      title: "签约未完成",
      text: "签约流程未走完，可以稍后再次发起签约。",
      type: "error",
    });
    router.replace({ query: {} });
  }
});
const form = reactive({
  real_name: "",
  id_card: "",
  phone: "",
  alipay_account: "",
});

const signStatusMap = {
  unsigned: { label: "未签约", cls: "status-unsigned" },
  signing: { label: "签约中", cls: "status-signing" },
  signed: { label: "已签约", cls: "status-signed" },
  terminated: { label: "已解约", cls: "status-terminated" },
};
const signStatusLabel = computed(
  () => signStatusMap[profile.value?.sign_status]?.label ?? "未签约",
);
const signStatusClass = computed(
  () => signStatusMap[profile.value?.sign_status]?.cls ?? "status-unsigned",
);
const hasActivePayout = computed(() => !!profile.value?.has_active_payout);

function startEdit() {
  if (hasActivePayout.value) {
    submitError.value = "当前有提现订单正在处理中，暂不能修改实名资料或收款账号。";
    return;
  }
  form.real_name = "";
  form.id_card = "";
  form.phone = "";
  form.alipay_account = "";
  submitError.value = "";
  editing.value = true;
}

function cancelEdit() {
  editing.value = false;
  submitError.value = "";
}

async function submitProfile() {
  submitError.value = "";
  if (!form.real_name || !form.id_card || !form.phone || !form.alipay_account) {
    submitError.value = "请完整填写所有必填项";
    return;
  }
  submitting.value = true;
  try {
    await useBaseFetch("yunzhanghu/profile", {
      method: "POST",
      body: { ...form },
      apiVersion: 3,
    });
    await refreshProfile();
    editing.value = false;
    data.$notify({
      group: "main",
      title: "保存成功",
      text: "实名信息已加密保存，下一步去签约。",
      type: "success",
    });
  } catch (err) {
    submitError.value = err?.data?.description || err?.message || "保存失败";
  } finally {
    submitting.value = false;
  }
}

async function goSign() {
  signError.value = "";
  if (hasActivePayout.value) {
    signError.value = "当前有提现订单正在处理中，暂不能发起签约。";
    data.$notify({
      group: "main",
      title: "暂不能签约",
      text: signError.value,
      type: "error",
    });
    return;
  }
  signing.value = true;
  try {
    const resp = await useBaseFetch("yunzhanghu/sign", {
      method: "POST",
      apiVersion: 3,
    });
    if (!resp?.url) {
      throw new Error("云账户未返回签约 URL");
    }
    signUrl.value = resp.url;
    signModalMode.value = "sign";
    await refreshProfile();
    signModalRef.value?.show();
  } catch (err) {
    signError.value = err?.data?.description || err?.message || "发起签约失败";
  } finally {
    signing.value = false;
  }
}

// 点击「申请解约」按钮 → 弹出确认弹窗
function goRelease() {
  if (hasActivePayout.value) {
    signError.value = "当前有提现订单正在处理中，暂不能发起解约。";
    data.$notify({
      group: "main",
      title: "暂不能解约",
      text: signError.value,
      type: "error",
    });
    return;
  }
  releaseConfirmRef.value?.show();
}

// 用户在 ConfirmModal 上点「发起解约」后真实发起请求
async function doRelease() {
  signError.value = "";
  if (hasActivePayout.value) {
    signError.value = "当前有提现订单正在处理中，暂不能发起解约。";
    data.$notify({
      group: "main",
      title: "暂不能解约",
      text: signError.value,
      type: "error",
    });
    return;
  }
  signing.value = true;
  try {
    const resp = await useBaseFetch("yunzhanghu/sign/release", {
      method: "POST",
      apiVersion: 3,
    });
    if (!resp?.url) {
      throw new Error("云账户未返回解约 URL");
    }
    signUrl.value = resp.url;
    signModalMode.value = "release";
    signModalRef.value?.show();
  } catch (err) {
    signError.value = err?.data?.description || err?.message || "发起解约失败";
    data.$notify({
      group: "main",
      title: "发起解约失败",
      text: signError.value,
      type: "error",
    });
  } finally {
    signing.value = false;
  }
}

function closeSignModal() {
  signModalRef.value?.hide();
}

function onSignModalHide() {
  // 关闭弹窗后清空 URL（避免缓存残留），下次点"去签约"会重新生成
  signUrl.value = "";
}

async function copySignUrl() {
  if (!signUrl.value) return;
  try {
    await navigator.clipboard.writeText(signUrl.value);
    data.$notify({
      group: "main",
      title: "签约链接已复制",
      text: "可在其他设备粘贴打开。",
      type: "success",
    });
  } catch {
    // 浏览器禁用 clipboard 时降级：选中文本让用户手动复制
    const el = document.querySelector(".sign-url");
    if (el) {
      const range = document.createRange();
      range.selectNodeContents(el);
      window.getSelection()?.removeAllRanges();
      window.getSelection()?.addRange(range);
    }
  }
}

async function refreshSign() {
  signError.value = "";
  signing.value = true;
  try {
    const resp = await useBaseFetch("yunzhanghu/sign/refresh", {
      method: "POST",
      apiVersion: 3,
    });
    await refreshProfile();
    const status = resp?.sign_status ?? profile.value?.sign_status;
    const mode = signModalMode.value;

    if (mode === "release") {
      // 解约模式
      if (status === "terminated") {
        signModalRef.value?.hide();
        data.$notify({
          group: "main",
          title: "解约成功",
          text: "您已完成解约。如需提现请重新签约。",
          type: "success",
        });
      } else if (status === "signed") {
        data.$notify({
          group: "main",
          title: "尚未完成解约",
          text: "云账户暂未收到解约完成事件，请稍候再试。",
          type: "warn",
        });
      } else {
        data.$notify({
          group: "main",
          title: "状态变更",
          text: `当前云账户状态：${status}`,
          type: "warn",
        });
      }
    } else if (status === "signed") {
      // 签约模式
      signModalRef.value?.hide();
      data.$notify({
        group: "main",
        title: "签约成功",
        text: "您已完成实名签约，可以开始提现了。",
        type: "success",
      });
    } else if (status === "terminated") {
      data.$notify({
        group: "main",
        title: "用户已解约",
        text: "云账户显示当前为已解约状态，请重新签约。",
        type: "error",
      });
    } else {
      data.$notify({
        group: "main",
        title: "尚未完成签约",
        text: "云账户暂未收到您的签约完成事件，请稍候再试。",
        type: "warn",
      });
    }
  } catch (err) {
    signError.value = err?.data?.description || err?.message || "刷新签约状态失败";
    data.$notify({
      group: "main",
      title: "刷新失败",
      text: signError.value,
      type: "error",
    });
  } finally {
    signing.value = false;
  }
}
</script>

<style lang="scss" scoped>
strong {
  color: var(--color-text-dark);
  font-weight: 500;
}

.hint {
  color: var(--color-text);
  font-size: var(--font-size-sm);
}

.info-list {
  list-style: none;
  margin: 0;
  padding: 0;

  li {
    display: flex;
    align-items: center;
    padding: var(--gap-sm) 0;
    border-bottom: 1px solid var(--color-divider);

    .label {
      width: 8rem;
      color: var(--color-text);
    }
  }
}

.alipay-transfer-tip {
  margin-top: var(--gap-md);
  padding: var(--gap-sm) var(--gap-md);
  border-left: 3px solid var(--color-brand);
  border-radius: var(--radius-sm);
  background-color: var(--color-bg);
  font-size: var(--font-size-sm);

  p {
    margin: var(--gap-xs) 0 0;
    color: var(--color-text);
  }
}

.status-unsigned,
.status-terminated {
  color: var(--color-red);
}
.status-signing {
  color: var(--color-orange);
}
.status-signed {
  color: var(--color-green);
}

.kyc-form {
  display: flex;
  flex-direction: column;
  gap: var(--gap-md);

  label {
    display: flex;
    flex-direction: column;
    gap: var(--gap-xs);

    span em {
      color: var(--color-red);
      font-style: normal;
      margin-left: 0.25rem;
    }

    input {
      max-width: 24rem;
    }
  }
}

.invalid {
  color: var(--color-red);
}

.warn {
  margin-top: var(--gap-md);
  padding: var(--gap-sm) var(--gap-md);
  background-color: var(--color-warning-bg, rgba(255, 165, 0, 0.1));
  border-left: 3px solid var(--color-orange);
  font-size: var(--font-size-sm);
}

// 签约弹窗
.sign-modal-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--gap-md);
  text-align: center;
  max-width: 400px;
}

.sign-modal-hint {
  font-size: var(--font-size-sm);
  color: var(--color-text);
  margin: 0;
}

.qr-wrapper {
  padding: var(--gap-md);
  background-color: #ffffff;
  border-radius: var(--radius-md);
  display: flex;
  justify-content: center;
}

.sign-url-details {
  width: 100%;
  text-align: left;

  summary {
    cursor: pointer;
    color: var(--color-text);
    font-size: var(--font-size-sm);
    user-select: none;
  }

  .sign-url {
    display: block;
    padding: var(--gap-sm);
    margin: var(--gap-sm) 0;
    background-color: var(--color-bg);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    word-break: break-all;
    user-select: all;
  }
}
</style>
