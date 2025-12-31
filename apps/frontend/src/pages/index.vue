<template>
  <div class="home-page">
    <!-- Hero Section -->
    <section class="hero">
      <div class="hero-content">
        <h1 class="hero-title">
          <span class="hero-title-line">发现最好的</span>
          <span class="hero-title-line"><span class="hero-title-highlight">Minecraft</span></span>
          <span class="hero-title-line">中文资源</span>
        </h1>

        <p class="hero-desc">
          探索数十万个模组、整合包、光影和资源包。下载、分享、与百万玩家共建最活跃的中文 MC 社区。
        </p>

        <div class="hero-actions">
          <NuxtLink to="/mods" class="hero-btn hero-btn-primary">开始探索</NuxtLink>
          <NuxtLink to="/dashboard/projects" class="hero-btn hero-btn-secondary">创作者入驻</NuxtLink>
        </div>

        <div class="hero-stats">
          <div class="hero-stat">
            <div class="hero-stat-value">{{ formatStatNumber(stats.projects) }}+</div>
            <div class="hero-stat-label">资源总数</div>
          </div>
          <div class="hero-stat">
            <div class="hero-stat-value">{{ formatStatNumber(stats.downloads) }}+</div>
            <div class="hero-stat-label">累计下载</div>
          </div>
          <div class="hero-stat">
            <div class="hero-stat-value">{{ formatStatNumber(stats.users) }}</div>
            <div class="hero-stat-label">注册用户</div>
          </div>
        </div>
      </div>

      <div class="hero-visual">
        <div
          class="hero-banner"
          :class="isDragging ? 'cursor-grabbing' : 'cursor-grab'"
          @mouseenter="handleMouseEnter"
          @mouseleave="handleMouseLeave"
          @mousedown="handleDragStart"
          @touchstart="handleDragStart"
          @touchmove="handleDragMove"
          @touchend="handleDragEnd"
        >
          <div
            v-for="(item, index) in heroBanners"
            :key="index"
            :class="[
              'hero-slide',
              {
                'active': index === currentHeroSlide,
              },
            ]"
            @click="handleBannerClick($event, item.slug)"
          >
            <img
              :src="item.image"
              :alt="item.title"
              class="hero-slide-image"
              :class="{ 'scale-effect': !isDragging }"
              draggable="false"
            />
            <div class="hero-slide-overlay">
              <span class="hero-slide-badge">{{ item.badge }}</span>
              <h3 class="hero-slide-title">{{ item.title }}</h3>
              <p class="hero-slide-desc">{{ item.description }}</p>
            </div>
          </div>
          <div class="hero-banner-nav">
            <button
              v-for="(_, index) in heroBanners"
              :key="index"
              :class="['hero-nav-dot', { active: currentHeroSlide === index }]"
              @click.stop="goToHeroSlide(index)"
            ></button>
          </div>
        </div>
      </div>
    </section>

    <!-- Categories -->
    <section class="categories">
      <div class="categories-track">
        <NuxtLink to="/" class="cat-chip active"><StarIcon class="cat-icon" /> 热门推荐</NuxtLink>
        <NuxtLink to="/mods" class="cat-chip"><BoxIcon class="cat-icon" /> 模组</NuxtLink>
        <NuxtLink to="/plugins" class="cat-chip"><PlugIcon class="cat-icon" /> 插件</NuxtLink>
        <NuxtLink to="/modpacks" class="cat-chip"><PackageClosedIcon class="cat-icon" /> 整合包</NuxtLink>
        <NuxtLink to="/shaders" class="cat-chip"><GlassesIcon class="cat-icon" /> 光影</NuxtLink>
        <NuxtLink to="/resourcepacks" class="cat-chip"><PaintBrushIcon class="cat-icon" /> 资源包</NuxtLink>
        <NuxtLink to="/languages" class="cat-chip"><LanguagesIcon class="cat-icon" /> 汉化</NuxtLink>
        <NuxtLink to="/datapacks" class="cat-chip"><BracesIcon class="cat-icon" /> 数据包</NuxtLink>
        <NuxtLink to="/softwares" class="cat-chip"><WrenchIcon class="cat-icon" /> 工具</NuxtLink>
      </div>
    </section>

    <!-- Main Content -->
    <main class="main-content">
      <div class="content-area">
        <!-- Hot Resources -->
        <section class="section">
          <header class="section-header">
            <div class="section-title-group">
              <span class="section-label">TRENDING</span>
              <h2 class="section-title">热门资源</h2>
            </div>
            <NuxtLink to="/mods" class="section-more">浏览全部 →</NuxtLink>
          </header>

          <div class="resource-grid">
            <NuxtLink
              v-for="project in hotProjects"
              :key="project.project_id"
              :to="getProjectLink(project)"
              class="resource-card"
            >
              <!-- Gallery Image -->
              <div class="resource-gallery" :style="getGalleryStyle(project)">
                <div class="resource-type-badge">{{ getProjectTypeLabel(project.project_type) }}</div>
              </div>

              <!-- Card Body -->
              <div class="resource-body">
                <!-- Header: Icon + Title -->
                <div class="resource-header">
                  <img
                    v-if="project.icon_url"
                    :src="project.icon_url"
                    :alt="project.title"
                    class="resource-icon"
                  />
                  <span v-else class="resource-icon-placeholder"><PackageClosedIcon /></span>
                  <div class="resource-title-wrap">
                    <div class="resource-name">{{ project.title }}</div>
                    <div class="resource-author">by {{ project.author }}</div>
                  </div>
                </div>

                <!-- Description -->
                <p class="resource-desc">{{ project.description }}</p>

                <!-- Tags: Loaders + Version -->
                <div class="resource-tags">
                  <span v-for="loader in (project.loaders || []).slice(0, 3)" :key="loader" class="tag loader">
                    {{ formatLoader(loader) }}
                  </span>
                  <span v-if="project.versions?.[0]" class="tag version">{{ project.versions[0] }}</span>
                </div>

                <!-- Footer Stats -->
                <div class="resource-footer">
                  <div class="resource-stat">
                    <DownloadIcon class="stat-icon" />
                    <span>{{ formatNumber(project.downloads) }}</span>
                  </div>
                  <div class="resource-stat">
                    <HeartIcon class="stat-icon" />
                    <span>{{ formatNumber(project.follows) }}</span>
                  </div>
                  <div class="resource-stat update">
                    <UpdatedIcon class="stat-icon" />
                    <span>{{ fromNow(project.date_modified) }}</span>
                  </div>
                </div>
              </div>
            </NuxtLink>
          </div>
        </section>

        <!-- Community News -->
        <section v-if="notices.length > 0" class="section">
          <header class="section-header">
            <div class="section-title-group">
              <span class="section-label">NEWS</span>
              <h2 class="section-title">社区资讯</h2>
            </div>
            <NuxtLink to="/forums/notice" class="section-more">查看全部 →</NuxtLink>
          </header>

          <div class="notice-list">
            <NuxtLink
              v-for="notice in notices"
              :key="notice.id"
              :to="`/d/${notice.id}`"
              class="notice-item"
            >
              <img :src="notice.avatar" :alt="notice.user_name" class="notice-avatar" />
              <div class="notice-content">
                <div class="notice-title">{{ notice.title }}</div>
                <div class="notice-meta">
                  <span class="notice-author">{{ notice.user_name }}</span>
                  <span class="notice-time">{{ fromNow(notice.created) }}</span>
                </div>
              </div>
            </NuxtLink>
          </div>
        </section>

        <!-- Discussions -->
        <section class="section">
          <header class="section-header">
            <div class="section-title-group">
              <span class="section-label">COMMUNITY</span>
              <h2 class="section-title">热门讨论</h2>
            </div>
            <NuxtLink to="/forums/chat" class="section-more">进入论坛 →</NuxtLink>
          </header>

          <div class="discussion-list">
            <NuxtLink
              v-for="forum in forums"
              :key="forum.id"
              :to="forum.project_id ? `/project/${forum.project_id}/forum` : `/d/${forum.id}`"
              class="discussion-item"
            >
              <img :src="forum.avatar" :alt="forum.user_name" class="discussion-avatar" />
              <div class="discussion-content">
                <div class="discussion-header">
                  <span :class="['discussion-category', `cat-${forum.category}`]">
                    {{ getCategoryLabel(forum.category) }}
                  </span>
                  <span class="discussion-time">{{ fromNow(forum.last_post_time) }}</span>
                </div>
                <div class="discussion-title">{{ forum.title }}</div>
                <div class="discussion-meta">
                  <span class="discussion-author">{{ forum.user_name }}</span>
                  <span class="discussion-replies"><MessageIcon class="stat-icon" /> {{ forum.replies }} 回复</span>
                </div>
              </div>
            </NuxtLink>
          </div>
        </section>
      </div>

      <!-- Sidebar -->
      <aside class="sidebar">
        <!-- Version Card -->
        <div class="sidebar-card">
          <div class="sidebar-header">
            <h3 class="sidebar-title"><GameIcon class="sidebar-icon" /> 版本动态</h3>
          </div>
          <div class="sidebar-body">
            <div class="version-list">
              <a href="https://minecraft.net" target="_blank" class="version-item">
                <span class="version-badge">1.21.4</span>
                <div class="version-info">
                  <div class="version-name">最新正式版</div>
                  <div class="version-date">2024年12月</div>
                </div>
              </a>
              <a href="https://minecraft.net" target="_blank" class="version-item">
                <span class="version-badge snapshot">1.21.5</span>
                <div class="version-info">
                  <div class="version-name">快照版本</div>
                  <div class="version-date">测试中</div>
                </div>
              </a>
            </div>
          </div>
        </div>

        <!-- Quick Links -->
        <div class="sidebar-card">
          <div class="sidebar-header">
            <h3 class="sidebar-title"><LinkIcon class="sidebar-icon" /> 快捷入口</h3>
          </div>
          <div class="quick-links">
            <NuxtLink to="/mods" class="quick-link">
              <span class="quick-link-icon"><BoxIcon /></span>
              <span class="quick-link-text">模组</span>
            </NuxtLink>
            <NuxtLink to="/modpacks" class="quick-link">
              <span class="quick-link-icon"><PackageClosedIcon /></span>
              <span class="quick-link-text">整合包</span>
            </NuxtLink>
            <NuxtLink to="/shaders" class="quick-link">
              <span class="quick-link-icon"><GlassesIcon /></span>
              <span class="quick-link-text">光影</span>
            </NuxtLink>
            <NuxtLink to="/forums/chat" class="quick-link">
              <span class="quick-link-icon"><MessageIcon /></span>
              <span class="quick-link-text">论坛</span>
            </NuxtLink>
          </div>
        </div>
      </aside>
    </main>
  </div>
