<template>
  <NuxtLayout>
    <div class="error-page">
      <div class="error-content">
        <!-- Error Code Display -->
        <div class="error-code">
          <span class="code-number">{{ error.statusCode || '404' }}</span>
        </div>

        <!-- Error Icon -->
        <div class="error-icon">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" />
          </svg>
        </div>

        <!-- Error Message -->
        <h1 class="error-title">
          {{ error.statusCode === 404 ? '页面走丢了' : '出错了' }}
        </h1>
        <p class="error-message">
          {{ error.statusCode === 404
            ? '抱歉，您访问的页面不存在或已被移除'
            : error.message || '发生了未知错误' }}
        </p>

        <!-- Action Buttons -->
        <div class="error-actions">
          <NuxtLink to="/" class="btn-primary">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
            </svg>
            返回首页
          </NuxtLink>
          <button @click="goBack" class="btn-secondary">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3" />
            </svg>
            返回上页
          </button>
        </div>

        <!-- Quick Links -->
        <div class="quick-links">
          <p class="quick-links-title">或者探索这些内容：</p>
          <div class="links-grid">
            <NuxtLink to="/mods" class="quick-link">
              <span class="link-icon"><BoxIcon /></span>
              <span>模组</span>
            </NuxtLink>
            <NuxtLink to="/modpacks" class="quick-link">
              <span class="link-icon"><PackageClosedIcon /></span>
              <span>整合包</span>
            </NuxtLink>
            <NuxtLink to="/shaders" class="quick-link">
              <span class="link-icon"><GlassesIcon /></span>
              <span>光影</span>
            </NuxtLink>
            <NuxtLink to="/forums/chat" class="quick-link">
              <span class="link-icon"><MessageIcon /></span>
              <span>论坛</span>
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup>
import {
  BoxIcon,
  PackageClosedIcon,
  GlassesIcon,
  MessageIcon,
} from "@modrinth/assets";

defineProps({
  error: {
    type: Object,
    default() {
      return {
        statusCode: 404,
        message: "页面不存在",
      };
    },
  },
});

const goBack = () => {
  if (window.history.length > 1) {
    window.history.back();
  } else {
    navigateTo('/');
  }
};
</script>

<style lang="scss" scoped>
.error-page {
  min-height: calc(100vh - 200px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
}

.error-content {
  text-align: center;
  max-width: 480px;
  width: 100%;
}

.error-code {
  margin-bottom: 24px;

  .code-number {
    font-size: 120px;
    font-weight: 800;
    line-height: 1;
    background: linear-gradient(135deg, var(--flame, #f16436) 0%, #ff8a5c 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    opacity: 0.9;

    @media (max-width: 480px) {
      font-size: 80px;
    }
  }
}

.error-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 24px;
  padding: 16px;
  background: var(--accent-muted, rgba(241, 100, 54, 0.1));
  border-radius: 50%;

  svg {
    width: 100%;
    height: 100%;
    color: var(--flame, #f16436);
  }
}

.error-title {
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-dark, var(--color-text));
  margin: 0 0 12px;
}

.error-message {
  font-size: 16px;
  color: var(--color-secondary);
  margin: 0 0 32px;
  line-height: 1.6;
}

.error-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  flex-wrap: wrap;
  margin-bottom: 48px;
}

.btn-primary,
.btn-secondary {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border-radius: 12px;
  font-size: 15px;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.2s ease;
  cursor: pointer;

  svg {
    width: 18px;
    height: 18px;
  }
}

.btn-primary {
  background: var(--flame, #f16436);
  color: white;
  border: none;
  box-shadow: 0 4px 12px rgba(241, 100, 54, 0.3);

  &:hover {
    background: #e55a2d;
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(241, 100, 54, 0.4);
  }
}

.btn-secondary {
  background: var(--color-button-bg);
  color: var(--color-text);
  border: 1px solid var(--color-divider);

  &:hover {
    background: var(--color-raised-bg);
    border-color: var(--flame, #f16436);
    color: var(--flame, #f16436);
  }
}

.quick-links {
  padding-top: 32px;
  border-top: 1px solid var(--color-divider);
}

.quick-links-title {
  font-size: 14px;
  color: var(--color-secondary);
  margin: 0 0 16px;
}

.links-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;

  @media (max-width: 480px) {
    grid-template-columns: repeat(2, 1fr);
  }
}

.quick-link {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 16px 12px;
  background: var(--color-button-bg);
  border: 1px solid var(--color-divider);
  border-radius: 12px;
  text-decoration: none;
  color: var(--color-text);
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s ease;

  .link-icon {
    display: flex;
    align-items: center;
    justify-content: center;

    svg {
      width: 1.5rem;
      height: 1.5rem;
      color: var(--flame, #f16436);
    }
  }

  &:hover {
    background: var(--accent-muted, rgba(241, 100, 54, 0.08));
    border-color: var(--flame, #f16436);
    color: var(--flame, #f16436);
    transform: translateY(-2px);

    .link-icon svg {
      transform: scale(1.1);
    }
  }
}
</style>