</template>

<script setup>
import dayjs from "dayjs";
import {
  BoxIcon,
  PackageClosedIcon,
  GlobeIcon,
  LanguagesIcon,
  MessageIcon,
  StarIcon,
  PlugIcon,
  GlassesIcon,
  PaintBrushIcon,
  BracesIcon,
  WrenchIcon,
  GameIcon,
  LinkIcon,
  SparklesIcon,
  HeartIcon,
  DownloadIcon,
  UpdatedIcon,
} from "@modrinth/assets";

// Data
const hotProjects = ref([]);
const forums = ref([]);
const notices = ref([]);
const stats = ref({
  projects: 12580,
  downloads: 1580000,
  users: 89234,
  modpacks: 1247,
  mods: 3842,
  languages: 4521,
  forums: 28000,
});

// Hero Banners - 来自整合包页面的内容
const heroBanners = ref([
  {
    image: "https://cdn.bbsmc.net/bbsmc/data/G23dLUsP/images/e681d996cd07316e12facedd8fb22e9f74ce68a1_350.webp",
    title: "剑与王国",
    description: "围绕模拟殖民地与村民招募玩法的深度魔改整合包",
    badge: "热门整合包",
    slug: "/modpack/snk",
  },
  {
    image: "https://cdn.bbsmc.net/bbsmc/data/EIrkPpcm/images/7d43813f0ff22b6c769e7382d36d5059657e8a94_350.webp",
    title: "龙之冒险：新征程",
    description: "面对众多怪物的冒险之旅，你做好准备了吗？",
    badge: "精选整合包",
    slug: "/modpack/lzmx",
  },
  {
    image: "https://cdn.bbsmc.net/raw/images/pcl2.jpg",
    title: "PCL2 启动器",
    description: "超快的下载速度，下载安装 Mod 和整合包，简洁且高度自定义的界面",
    badge: "推荐软件",
    slug: "/software/pcl",
  },
  {
    image: "https://cdn.bbsmc.net/bbsmc/data/XMUypeti/images/82d38f228afad3b75202eaf8a148c1318a8cea48_350.webp",
    title: "愚者 - The Fool",
    description: "愚弄、伪装、欺诈，屠龙者终成恶龙。",
    badge: "精选整合包",
    slug: "/modpack/the-fool",
  },
  {
    image: "https://cdn.bbsmc.net/bbsmc/data/e11vzqXl/images/346fd8930411f592c94acce68b8290a5266843e3_350.webp",
    title: "香草纪元:食旅纪行",
    description: "农夫乐事全附属与异界冒险",
    badge: "热门整合包",
    slug: "/modpack/vefc",
  },
]);

const currentHeroSlide = ref(0);
const autoPlayInterval = ref(null);
const isClient = ref(false);
const isDragging = ref(false);
const dragStartX = ref(0);
const dragCurrentX = ref(0);
const hasDragged = ref(false);

// Fetch data
async function fetchData() {
  try {
    const [projectsResponse, forumsResponse, noticesResponse] = await Promise.all([
      useBaseFetch(`search?limit=6&index=relevance`),
      useBaseFetch(`forum`, { apiVersion: 3 }),
      useBaseFetch(`forum/notice/lists`, { apiVersion: 3 }),
    ]);

    hotProjects.value = projectsResponse.hits ?? [];
    forums.value = (forumsResponse.forums ?? []).slice(0, 5);
    notices.value = (noticesResponse.forums ?? []).slice(0, 5);
  } catch (e) {
    console.error("Failed to fetch data:", e);
  }
}

await fetchData();

// Time formatting
const fromNow = (date) => {
  const currentDate = useCurrentDate();
  return dayjs(date).from(currentDate.value);
};

// Number formatting
const formatNumber = (num) => {
  if (!num) return "0";
  if (num >= 10000) {
    return (num / 10000).toFixed(1).replace(/\.0$/, "") + "万";
  }
  return num.toLocaleString();
};

const formatStatNumber = (num) => {
  if (!num) return "0";
  if (num >= 10000) {
    return (num / 10000).toFixed(0) + "万";
  }
  if (num >= 1000) {
    return (num / 1000).toFixed(1).replace(/\.0$/, "") + "k";
  }
  return num.toLocaleString();
};

// Project link
const getProjectLink = (project) => {
  const typeMap = {
    mod: "mod",
    modpack: "modpack",
    plugin: "plugin",
    resourcepack: "resourcepack",
    shader: "shader",
    datapack: "datapack",
    software: "software",
    language: "language",
  };
  const type = typeMap[project.project_type] || project.project_type;
  return `/${type}/${project.slug || project.project_id}`;
};

// Project type label
const getProjectTypeLabel = (type) => {
  const labels = {
    mod: "模组",
    modpack: "整合包",
    plugin: "插件",
    resourcepack: "资源包",
    shader: "光影",
    datapack: "数据包",
    software: "软件",
    language: "汉化",
  };
  return labels[type] || type;
};

// Get gallery image URL (handles different API response formats)
const getGalleryStyle = (project) => {
  // Try different possible gallery formats
  let imageUrl = null;

  // Format 1: gallery is array of strings
  if (project.gallery?.[0] && typeof project.gallery[0] === 'string') {
    imageUrl = project.gallery[0];
  }
  // Format 2: gallery is array of objects with url property
  else if (project.gallery?.[0]?.url) {
    imageUrl = project.gallery[0].url;
  }
  // Format 3: featured_gallery field
  else if (project.featured_gallery) {
    imageUrl = project.featured_gallery;
  }
  // Format 4: icon_url as fallback for software
  else if (project.project_type === 'software' && project.icon_url) {
    imageUrl = project.icon_url;
  }

  return imageUrl ? `background-image: url(${imageUrl})` : '';
};

// Forum category label
const getCategoryLabel = (category) => {
  const labels = {
    chat: '闲聊',
    project: '资源',
    article: '专栏',
    notice: '公告',
    help: '求助',
    share: '分享',
  };
  return labels[category] || category;
};

// Loader formatting
const formatLoader = (loader) => {
  const loaderNames = {
    fabric: "Fabric",
    forge: "Forge",
    neoforge: "NeoForge",
    quilt: "Quilt",
    bukkit: "Bukkit",
    spigot: "Spigot",
    paper: "Paper",
    purpur: "Purpur",
    sponge: "Sponge",
    bungeecord: "BungeeCord",
    velocity: "Velocity",
    waterfall: "Waterfall",
    folia: "Folia",
    canvas: "Canvas",
    iris: "Iris",
    optifine: "OptiFine",
    vanilla: "原版",
  };
  return loaderNames[loader] || loader;
};

// Hero Banner controls
const startAutoPlay = () => {
  if (!isClient.value) return;
  stopAutoPlay();
  autoPlayInterval.value = setInterval(() => {
    currentHeroSlide.value = (currentHeroSlide.value + 1) % heroBanners.value.length;
  }, 5000);
};

const stopAutoPlay = () => {
  if (autoPlayInterval.value) {
    clearInterval(autoPlayInterval.value);
    autoPlayInterval.value = null;
  }
};

const goToHeroSlide = (index) => {
  if (index === currentHeroSlide.value) {
    navigateTo(heroBanners.value[index].slug);
    return;
  }
  currentHeroSlide.value = index;
  startAutoPlay();
};

const prevHeroSlide = () => {
  currentHeroSlide.value = (currentHeroSlide.value - 1 + heroBanners.value.length) % heroBanners.value.length;
  startAutoPlay();
};

const nextHeroSlide = () => {
  currentHeroSlide.value = (currentHeroSlide.value + 1) % heroBanners.value.length;
  startAutoPlay();
};

// Banner 拖拽处理函数
const handleDragStart = (e) => {
  const isTouchEvent = e.type.includes("touch");
  isDragging.value = true;
  hasDragged.value = false;
  dragStartX.value = isTouchEvent ? e.touches[0].clientX : e.clientX;
  dragCurrentX.value = dragStartX.value;
  stopAutoPlay();

  if (!isTouchEvent) {
    e.preventDefault();
    document.addEventListener("mousemove", handleDragMove);
    document.addEventListener("mouseup", handleDragEnd);
  }
};

const handleDragMove = (e) => {
  if (!isDragging.value) return;
  const isTouchEvent = e.type.includes("touch");
  const currentX = isTouchEvent ? e.touches[0].clientX : e.clientX;
  dragCurrentX.value = currentX;

  const distance = Math.abs(currentX - dragStartX.value);
  if (distance > 5) {
    hasDragged.value = true;
    e.preventDefault();
  }
};

const handleDragEnd = () => {
  if (!isDragging.value) return;

  const dragDistance = dragCurrentX.value - dragStartX.value;
  const threshold = 50;

  if (Math.abs(dragDistance) > threshold) {
    if (dragDistance > 0) {
      prevHeroSlide();
    } else {
      nextHeroSlide();
    }
  }

  isDragging.value = false;
  startAutoPlay();

  document.removeEventListener("mousemove", handleDragMove);
  document.removeEventListener("mouseup", handleDragEnd);

  setTimeout(() => {
    dragStartX.value = 0;
    dragCurrentX.value = 0;
  }, 10);
};

const handleBannerClick = (e, url) => {
  e.preventDefault();
  e.stopPropagation();

  if (hasDragged.value) {
    hasDragged.value = false;
    return;
  }

  navigateTo(url);
};

const handleMouseEnter = () => {
  if (!isClient.value) return;
  stopAutoPlay();
};

const handleMouseLeave = () => {
  if (!isClient.value) return;
  startAutoPlay();
};

onMounted(() => {
  isClient.value = true;
  currentHeroSlide.value = Math.floor(Math.random() * heroBanners.value.length);
  startAutoPlay();
});

onUnmounted(() => {
  stopAutoPlay();
  isClient.value = false;
});
</script>

<style scoped lang="scss">
.home-page {
  min-height: 100vh;
}

// ==========================================
// HERO SECTION
// ==========================================
.hero {
  min-height: 90vh;
  padding: 120px 40px 80px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 60px;
  align-items: center;
  max-width: 1600px;
  margin: 0 auto;
  position: relative;
}

.hero-content {
  position: relative;
  z-index: 1;
}

.hero-title {
  font-family: var(--font-display);
  font-size: clamp(2.5rem, 5vw, 4rem);
  font-weight: 900;
  line-height: 1.1;
  letter-spacing: -0.02em;
  margin-bottom: 28px;
  color: var(--color-text-dark);
  animation: reveal-up 0.8s var(--ease-out) 0.1s both;
}

.hero-title-line {
  display: block;
}

.hero-title-highlight {
  color: var(--flame, #f16436);
  position: relative;
}

.hero-title-highlight::after {
  content: '';
  position: absolute;
  left: 0;
  bottom: 0.05em;
  width: 100%;
  height: 0.12em;
  background: var(--flame, #f16436);
  opacity: 0.3;
  border-radius: 4px;
}

.hero-desc {
  font-size: 1.15rem;
  color: var(--color-secondary);
  line-height: 1.8;
  max-width: 500px;
  margin-bottom: 40px;
  animation: reveal-up 0.8s var(--ease-out) 0.2s both;
}

.hero-actions {
  display: flex;
  gap: 16px;
  margin-bottom: 60px;
  animation: reveal-up 0.8s var(--ease-out) 0.3s both;
}

.hero-btn {
  font-family: var(--font-display);
  font-size: 1rem;
  font-weight: 700;
  padding: 16px 32px;
  border-radius: 16px;
  border: none;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  transition: all 0.4s var(--ease-spring);
  text-decoration: none;
}

.hero-btn-primary {
  background: var(--flame, #f16436);
  color: #000;

  &:hover {
    transform: translateY(-4px) scale(1.02);
    box-shadow: 0 20px 40px var(--accent-glow, rgba(241, 100, 54, 0.4));
  }
}

.hero-btn-secondary {
  background: var(--color-raised-bg);
  border: 2px solid var(--color-divider);
  color: var(--color-text-dark);

  &:hover {
    border-color: var(--color-divider-dark);
    transform: translateY(-4px);
  }
}

.hero-stats {
  display: flex;
  gap: 48px;
  animation: reveal-up 0.8s var(--ease-out) 0.4s both;
}

.hero-stat {
  position: relative;

  &::after {
    content: '';
    position: absolute;
    right: -24px;
    top: 50%;
    transform: translateY(-50%);
    width: 1px;
    height: 40px;
    background: var(--color-divider);
  }

  &:last-child::after {
    display: none;
  }
}

.hero-stat-value {
  font-family: var(--font-display);
  font-size: 2rem;
  font-weight: 900;
  color: var(--color-text-dark);
  letter-spacing: -0.02em;
}

.hero-stat-label {
  font-size: 0.9rem;
  color: var(--color-secondary);
  margin-top: 4px;
}

// Hero Visual - Banner Carousel
.hero-visual {
  position: relative;
  z-index: 1;
  animation: reveal-scale 1s var(--ease-out) 0.3s both;
}

.hero-banner {
  position: relative;
  width: 100%;
  height: 480px;
  border-radius: 24px;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
  user-select: none;

  &.cursor-grab {
    cursor: grab;
  }

  &.cursor-grabbing {
    cursor: grabbing;
  }
}

.hero-slide {
  position: absolute;
  inset: 0;
  opacity: 0;
  transition: opacity 0.6s ease, transform 0.6s ease;
  transform: scale(1.02);

  &.active {
    opacity: 1;
    transform: scale(1);
  }
}

.hero-slide-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  user-select: none;
  pointer-events: none;

  &.scale-effect {
    transition: transform 0.5s ease;

    .hero-banner:hover & {
      transform: scale(1.05);
    }
  }
}

.hero-slide-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 40px 32px;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.85) 0%, rgba(0, 0, 0, 0.4) 60%, transparent 100%);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.hero-slide-badge {
  display: inline-flex;
  align-self: flex-start;
  padding: 6px 16px;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  color: #fff;
  font-size: 0.75rem;
  font-weight: 600;
  border-radius: 6px;
  letter-spacing: 0.03em;
  border: 1px solid rgba(255, 255, 255, 0.2);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.hero-slide-title {
  font-family: var(--font-display);
  font-size: 1.8rem;
  font-weight: 800;
  color: #fff;
  margin: 0;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.hero-slide-desc {
  font-size: 1rem;
  color: rgba(255, 255, 255, 0.85);
  margin: 0;
  max-width: 400px;
  line-height: 1.5;
}

.hero-slide-link {
  position: absolute;
  inset: 0;
  z-index: 1;
}

.hero-banner-nav {
  position: absolute;
  bottom: 20px;
  right: 32px;
  display: flex;
  gap: 8px;
  z-index: 2;
}

.hero-nav-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.4);
  border: none;
  cursor: pointer;
  transition: all 0.3s ease;
  padding: 0;

  &:hover {
    background: rgba(255, 255, 255, 0.7);
  }

  &.active {
    background: var(--flame, #f16436);
    width: 28px;
    border-radius: 5px;
  }
}

.hero-nav-arrow {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.15);
  color: #fff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  opacity: 0;
  z-index: 2;

  svg {
    width: 20px;
    height: 20px;
  }

  &:hover {
    background: var(--flame, #f16436);
    border-color: var(--flame, #f16436);
  }

  &.prev {
    left: 16px;
  }

  &.next {
    right: 16px;
  }
}

// ==========================================
// CATEGORIES
// ==========================================
.categories {
  max-width: 1600px;
  margin: 0 auto 60px;
  padding: 0 40px;
  overflow: hidden;
}

.categories-track {
  display: flex;
  gap: 12px;
  overflow-x: auto;
  padding: 20px 0;
  scrollbar-width: none;
  -webkit-overflow-scrolling: touch;

  &::-webkit-scrollbar {
    display: none;
  }

  @media (min-width: 1400px) {
    justify-content: center;
    flex-wrap: wrap;
  }
}

.cat-chip {
  flex-shrink: 0;
  font-family: var(--font-display);
  font-size: 0.9rem;
  font-weight: 600;
  padding: 14px 24px;
  background: var(--color-raised-bg);
  border: 1px solid var(--color-divider);
  border-radius: 100px;
  color: var(--color-secondary);
  display: flex;
  align-items: center;
  gap: 10px;
  transition: all 0.3s var(--ease-out);
  cursor: pointer;
  text-decoration: none;

  &:hover {
    border-color: var(--flame, #f16436);
    color: var(--color-text-dark);
    transform: translateY(-3px);
  }

  &.active {
    background: var(--flame, #f16436);
    border-color: var(--flame, #f16436);
    color: #000;
  }
}

.cat-icon {
  width: 1.1rem;
  height: 1.1rem;
  flex-shrink: 0;
}

// ==========================================
// MAIN CONTENT
// ==========================================
.main-content {
  max-width: 1600px;
  margin: 0 auto;
  padding: 0 40px 80px;
  display: grid;
  grid-template-columns: 1fr 380px;
  gap: 48px;
}

.content-area {
  min-width: 0;
}

// Section Headers
.section {
  margin-bottom: 48px;
}

.section-header {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--color-divider);
}

.section-title-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.section-label {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--flame, #f16436);
  text-transform: uppercase;
  letter-spacing: 0.15em;
}

.section-title {
  font-family: var(--font-display);
  font-size: 1.5rem;
  font-weight: 800;
  letter-spacing: -0.02em;
  color: var(--color-text-dark);
  margin: 0;
}

.section-more {
  font-family: var(--font-display);
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--flame, #f16436);
  display: flex;
  align-items: center;
  gap: 8px;
  transition: gap 0.3s var(--ease-out);
  text-decoration: none;

  &:hover {
    gap: 14px;
  }
}

// Resource Grid - Card Layout
.resource-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;

  @media (max-width: 1200px) {
    grid-template-columns: repeat(2, 1fr);
  }

  @media (max-width: 768px) {
    grid-template-columns: 1fr;
  }
}

.resource-card {
  background: var(--bg-card, var(--color-raised-bg));
  border: 1px solid var(--color-divider);
  border-radius: 16px;
  overflow: hidden;
  transition: all 0.3s var(--ease-out);
  text-decoration: none;
  display: flex;
  flex-direction: column;

  &:hover {
    border-color: var(--flame, #f16436);
    transform: translateY(-6px);
    box-shadow: 0 20px 40px var(--accent-glow, rgba(241, 100, 54, 0.15));

    .resource-gallery {
      &::after {
        opacity: 0.3;
      }
    }
  }
}

.resource-gallery {
  height: 120px;
  background-color: var(--bg-elevated, #1a1d23);
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  position: relative;

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, var(--color-divider) 0%, var(--bg-elevated, #1a1d23) 100%);
    z-index: 0;
  }

  // Hide fallback gradient when image is loaded
  &[style*="background-image"] {
    &::before {
      display: none;
    }
  }

  &::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.6), transparent);
    opacity: 0.5;
    transition: opacity 0.3s;
    z-index: 1;
  }
}

.resource-type-badge {
  position: absolute;
  top: 10px;
  right: 10px;
  padding: 5px 10px;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  color: #fff;
  font-size: 0.7rem;
  font-weight: 600;
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.15);
  z-index: 2;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.resource-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  flex: 1;
}

.resource-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.resource-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  object-fit: cover;
  flex-shrink: 0;
  border: 1px solid var(--color-divider);
}

.resource-icon-placeholder {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: var(--bg-elevated, #12151a);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  svg {
    width: 1.5rem;
    height: 1.5rem;
    color: var(--color-secondary);
  }
}

.resource-title-wrap {
  min-width: 0;
  flex: 1;
}

.resource-name {
  font-family: var(--font-display);
  font-size: 1rem;
  font-weight: 700;
  color: var(--color-text-dark);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.resource-author {
  font-size: 0.8rem;
  color: var(--color-secondary);
}

.resource-desc {
  font-size: 0.85rem;
  color: var(--color-secondary);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  margin: 0;
  flex: 1;
}

.resource-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.tag {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  font-weight: 600;
  padding: 4px 8px;
  background: var(--bg-elevated, #12151a);
  border-radius: 4px;
  color: var(--color-secondary);
  letter-spacing: 0.02em;

  &.loader {
    background: var(--accent-muted, rgba(241, 100, 54, 0.1));
    color: var(--flame, #f16436);
  }

  &.version {
    background: rgba(45, 212, 191, 0.1);
    color: var(--teal, #2dd4bf);
  }
}

.resource-footer {
  display: flex;
  align-items: center;
  gap: 16px;
  padding-top: 12px;
  border-top: 1px solid var(--color-divider);
  margin-top: auto;
}

.resource-stat {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 0.8rem;
  color: var(--color-secondary);

  .stat-icon {
    width: 0.9rem;
    height: 0.9rem;
    color: var(--color-secondary);
  }

  &.update {
    margin-left: auto;
    font-size: 0.75rem;
  }
}

// Notice List
.notice-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.notice-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 18px 20px;
  background: var(--bg-card, var(--color-raised-bg));
  border: 1px solid var(--color-divider);
  border-radius: 16px;
  transition: all 0.3s var(--ease-out);
  text-decoration: none;
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 4px;
    background: linear-gradient(180deg, var(--flame, #f16436) 0%, #ff8a5c 100%);
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  &:hover {
    border-color: var(--flame, #f16436);
    background: var(--accent-muted, rgba(241, 100, 54, 0.06));

    &::before {
      opacity: 1;
    }

    .notice-badge {
      background: var(--flame, #f16436);
      color: #fff;
    }
  }
}

.notice-avatar {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  object-fit: cover;
  flex-shrink: 0;
  transition: transform 0.3s ease;

  .notice-item:hover & {
    transform: scale(1.05);
  }
}

.notice-content {
  flex: 1;
  min-width: 0;
}

.notice-title {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--color-text-dark);
  margin-bottom: 6px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.notice-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 0.8rem;
  color: var(--color-secondary);
}

.notice-author {
  font-weight: 500;
}

.notice-time {
  opacity: 0.8;
}

// Discussion List
.discussion-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.discussion-item {
  display: flex;
  gap: 16px;
  padding: 20px;
  background: var(--bg-card, var(--color-raised-bg));
  border: 1px solid var(--color-divider);
  border-radius: 16px;
  transition: all 0.3s var(--ease-out);
  text-decoration: none;

  &:hover {
    border-color: var(--color-divider-dark);
    background: var(--bg-card-hover, var(--color-raised-bg));
    transform: translateX(4px);

    .discussion-avatar {
      transform: scale(1.05);
    }
  }
}

.discussion-avatar {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  object-fit: cover;
  flex-shrink: 0;
  transition: transform 0.3s ease;
}

.discussion-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.discussion-header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.discussion-category {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  font-weight: 600;
  padding: 4px 10px;
  border-radius: 6px;
  text-transform: uppercase;
  letter-spacing: 0.03em;

  // Default style
  background: var(--accent-muted, rgba(241, 100, 54, 0.1));
  color: var(--flame, #f16436);

  // Category specific colors
  &.cat-project {
    background: rgba(45, 212, 191, 0.12);
    color: #2dd4bf;
  }

  &.cat-chat {
    background: rgba(139, 92, 246, 0.12);
    color: #8b5cf6;
  }

  &.cat-article {
    background: rgba(59, 130, 246, 0.12);
    color: #3b82f6;
  }

  &.cat-notice {
    background: var(--accent-muted, rgba(241, 100, 54, 0.1));
    color: var(--flame, #f16436);
  }

  &.cat-help {
    background: rgba(255, 179, 71, 0.12);
    color: #ffb347;
  }

  &.cat-share {
    background: rgba(34, 197, 94, 0.12);
    color: #22c55e;
  }
}

.discussion-time {
  font-size: 0.75rem;
  color: var(--color-secondary);
  opacity: 0.7;
}

.discussion-title {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--color-text-dark);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  line-height: 1.4;
}

.discussion-meta {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 0.8rem;
  color: var(--color-secondary);
}

.discussion-author {
  font-weight: 500;
}

.discussion-replies {
  display: flex;
  align-items: center;
  gap: 5px;
  opacity: 0.8;
}

.stat-icon {
  width: 0.9rem;
  height: 0.9rem;
  color: var(--color-secondary);
}

// ==========================================
// SIDEBAR
// ==========================================
.sidebar {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.sidebar-card {
  background: var(--bg-card, var(--color-raised-bg));
  border: 1px solid var(--color-divider);
  border-radius: 20px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
}

.sidebar-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider);
}

.sidebar-title {
  font-family: var(--font-display);
  font-size: 1rem;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--color-text-dark);
  margin: 0;
}

.sidebar-icon {
  width: 1.2rem;
  height: 1.2rem;
  color: var(--flame, #f16436);
}

.sidebar-body {
  padding: 20px 24px;
}

// Version List
.version-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.version-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 16px;
  background: var(--bg-elevated, #12151a);
  border-radius: 12px;
  transition: all 0.3s;
  text-decoration: none;

  &:hover {
    background: var(--bg-card-hover, #1e2229);
  }
}

.version-badge {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  font-weight: 600;
  padding: 8px 14px;
  background: var(--flame, #f16436);
  color: #000;
  border-radius: 10px;

  &.snapshot {
    background: var(--teal, #2dd4bf);
  }
}

.version-info {
  flex: 1;
}

.version-name {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--color-text-dark);
}

.version-date {
  font-size: 0.8rem;
  color: var(--color-secondary);
}

// Quick Links
.quick-links {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  padding: 20px;
}

.quick-link {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 20px 16px;
  background: var(--bg-elevated, #12151a);
  border-radius: 12px;
  transition: all 0.3s var(--ease-out);
  text-decoration: none;

  &:hover {
    background: var(--accent-muted, rgba(241, 100, 54, 0.12));
    transform: translateY(-4px);
  }
}

.quick-link-icon {
  font-size: 1.6rem;
  display: flex;
  align-items: center;
  justify-content: center;

  svg {
    width: 1.6rem;
    height: 1.6rem;
    color: var(--flame, #f16436);
  }
}

.quick-link-text {
  font-family: var(--font-display);
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--color-secondary);
}

// ==========================================
// ANIMATIONS
// ==========================================
@keyframes reveal-up {
  from {
    opacity: 0;
    transform: translateY(40px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes reveal-scale {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes glow {
  0%, 100% {
    filter: drop-shadow(0 0 10px var(--accent-glow, rgba(241, 100, 54, 0.4)));
  }
  50% {
    filter: drop-shadow(0 0 20px var(--accent-glow, rgba(241, 100, 54, 0.4)));
  }
}

// ==========================================
// RESPONSIVE
// ==========================================
@media (max-width: 1200px) {
  .hero {
    grid-template-columns: 1fr;
    padding: 120px 32px 60px;
    min-height: auto;
  }

  .hero-visual {
    max-width: 100%;
  }

  .hero-banner {
    height: 400px;
  }

  .main-content {
    grid-template-columns: 1fr;
    padding: 0 32px 60px;
  }

  .sidebar {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
  }
}

@media (max-width: 768px) {
  .hero {
    padding: 100px 24px 40px;
  }

  .hero-title {
    font-size: clamp(2rem, 8vw, 2.5rem);
  }

  .hero-banner {
    height: 320px;
  }

  .hero-slide-title {
    font-size: 1.4rem;
  }

  .hero-slide-desc {
    font-size: 0.9rem;
    display: none;
  }

  .hero-slide-overlay {
    padding: 24px 20px;
  }

  .hero-nav-arrow {
    width: 40px;
    height: 40px;
    opacity: 1;
  }

  .hero-stats {
    flex-wrap: wrap;
    gap: 24px;
  }

  .hero-stat::after {
    display: none;
  }

  .categories {
    padding: 0 24px;
  }

  .main-content {
    padding: 0 24px 40px;
  }

  .resource-item {
    grid-template-columns: 56px 1fr;
  }

  .resource-icon-wrap {
    width: 56px;
    height: 56px;
  }

  .resource-stats {
    display: none;
  }

  .sidebar {
    grid-template-columns: 1fr;
  }
}
</style>
